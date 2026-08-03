<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import type { CourseDetailView } from '../api/types';

  let { courseId, onClose }: { courseId: string; onClose: () => void } = $props();

  let detail = $state<CourseDetailView | null>(null);

  onMount(async () => {
    detail = await api.getCourseDetail(courseId);
  });

  async function move(index: number, dir: -1 | 1) {
    if (!detail) return;
    const ids = detail.all_lessons.map((l) => l.id);
    const target = index + dir;
    if (target < 0 || target >= ids.length) return;
    [ids[index], ids[target]] = [ids[target], ids[index]];
    detail = await api.reorderLessons(courseId, ids);
  }

  const statusLabel: Record<string, string> = {
    planned: 'planlı',
    done: 'bitti',
    skipped: 'atlandı',
  };
</script>

<div class="cd-overlay">
  <div class="cd-panel">
    {#if !detail}
      <p class="empty-state">&hellip;</p>
    {:else}
      <div class="cd-header">
        <span class="pillar-dot" style={`background:${detail.course.color_token}`}></span>
        <h2>{detail.course.name}</h2>
        <span class="action-meta"
          >{detail.course.done_lessons}/{detail.course.total_lessons} konu</span
        >
        <button class="focus-btn" onclick={onClose}>Kapat</button>
      </div>

      {#if detail.upcoming.length > 0}
        <div class="cd-section">
          <h3>Yaklaşan tekrarlar</h3>
          {#each detail.upcoming as lesson (lesson.id)}
            <div class="action-row">
              <span class="action-name">{lesson.title}</span>
              <span class="action-meta">{lesson.planned_on}</span>
              {#if lesson.review_of}<span class="action-meta">tekrar</span>{/if}
            </div>
          {/each}
        </div>
      {/if}

      <div class="cd-section">
        <h3>Tüm konular</h3>
        {#if detail.all_lessons.length === 0}
          <p class="empty-state">Henüz konu yok.</p>
        {:else}
          {#each detail.all_lessons as lesson, i (lesson.id)}
            <div class="action-row">
              <div class="cd-reorder">
                <button class="cd-arrow" onclick={() => move(i, -1)} disabled={i === 0}>&uarr;</button>
                <button
                  class="cd-arrow"
                  onclick={() => move(i, 1)}
                  disabled={i === detail.all_lessons.length - 1}>&darr;</button
                >
              </div>
              <span class="action-name">{lesson.title}</span>
              <span class="action-meta">{statusLabel[lesson.status]}</span>
              <span class="action-meta">{lesson.planned_on}</span>
              {#if lesson.review_of}<span class="action-meta">tekrar</span>{/if}
            </div>
          {/each}
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .cd-overlay {
    position: fixed;
    inset: 0;
    background: rgba(6, 7, 10, 0.75);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 60;
    padding: 24px;
  }
  .cd-panel {
    background: var(--card);
    border: 1px solid var(--card-border);
    border-radius: var(--radius);
    padding: 20px 24px;
    width: min(680px, 100%);
    max-height: 80vh;
    overflow-y: auto;
  }
  .cd-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 12px;
  }
  .cd-header h2 {
    font-size: 15px;
    margin: 0;
    flex: 1;
  }
  .cd-section {
    margin-top: 16px;
  }
  .cd-section h3 {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-muted);
    margin: 0 0 8px;
  }
  .cd-reorder {
    display: flex;
    flex-direction: column;
    gap: 0;
  }
  .cd-arrow {
    background: none;
    border: none;
    color: var(--ink-muted);
    cursor: pointer;
    font-size: 11px;
    line-height: 1;
    padding: 1px 4px;
  }
  .cd-arrow:disabled {
    opacity: 0.25;
    cursor: default;
  }
</style>
