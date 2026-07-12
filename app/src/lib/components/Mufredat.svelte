<script lang="ts">
  import { onMount } from 'svelte';
  import { curriculum } from '../stores/curriculum.svelte';
  import { today } from '../stores/today.svelte';
  import { ritual } from '../stores/ritual.svelte';
  import CourseDetail from './CourseDetail.svelte';
  import type { CourseView } from '../api/types';

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
    <span class="action-meta" style="margin-left:auto">{curriculum.courses.length} courses</span>
  </div>

  <form class="task-add" onsubmit={submitCourse}>
    <input class="task-input" placeholder="New course…" bind:value={courseName} />
    {#if today.view}
      <select class="task-select" bind:value={coursePillarId}>
        <option value="">Pillar…</option>
        {#each today.view.pillars as pillar (pillar.id)}
          <option value={pillar.id}>{pillar.name}</option>
        {/each}
      </select>
    {/if}
    <button class="btn primary" type="submit">Add course</button>
  </form>

  {#if curriculum.courses.length === 0}
    <p class="empty-state">No courses yet — plant the first row.</p>
  {:else}
    {#each curriculum.courses as course (course.id)}
      {@const pct = course.total_lessons === 0 ? 0 : Math.round((course.done_lessons / course.total_lessons) * 100)}
      <div class="mufredat-course">
        <div class="mufredat-course-header">
          <span class="pillar-dot" style={`background:${course.color_token}`}></span>
          <button class="mufredat-course-link" onclick={() => (detailCourseId = course.id)}
            >{course.name}</button
          >
          <span class="action-meta">{course.done_lessons}/{course.total_lessons} planted</span>
        </div>
        <div class="pillar-hairline">
          <div style={`width:${pct}%;background:${course.color_token}`}></div>
        </div>

        {#if course.due_lessons.length === 0}
          <p class="empty-state">Nothing due — add a topic below.</p>
        {:else}
          {#each course.due_lessons as lesson (lesson.id)}
            <div class="action-row">
              <span class="action-name">{lesson.title}</span>
              {#if lesson.review_of}<span class="action-meta">review</span>{/if}
              <button class="focus-btn" onclick={() => study(course, lesson.id, lesson.title)}
                >Study</button
              >
              <button class="focus-btn" onclick={() => curriculum.completeLesson(lesson.id)}
                >Mark done</button
              >
              <button class="focus-btn" onclick={() => curriculum.skipLesson(lesson.id)}>Skip</button>
            </div>
          {/each}
        {/if}

        <form class="task-add" onsubmit={(e) => submitLesson(course.id, e)}>
          <input
            class="task-input"
            placeholder="Add a topic…"
            value={lessonDrafts[course.id] ?? ''}
            oninput={(e) => (lessonDrafts[course.id] = (e.target as HTMLInputElement).value)}
          />
          <button class="btn" type="submit">Add topic</button>
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
</style>
