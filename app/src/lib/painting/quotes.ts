export interface Maxim {
  text: string;
  cite: string;
}

// Sourced only — no generic motivational copy. Picked deterministically by
// day-of-year so the maxim is stable across a day's redraws.
export const MAXIMS: Maxim[] = [
  {
    text: 'If you are distressed by anything external, the pain is not due to the thing itself, but to your own judgment of it — and this you have the power to revoke at any moment.',
    cite: 'Marcus Aurelius · Meditations, VIII.47',
  },
  {
    text: 'We suffer more often in imagination than in reality.',
    cite: 'Seneca · Letters, XIII',
  },
  {
    text: "Man is disturbed not by things, but by the views he takes of them.",
    cite: 'Epictetus · Enchiridion, 5',
  },
  {
    text: 'The impediment to action advances action. What stands in the way becomes the way.',
    cite: 'Marcus Aurelius · Meditations, V.20',
  },
  {
    text: 'It is not that we have a short time to live, but that we waste a lot of it.',
    cite: 'Seneca · On the Shortness of Life, I',
  },
  {
    text: 'No man is free who is not master of himself.',
    cite: 'Epictetus · Fragments',
  },
  {
    text: 'Confine yourself to the present.',
    cite: 'Marcus Aurelius · Meditations, VIII.36',
  },
];

export function maximForDay(dayOfYear: number): Maxim {
  return MAXIMS[dayOfYear % MAXIMS.length];
}
