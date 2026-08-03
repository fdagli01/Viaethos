<script lang="ts">
  import type { ActionView } from '../api/types';
  import { api } from '../api/commands';

  let {
    action,
    onClose,
    lessonId = null,
    lessonTitle = null,
  }: {
    action: ActionView;
    onClose: () => void;
    lessonId?: string | null;
    lessonTitle?: string | null;
  } = $props();

  type Phase = 'intend' | 'active' | 'seal';

  let phase = $state<Phase>(action.active_session ? 'active' : 'intend');
  let intention = $state(action.active_session?.intention ?? lessonTitle ?? '');
  let minutes = $state(action.active_session?.planned_minutes ?? action.default_minutes ?? 25);
  let entryId = $state(action.active_session?.entry_id ?? '');
  let startedAt = $state(action.active_session?.started_at ?? 0);
  let reflection = $state('');
  let nowSec = $state(Math.floor(Date.now() / 1000));

  $effect(() => {
    if (phase !== 'active') return;
    const id = setInterval(() => {
      nowSec = Math.floor(Date.now() / 1000);
    }, 1000);
    return () => clearInterval(id);
  });

  const elapsed = $derived(Math.max(0, nowSec - startedAt));
  const plannedSeconds = $derived(minutes * 60);
  const remaining = $derived(plannedSeconds - elapsed);
  const overtime = $derived(remaining < 0);
  const displaySeconds = $derived(Math.abs(overtime ? remaining : remaining));

  function fmt(totalSeconds: number) {
    const m = Math.floor(totalSeconds / 60);
    const s = totalSeconds % 60;
    return `${m}:${s.toString().padStart(2, '0')}`;
  }

  async function begin() {
    const view = await api.startFocus(action.id, intention.trim() || null, minutes, lessonId);
    const pillar = view.pillars.find((p) => p.id === action.pillar_id);
    const updated = pillar?.actions.find((a) => a.id === action.id);
    if (updated?.active_session) {
      entryId = updated.active_session.entry_id;
      startedAt = updated.active_session.started_at;
    }
    phase = 'active';
  }

  function toSeal() {
    phase = 'seal';
  }

  async function complete() {
    await api.endFocus(entryId, 'completed', reflection.trim() || null);
    onClose();
  }

  async function abandon() {
    await api.endFocus(entryId, 'abandoned', null);
    onClose();
  }
</script>

<div class="ritual-overlay">
  <div class="ritual-card">
    {#if phase === 'intend'}
      <div class="ritual-ring">
        <div class="time">{minutes}<span style="font-size:16px">m</span></div>
        <div class="label">{action.name}</div>
      </div>
      <textarea
        class="ritual-input"
        rows="2"
        placeholder="Bu seans ne için? (isteğe bağlı)"
        bind:value={intention}
      ></textarea>
      <div class="ritual-actions">
        <button class="btn ghost" onclick={onClose}>Vazgeç</button>
        <input
          type="number"
          class="ritual-input"
          style="width:72px;text-align:center;margin:0"
          min="5"
          max="180"
          bind:value={minutes}
        />
        <button class="btn primary" onclick={begin}>Başla</button>
      </div>
    {:else if phase === 'active'}
      <div class="ritual-ring" style={overtime ? 'border-color:var(--body)' : ''}>
        <div class="time">{overtime ? '+' : ''}{fmt(displaySeconds)}</div>
        <div class="label">{overtime ? 'ek süre' : 'kalan'}</div>
      </div>
      {#if intention}
        <div class="ritual-intention">&ldquo;{intention}&rdquo;</div>
      {/if}
      <div class="ritual-actions">
        <button class="btn ghost" onclick={abandon}>Bırak</button>
        <button class="btn primary" onclick={toSeal}>Bitir</button>
      </div>
    {:else}
      <div class="ritual-ring">
        <div class="time" style="font-size:24px">{fmt(elapsed)}</div>
        <div class="label">harcanan</div>
      </div>
      <textarea
        class="ritual-input"
        rows="2"
        placeholder="Kapanış notu (isteğe bağlı)"
        bind:value={reflection}
      ></textarea>
      <div class="ritual-actions">
        <button class="btn ghost" onclick={onClose}>Sonra</button>
        <button class="btn primary" onclick={complete}>Mühürle</button>
      </div>
    {/if}
  </div>
</div>
