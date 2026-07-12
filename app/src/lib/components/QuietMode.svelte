<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { renderPainting, conditionFromWmoCode, type WeatherName } from '../painting/engine';
  import { maximForDay } from '../painting/quotes';
  import { api } from '../api/commands';
  import { today } from '../stores/today.svelte';
  import type { QuietModeView, WeatherSnapshot } from '../api/types';

  let canvasEl: HTMLCanvasElement;
  let view = $state<QuietModeView | null>(null);
  let realWeather = $state<WeatherSnapshot | null>(null);
  let now = $state(new Date());

  const realCondition = $derived(
    realWeather ? conditionFromWmoCode(realWeather.weather_code) : null,
  );
  const conditionLabels: Record<string, string> = {
    clear: 'Clear',
    partly_cloudy: 'Partly cloudy',
    overcast: 'Overcast',
    fog: 'Fog',
    rain: 'Rain',
    storm: 'Storm',
    snow: 'Snow',
  };

  const hour = $derived(now.getHours() + now.getMinutes() / 60);
  const dayOfYear = $derived(
    Math.floor(
      (Date.UTC(now.getFullYear(), now.getMonth(), now.getDate()) -
        Date.UTC(now.getFullYear(), 0, 0)) /
        86400000,
    ),
  );
  const maxim = $derived(maximForDay(dayOfYear));

  const weatherLabels: Record<WeatherName, string> = {
    clear: 'Clear',
    radiant: 'Radiant',
    heavy: 'Heavy',
    stormy: 'Stormy',
  };

  function draw() {
    if (!canvasEl || !view) return;
    const ctx = canvasEl.getContext('2d');
    if (!ctx) return;
    const dpr = window.devicePixelRatio || 1;
    const W = canvasEl.clientWidth;
    const H = canvasEl.clientHeight;
    if (canvasEl.width !== W * dpr) {
      canvasEl.width = W * dpr;
      canvasEl.height = H * dpr;
    }
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    const filter = renderPainting(
      ctx,
      W,
      H,
      hour,
      view.inner_weather,
      realWeather && realCondition
        ? {
            condition: realCondition,
            windKmh: realWeather.wind_speed_kmh,
            precipitationMm: realWeather.precipitation_mm,
          }
        : undefined,
      view.kcal_budget > 0
        ? { ratio: view.kcal_today / view.kcal_budget, overBudget: view.kcal_today > view.kcal_budget }
        : undefined,
      view.course_bands.map((b) => ({ ratio: b.ratio })),
      sleepHaze(),
    );
    canvasEl.style.filter = filter;
  }

  // No log yet means no judgment — a clear night by default, not a hazy one.
  function sleepHaze(): number {
    if (!view || view.sleep_hours == null || view.sleep_quality == null) return 0;
    const score = (view.sleep_quality / 5) * 0.5 + Math.min(view.sleep_hours / 8, 1) * 0.5;
    return Math.max(0, Math.min((1 - score) * 0.5, 0.5));
  }

  async function refresh() {
    view = await api.getQuietMode();
    draw();
  }

  async function refreshWeather() {
    realWeather = await api.getWeather();
    draw();
  }

  async function setWeather(w: WeatherName) {
    await api.setInnerWeather(w);
    await refresh();
  }

  $effect(() => {
    // hour/view/realWeather changed — repaint
    hour;
    view;
    realWeather;
    draw();
  });

  onMount(() => {
    refresh();
    refreshWeather();
    const clockId = setInterval(() => (now = new Date()), 1000);
    const repaintId = setInterval(draw, 60000);
    const weatherId = setInterval(refreshWeather, 20 * 60 * 1000);
    const onResize = () => draw();
    window.addEventListener('resize', onResize);
    const unlistenPromise = listen('entry-logged', () => refresh());
    return () => {
      clearInterval(clockId);
      clearInterval(repaintId);
      clearInterval(weatherId);
      window.removeEventListener('resize', onResize);
      unlistenPromise.then((un) => un());
    };
  });

  function fmtClock(d: Date) {
    const pad = (n: number) => String(n).padStart(2, '0');
    return { hm: `${pad(d.getHours())}:${pad(d.getMinutes())}`, s: pad(d.getSeconds()) };
  }

  function fmtDate(d: Date) {
    return d
      .toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' })
      .replace(',', ' ·');
  }

  function fmtFocus(minutes: number) {
    const h = Math.floor(minutes / 60);
    const m = minutes % 60;
    return h > 0 ? `${h}h ${m}m` : `${m}m`;
  }
</script>

<div class="qm-frame">
  <canvas bind:this={canvasEl} class="qm-canvas"></canvas>
  <div class="qm-grain"></div>
  <div class="qm-scrim"></div>

  <div class="qm-ui">
    <div class="qm-top">
      <div class="qm-wordmark">Via <b>&middot;</b> Ethos</div>
      {#if view}
        <div class="qm-daysum">
          <b>{view.honored_today} of {view.due_today}</b> honored<br />
          <b>{fmtFocus(view.focus_minutes_today)}</b> in focus
          {#if view.sleep_hours != null}
            <br /><b>{view.sleep_hours.toFixed(1)}h</b> slept
          {/if}
        </div>
      {/if}
    </div>

    <div class="qm-sky">
      <div class="qm-clock">{fmtClock(now).hm}<span class="qm-sec">:{fmtClock(now).s}</span></div>
      <div class="qm-date">{fmtDate(now)}</div>
      {#if realWeather && realCondition}
        <div class="qm-weather-real">
          {Math.round(realWeather.temperature_c)}&deg; &middot; {conditionLabels[realCondition]}
          {#if realWeather.stale}<span class="qm-stale"> &middot; last known</span>{/if}
        </div>
      {/if}
      {#if view}
        <div class="qm-chips">
          <div class="qm-chip-group">
            {#each Object.entries(weatherLabels) as [w, label]}
              <button
                class="qm-mood-btn"
                class:active={view.inner_weather === w}
                onclick={() => setWeather(w as WeatherName)}
              >
                {label}
              </button>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <div class="qm-maxim">
      <blockquote>&ldquo;{maxim.text}&rdquo;</blockquote>
      <cite>{maxim.cite}</cite>
    </div>

    <footer class="qm-footer">
      {#if view}
        <div class="qm-path">
          <span class="qm-path-label">The Path &middot; 14 days</span>
          {#each view.path as day, i}
            {#if day.pillar_colors.length === 0}
              <span class="qm-stone qm-gap"></span>
            {:else}
              <span
                class="qm-stone"
                class:qm-today={i === view.path.length - 1}
                style={day.pillar_colors.length > 1
                  ? `background: linear-gradient(135deg, ${day.pillar_colors[0]} 0 50%, ${day.pillar_colors[1]} 50% 100%)`
                  : `background:${day.pillar_colors[0]}`}
              ></span>
            {/if}
          {/each}
        </div>
        {#if view.milestones.length > 0}
          <div class="qm-milestones">
            <span class="qm-path-label">Milestones</span>
            {#each view.milestones as m}
              <span
                class="qm-milestone"
                class:qm-overdue={m.overdue}
                style={`--c:${m.color_token}`}
                title={m.title}
              ></span>
            {/each}
          </div>
        {/if}
        {#if view.course_bands.length > 0}
          <div class="qm-milestones">
            <span class="qm-path-label">Field rows</span>
            {#each view.course_bands as band}
              <span class="qm-course-chip" style={`--c:${band.color_token}`}
                >{band.name} {Math.round(band.ratio * 100)}%</span
              >
            {/each}
          </div>
        {/if}
        <div class="qm-stats">
          <div class="qm-stat">
            <div class="qm-v">{view.total_points.toLocaleString()}</div>
            <div class="qm-k">Ethos Points</div>
          </div>
          <div class="qm-stat">
            <div class="qm-v">{view.best_streak}d</div>
            <div class="qm-k">Best Streak</div>
          </div>
        </div>
      {/if}
      <div class="qm-legend">
        {#if today.view}
          {#each today.view.pillars as pillar (pillar.id)}
            <span class="qm-pillar" style={`--c:${pillar.color_token}`}
              ><i></i>{pillar.name}</span
            >
          {/each}
        {/if}
      </div>
    </footer>
  </div>
</div>

<style>
  .qm-frame {
    position: relative;
    width: 100%;
    height: 100%;
    min-height: 560px;
    border-radius: 12px;
    overflow: hidden;
    background: #0a0814;
  }

  .qm-canvas {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    transition: filter 1.2s ease;
  }

  .qm-scrim {
    position: absolute;
    inset: 0;
    pointer-events: none;
    background:
      linear-gradient(180deg, rgba(8, 6, 18, 0.35) 0%, transparent 22%),
      linear-gradient(0deg, rgba(8, 6, 18, 0.72) 0%, rgba(8, 6, 18, 0.35) 26%, transparent 48%);
  }

  .qm-grain {
    position: absolute;
    inset: 0;
    pointer-events: none;
    mix-blend-mode: overlay;
    opacity: 0.5;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='180' height='180'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='2' stitchTiles='stitch'/%3E%3CfeColorMatrix type='saturate' values='0'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
  }

  .qm-ui {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    z-index: 2;
    color: #f4f0e6;
    font-family: 'Avenir Next', 'Segoe UI', system-ui, sans-serif;
  }

  .qm-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 26px 36px 0;
  }

  .qm-wordmark {
    font-family: 'Iowan Old Style', 'Palatino Linotype', Georgia, serif;
    font-size: 14px;
    letter-spacing: 0.4em;
    text-transform: uppercase;
  }
  .qm-wordmark b {
    color: var(--craft);
    font-weight: 500;
  }

  .qm-daysum {
    text-align: right;
    font-size: 12.5px;
    color: #c3bfce;
    font-variant-numeric: tabular-nums;
    line-height: 1.7;
    text-shadow: 0 1px 10px rgba(0, 0, 0, 0.7);
  }
  .qm-daysum b {
    color: #f4f0e6;
    font-weight: 600;
  }

  .qm-sky {
    text-align: center;
    margin-top: 2px;
    text-shadow: 0 2px 30px rgba(0, 0, 0, 0.55);
  }

  .qm-clock {
    font-size: clamp(52px, 7vw, 88px);
    font-weight: 200;
    line-height: 1;
    font-variant-numeric: tabular-nums;
  }
  .qm-sec {
    font-size: 0.3em;
    color: #c3bfce;
    font-weight: 300;
  }

  .qm-date {
    margin-top: 6px;
    font-size: 13px;
    color: #c3bfce;
    letter-spacing: 0.14em;
    text-transform: uppercase;
  }

  .qm-weather-real {
    margin-top: 10px;
    font-size: 12.5px;
    letter-spacing: 0.04em;
    color: #c3bfce;
  }
  .qm-stale {
    color: #8d88a0;
    font-style: italic;
  }

  .qm-chips {
    display: flex;
    justify-content: center;
    margin-top: 14px;
  }

  .qm-chip-group {
    display: flex;
    gap: 6px;
    padding: 4px;
    border: 1px solid rgba(244, 240, 230, 0.16);
    border-radius: 999px;
    background: rgba(8, 6, 18, 0.38);
    backdrop-filter: blur(6px);
  }

  .qm-mood-btn {
    font-family: inherit;
    font-size: 11px;
    letter-spacing: 0.08em;
    padding: 6px 13px;
    border-radius: 999px;
    border: none;
    background: transparent;
    color: #c3bfce;
    cursor: pointer;
  }
  .qm-mood-btn.active {
    background: rgba(244, 240, 230, 0.16);
    color: #f4f0e6;
    font-weight: 600;
  }

  .qm-maxim {
    text-align: center;
    padding: 0 40px;
    margin-top: auto;
  }
  .qm-maxim blockquote {
    margin: 0 auto;
    font-family: 'Iowan Old Style', 'Palatino Linotype', Georgia, serif;
    font-style: italic;
    font-size: clamp(14px, 1.6vw, 19px);
    line-height: 1.6;
    max-width: 42em;
    text-shadow: 0 1px 18px rgba(0, 0, 0, 0.75);
  }
  .qm-maxim cite {
    display: block;
    margin-top: 8px;
    font-style: normal;
    font-size: 11px;
    letter-spacing: 0.16em;
    text-transform: uppercase;
    color: #c3bfce;
  }

  .qm-footer {
    display: flex;
    align-items: center;
    gap: 22px;
    padding: 14px 36px 20px;
    flex-wrap: wrap;
  }

  .qm-path {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .qm-path-label {
    font-size: 10px;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    color: #c3bfce;
    margin-right: 6px;
    text-shadow: 0 1px 8px rgba(0, 0, 0, 0.8);
  }
  .qm-stone {
    width: 13px;
    height: 13px;
    border-radius: 50%;
  }
  .qm-stone.qm-gap {
    background: transparent;
    border: 1px dashed rgba(244, 240, 230, 0.16);
    width: 9px;
    height: 9px;
  }
  .qm-stone.qm-today {
    outline: 1.5px solid #c3bfce;
    outline-offset: 2px;
  }

  .qm-milestones {
    display: flex;
    align-items: center;
    gap: 5px;
  }
  .qm-milestone {
    width: 9px;
    height: 9px;
    background: var(--c);
    transform: rotate(45deg);
    box-shadow: 0 0 6px var(--c);
  }
  .qm-milestone.qm-overdue {
    outline: 1.5px dashed #f4f0e6;
    outline-offset: 2px;
  }

  .qm-course-chip {
    font-size: 10.5px;
    color: #c3bfce;
    border-left: 2px solid var(--c);
    padding-left: 6px;
  }

  .qm-stats {
    display: flex;
    gap: 22px;
    text-shadow: 0 1px 10px rgba(0, 0, 0, 0.8);
  }
  .qm-stat {
    text-align: right;
  }
  .qm-v {
    font-size: 17px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }
  .qm-k {
    font-size: 9.5px;
    color: #c3bfce;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    margin-top: 2px;
  }

  .qm-legend {
    display: flex;
    gap: 13px;
    margin-left: auto;
    padding-left: 20px;
    border-left: 1px solid rgba(244, 240, 230, 0.16);
  }
  .qm-pillar {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: #c3bfce;
    text-shadow: 0 1px 8px rgba(0, 0, 0, 0.8);
  }
  .qm-pillar i {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--c);
    box-shadow: 0 0 8px var(--c);
  }
</style>
