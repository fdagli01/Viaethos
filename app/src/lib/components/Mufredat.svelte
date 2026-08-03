<script lang="ts">
  import { onMount } from 'svelte';
  import { curriculum } from '../stores/curriculum.svelte';
  import { today } from '../stores/today.svelte';
  import { ritual } from '../stores/ritual.svelte';
  import CourseDetail from './CourseDetail.svelte';
  import type { CourseView } from '../api/types';

  let adding = $state(false);
  let courseName = $state('');
  let coursePillarId = $state('');
  let detailCourseId = $state<string | null>(null);

  onMount(() => {
    curriculum.init();
  });

  async function submitCourse(e: Event) {
    e.preventDefault();
    const name = courseName.trim();
    if (!name || !coursePillarId) return;
    // Three hours a week is the house default — a number nobody wants to be
    // asked for every time they add a course.
    await curriculum.addCourse(name, coursePillarId, 3);
    courseName = '';
  }

  const lessonDrafts = $state<Record<string, string>>({});

  async function submitLesson(courseId: string, e: Event) {
    e.preventDefault();
    const title = (lessonDrafts[courseId] ?? '').trim();
    if (!title) return;
    const todayStr = new Date().toISOString().slice(0, 10);
    await curriculum.addLesson(courseId, title, todayStr);
    lessonDrafts[courseId] = '';
  }

  function studyAction(course: CourseView) {
    if (!today.view) return null;
    const pillar = today.view.pillars.find((p) => p.id === course.pillar_id);
    return pillar?.actions.find((a) => a.id === course.action_id) ?? null;
  }

  function study(course: CourseView, lessonId: string, lessonTitle: string) {
    const action = studyAction(course);
    if (!action) return;
    ritual.open(action, lessonId, lessonTitle);
  }
</script>

<section class="pillar-section">
  <div class="pillar-header">
    <h2>M&uuml;fredat</h2>
    <span class="action-meta" style="margin-left:auto">{curriculum.courses.length} ders</span>
    <button
      class="add-toggle"
      class:open={adding}
      style="margin-left:0"
      onclick={() => (adding = !adding)}
      aria-label="Ders ekle">+</button
    >
  </div>

  {#if adding}
    <form class="add-form" onsubmit={submitCourse}>
      <input class="task-input" placeholder="Yeni ders…" bind:value={courseName} />
      {#if today.view}
        <select class="task-select" bind:value={coursePillarId} aria-label="Sütun">
          <option value="">Sütun…</option>
          {#each today.view.pillars as pillar (pillar.id)}
            <option value={pillar.id}>{pillar.name}</option>
          {/each}
        </select>
      {/if}
      <button class="btn primary" type="submit">Ekle</button>
    </form>
  {/if}

  {#if curriculum.courses.length === 0}
    <p class="empty-state">Henüz ders yok.</p>
  {:else}
    {#each curriculum.courses as course (course.id)}
      {@const pct =
        course.total_lessons === 0
          ? 0
          : Math.round((course.done_lessons / course.total_lessons) * 100)}
      <div class="mufredat-course">
        <div class="mufredat-course-header">
          <span class="pillar-dot" style={`background:${course.color_token}`}></span>
          <button class="mufredat-course-link" onclick={() => (detailCourseId = course.id)}
            >{course.name}</button
          >
          <span class="action-meta">{course.done_lessons}/{course.total_lessons}</span>
        </div>
        <div class="pillar-hairline">
          <div style={`width:${pct}%;background:${course.color_token}`}></div>
        </div>

        {#if course.due_lessons.length === 0}
          <p class="empty-state">Bugün için konu yok.</p>
        {:else}
          {#each course.due_lessons as lesson (lesson.id)}
            <div class="action-row">
              <span class="action-name">{lesson.title}</span>
              {#if lesson.review_of}<span class="action-meta">tekrar</span>{/if}
              <button class="focus-btn" onclick={() => study(course, lesson.id, lesson.title)}
                >Çalış</button
              >
              <button class="focus-btn" onclick={() => curriculum.completeLesson(lesson.id)}
                >Bitti</button
              >
              <button class="focus-btn" onclick={() => curriculum.skipLesson(lesson.id)}>Atla</button
              >
            </div>
          {/each}
        {/if}

        <form class="mufredat-topic" onsubmit={(e) => submitLesson(course.id, e)}>
          <input
            class="task-input"
            placeholder="Konu ekle…"
            value={lessonDrafts[course.id] ?? ''}
            oninput={(e) => (lessonDrafts[course.id] = (e.target as HTMLInputElement).value)}
          />
          <button class="btn" type="submit">Ekle</button>
        </form>
      </div>
    {/each}
  {/if}
</section>

{#if detailCourseId}
  <CourseDetail courseId={detailCourseId} onClose={() => (detailCourseId = null)} />
{/if}

<style>
  .mufredat-course {
    border-bottom: 1px solid var(--card-border);
    padding: 12px 18px;
  }
  .mufredat-course:last-child {
    border-bottom: none;
  }
  .mufredat-course-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 6px;
  }
  .mufredat-course .pillar-hairline {
    margin: 0 0 8px;
  }
  .mufredat-course-link {
    background: none;
    border: none;
    color: var(--ink);
    font-size: 14.5px;
    font-weight: 600;
    cursor: pointer;
    padding: 0;
    text-align: left;
  }
  .mufredat-course-link:hover {
    text-decoration: underline;
  }
  .mufredat-topic {
    display: flex;
    gap: 8px;
    align-items: center;
    padding-top: 10px;
  }
</style>
