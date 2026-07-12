// The living landscape — Van Gogh-style flow-field brushwork, ported from
// design/mockups/sessiz-mod.html. Pure canvas drawing; no DOM/UI concerns.

export type WeatherName = 'clear' | 'radiant' | 'heavy' | 'stormy';

// ---- Real weather (Open-Meteo) — independent of the Inner Weather mood
// layer above. This is what the sky actually shows outside; it never
// changes brush turbulence, only adds a real-condition overlay on top.

export type RealCondition =
  | 'clear'
  | 'partly_cloudy'
  | 'overcast'
  | 'fog'
  | 'rain'
  | 'storm'
  | 'snow';

export interface RealWeatherInput {
  condition: RealCondition;
  windKmh: number;
  precipitationMm: number;
}

/** WMO weather code (Open-Meteo) -> simplified condition bucket. */
export function conditionFromWmoCode(code: number): RealCondition {
  if (code === 0) return 'clear';
  if (code === 1 || code === 2) return 'partly_cloudy';
  if (code === 3) return 'overcast';
  if (code === 45 || code === 48) return 'fog';
  if ((code >= 51 && code <= 67) || (code >= 80 && code <= 82)) return 'rain';
  if ((code >= 71 && code <= 77) || code === 85 || code === 86) return 'snow';
  if (code >= 95) return 'storm';
  return 'clear';
}

export interface WeatherConfig {
  name: WeatherName;
  filter: string;
  mist: number;
  amp: number;
  freq: number;
  len: number;
  count: number;
  vortex: number;
  thick: number;
  dense: number;
  pal: string[];
}

// Sky keyframes: [hour, top, middle, horizon] — bold Provence blue by day.
const SKY: [number, string, string, string][] = [
  [0, '#060814', '#0b1130', '#181c44'],
  [4.5, '#0a0f2e', '#231b4e', '#4c2a56'],
  [6, '#1c5590', '#3a7ab8', '#e89a62'],
  [7.5, '#1273c4', '#2f92d6', '#f6c98a'],
  [11, '#0e6fc9', '#2f97dd', '#eaf3ea'],
  [15, '#0d6cc6', '#2f95db', '#eef2df'],
  [18, '#1a5aa8', '#b56a78', '#f4a05e'],
  [19.5, '#252a66', '#6e3f78', '#d96a56'],
  [21, '#0e1338', '#2c2158', '#5e3260'],
  [24, '#060814', '#0b1130', '#181c44'],
];

// Turbulence: how violently the brushwork swirls per inner-weather state.
// `thick` and `dense` drive stroke width/count everywhere the brush touches
// canvas — sky AND field — so mood reads the same across the whole scene.
export const WEATHER: Record<WeatherName, WeatherConfig> = {
  clear: {
    name: 'clear',
    filter: 'none',
    mist: 0.06,
    amp: 0.55,
    freq: 1.0,
    len: 62,
    count: 0.85,
    vortex: 0.55,
    thick: 1.0,
    dense: 1.0,
    pal: ['#1c6fc2', '#3f93df', '#eef3f6', '#0e4f96'],
  },
  radiant: {
    name: 'radiant',
    filter: 'saturate(1.25) brightness(1.1)',
    mist: 0.03,
    amp: 0.7,
    freq: 1.15,
    len: 68,
    count: 0.95,
    vortex: 0.75,
    thick: 1.3,
    dense: 1.2,
    pal: ['#ffd23f', '#ff9d3d', '#3f93df', '#fff3c4'],
  },
  heavy: {
    name: 'heavy',
    filter: 'saturate(0.5) brightness(0.8)',
    mist: 0.28,
    amp: 0.35,
    freq: 0.7,
    len: 46,
    count: 0.65,
    vortex: 0.35,
    thick: 0.62,
    dense: 0.55,
    pal: ['#2b3252', '#3a3f5c', '#20223a', '#454a68'],
  },
  stormy: {
    name: 'stormy',
    filter: 'saturate(0.6) brightness(0.68) contrast(1.12)',
    mist: 0.22,
    amp: 1.05,
    freq: 1.75,
    len: 78,
    count: 1.2,
    vortex: 1.15,
    thick: 1.55,
    dense: 1.35,
    pal: ['#1c2140', '#3d2f66', '#0f1230', '#5a3a6e'],
  },
};

const hex = (h: string): [number, number, number] => [
  parseInt(h.slice(1, 3), 16),
  parseInt(h.slice(3, 5), 16),
  parseInt(h.slice(5, 7), 16),
];
const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
// Format-agnostic: accepts "#rrggbb" or "rgb(r,g,b)" on either side.
const toRgbArr = (c: string): number[] =>
  c[0] === '#' ? hex(c) : (c.match(/[\d.]+/g) ?? []).map(Number);
const mix = (a: string, b: string, t: number): string => {
  const [r1, g1, b1] = toRgbArr(a);
  const [r2, g2, b2] = toRgbArr(b);
  return `rgb(${lerp(r1, r2, t) | 0},${lerp(g1, g2, t) | 0},${lerp(b1, b2, t) | 0})`;
};

function skyAt(h: number): [string, string, string] {
  for (let i = 0; i < SKY.length - 1; i++) {
    const [h0] = SKY[i];
    const [h1] = SKY[i + 1];
    if (h >= h0 && h <= h1) {
      const t = (h - h0) / (h1 - h0);
      return [
        mix(SKY[i][1], SKY[i + 1][1], t),
        mix(SKY[i][2], SKY[i + 1][2], t),
        mix(SKY[i][3], SKY[i + 1][3], t),
      ];
    }
  }
  return [SKY[0][1], SKY[0][2], SKY[0][3]];
}

// Seeded pseudo-random (stable stars & ridges between frames).
function rng(seed: number) {
  let s = seed;
  return () => (s = (s * 16807) % 2147483647) / 2147483647;
}

export function darkness(h: number): number {
  // 0 = full day, 1 = full night
  if (h >= 8 && h <= 17) return 0;
  if (h < 5 || h > 21) return 1;
  if (h < 8) return 1 - (h - 5) / 3;
  return (h - 17) / 4;
}

type Field = (u: number, v: number) => number;

function makeField(seed: number, vortexStrength: number, freq: number): Field {
  const r = rng(seed);
  const vortices = [
    { x: r(), y: 0.25 + r() * 0.3, s: vortexStrength, cw: 1 },
    { x: r(), y: 0.15 + r() * 0.25, s: vortexStrength * 0.7, cw: -1 },
  ];
  return (u, v) => {
    let ang = Math.sin(u * 6.5 * freq + v * 3.1) * 1.4 + Math.cos(v * 5.2 * freq - u * 2.4) * 0.9;
    for (const vt of vortices) {
      const dx = u - vt.x;
      const dy = v - vt.y;
      const d2 = dx * dx + dy * dy + 0.02;
      const w = vt.s / d2;
      ang += (Math.atan2(dy, dx) + (vt.cw * Math.PI) / 2) * Math.min(w, 6) * 0.18;
    }
    return ang;
  };
}

function paintStroke(
  ctx: CanvasRenderingContext2D,
  W: number,
  H: number,
  x0: number,
  y0: number,
  len: number,
  angle0: number,
  field: Field,
  width: number,
  color: string,
  alpha: number,
) {
  ctx.beginPath();
  ctx.moveTo(x0, y0);
  let x = x0;
  let y = y0;
  let a = angle0;
  const steps = 6;
  for (let i = 0; i < steps; i++) {
    const localAng = field(x / W, y / H) * 0.7 + a * 0.3;
    x += Math.cos(localAng) * (len / steps);
    y += Math.sin(localAng) * (len / steps) * 0.6;
    ctx.lineTo(x, y);
    a = localAng;
  }
  ctx.strokeStyle = color;
  ctx.globalAlpha = alpha;
  ctx.lineWidth = width;
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
  ctx.stroke();
}

function paintSwirlSky(
  ctx: CanvasRenderingContext2D,
  W: number,
  horizonY: number,
  wx: WeatherConfig,
  h: number,
  seedBase: number,
) {
  const field = makeField(seedBase, wx.vortex, wx.freq);
  const r = rng(seedBase + 1);
  const base = skyAt(h);
  const cell = 26 / Math.sqrt(wx.dense);
  const cols = Math.ceil(W / cell) + 1;
  const rows = Math.ceil(horizonY / cell) + 1;
  for (let gy = 0; gy < rows; gy++) {
    for (let gx = 0; gx < cols; gx++) {
      const x = gx * cell + (r() - 0.5) * cell * 1.6;
      const y = gy * cell + (r() - 0.5) * cell * 1.6;
      const u = x / W;
      const v = Math.max(0, Math.min(1, y / horizonY));
      const under = mix(base[0], base[2], v);
      const col = mix(under, wx.pal[Math.floor(r() * wx.pal.length)], 0.3 + r() * 0.45);
      const ang = field(u, v) + (r() - 0.5) * wx.amp;
      paintStroke(
        ctx,
        W,
        horizonY,
        x,
        y,
        wx.len * (0.5 + r() * 0.6),
        ang,
        field,
        (5 + r() * 6) * wx.thick,
        col,
        0.55 + r() * 0.4,
      );
    }
  }
  const n = Math.round(220 * wx.count);
  for (let i = 0; i < n; i++) {
    const u = r();
    const v = r() * 0.95;
    const x = u * W;
    const y = v * horizonY;
    const under = mix(base[0], base[2], v);
    const col = mix(under, wx.pal[Math.floor(r() * wx.pal.length)], 0.5 + r() * 0.4);
    const ang = field(u, v) + (r() - 0.5) * wx.amp;
    paintStroke(
      ctx,
      W,
      horizonY,
      x,
      y,
      wx.len * (0.55 + r() * 0.6),
      ang,
      field,
      (4 + r() * 5) * wx.thick,
      col,
      0.4 + r() * 0.35,
    );
  }
  ctx.globalAlpha = 1;
}

function paintHouse(ctx: CanvasRenderingContext2D, W: number, horizonY: number, dk: number, seedBase: number) {
  const r = rng(seedBase + 71);
  const hw = W * 0.085;
  const hh = horizonY * 0.16;
  const hx = W * 0.3;
  const hy = horizonY - hh;
  const dim = (c: string) => mix(c, '#0e1020', Math.min(dk * 1.05, 0.82));
  const jitterRect = (x: number, y: number, w: number, hgt: number, col: string, n: number) => {
    ctx.fillStyle = col;
    for (let i = 0; i < n; i++) {
      const sx = x + r() * w;
      const sy = y + r() * hgt;
      ctx.globalAlpha = 0.5 + r() * 0.45;
      ctx.fillRect(sx - 2, sy - 2, 5 + r() * 8, 3 + r() * 4);
    }
    ctx.globalAlpha = 1;
  };
  ctx.fillStyle = dim('#e8c04a');
  ctx.fillRect(hx, hy, hw, hh);
  ctx.fillRect(hx + hw, hy + hh * 0.22, hw * 0.55, hh * 0.78);
  jitterRect(hx, hy, hw * 1.5, hh, dim('#f2d675'), 40);
  jitterRect(hx, hy, hw * 1.5, hh, dim('#c99a2e'), 22);
  ctx.fillStyle = dim('#c96a3a');
  ctx.beginPath();
  ctx.moveTo(hx - 6, hy);
  ctx.lineTo(hx + hw * 0.5, hy - hh * 0.42);
  ctx.lineTo(hx + hw + 6, hy);
  ctx.closePath();
  ctx.fill();
  ctx.fillRect(hx + hw - 2, hy + hh * 0.22 - hh * 0.14, hw * 0.55 + 6, hh * 0.15);
  jitterRect(hx - 4, hy - hh * 0.4, hw + 8, hh * 0.38, dim('#e08248'), 18);
  ctx.fillStyle = dim('#3a7a8c');
  ctx.fillRect(hx + hw * 0.14, hy + hh * 0.3, hw * 0.15, hh * 0.3);
  ctx.fillRect(hx + hw * 0.55, hy + hh * 0.3, hw * 0.15, hh * 0.3);
  ctx.fillRect(hx + hw * 0.33, hy + hh * 0.5, hw * 0.16, hh * 0.5);
  ctx.fillRect(hx + hw * 1.12, hy + hh * 0.5, hw * 0.14, hh * 0.35);
  if (dk > 0.4) {
    ctx.fillStyle = '#ffca5f';
    ctx.globalAlpha = (dk - 0.4) * 1.4;
    ctx.fillRect(hx + hw * 0.55, hy + hh * 0.3, hw * 0.15, hh * 0.3);
    ctx.globalAlpha = 1;
  }
}

function paintPath(
  ctx: CanvasRenderingContext2D,
  W: number,
  H: number,
  horizonY: number,
  dk: number,
  wx: WeatherConfig,
  seedBase: number,
) {
  const r = rng(seedBase + 81);
  const dim = (c: string) => mix(c, '#101224', Math.min(dk * 1.0, 0.8));
  const cream = ['#e9d9a0', '#dcc783', '#f2e6bb', '#c9b370'];
  const count = Math.round(260 * wx.dense);
  for (let i = 0; i < count; i++) {
    const t = r();
    const yy = horizonY + t * t * (H - horizonY);
    const center = W * (0.34 - t * 0.3);
    const halfw = 8 + t * t * W * 0.075;
    const x = center + (r() - 0.5) * 2 * halfw;
    const len = 6 + t * 26 + r() * 10;
    const ang = 0.9 + (r() - 0.5) * (0.5 + wx.amp * 0.3);
    ctx.strokeStyle = dim(cream[Math.floor(r() * cream.length)]);
    ctx.lineWidth = (2 + t * 4 + r() * 2) * wx.thick;
    ctx.lineCap = 'round';
    ctx.globalAlpha = 0.5 + r() * 0.4;
    ctx.beginPath();
    ctx.moveTo(x, yy);
    ctx.lineTo(x + Math.cos(ang) * len * 0.4, yy + Math.sin(ang) * len);
    ctx.stroke();
  }
  ctx.globalAlpha = 1;
}

function paintHalo(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  coreColor: string,
  glowColor: string,
  radius: number,
  rings: number,
) {
  for (let i = rings; i >= 1; i--) {
    const rr = radius * (i / rings);
    ctx.beginPath();
    ctx.arc(cx, cy, rr, 0, Math.PI * 2);
    ctx.strokeStyle = glowColor;
    ctx.lineWidth = 3 + (rings - i) * 0.6;
    ctx.globalAlpha = 0.1 + (0.05 * (rings - i)) / rings;
    ctx.stroke();
  }
  ctx.globalAlpha = 1;
  ctx.beginPath();
  ctx.arc(cx, cy, radius * 0.18, 0, Math.PI * 2);
  ctx.fillStyle = coreColor;
  ctx.fill();
}

function paintStars(ctx: CanvasRenderingContext2D, W: number, horizonY: number, dk: number, seed: number) {
  if (dk <= 0.05) return;
  const r = rng(seed);
  const count = 9;
  for (let i = 0; i < count; i++) {
    const x = r() * W;
    const y = r() * horizonY * 0.8;
    const size = 10 + r() * 20;
    ctx.globalAlpha = dk * (0.55 + r() * 0.35);
    paintHalo(ctx, x, y, '#fff6cf', '#ffe98a', size, 4);
  }
  ctx.globalAlpha = 1;
}

function paintTree(
  ctx: CanvasRenderingContext2D,
  W: number,
  groundY: number,
  dk: number,
  wx: WeatherConfig,
  seedBase: number,
) {
  const r = rng(seedBase + 41);
  const trunkX = W * 0.15;
  const canopyCx = trunkX + W * 0.015;
  const canopyCy = groundY - groundY * 0.36;
  const canopyR = groundY * 0.34;
  const dim = (c: string) => mix(c, '#0a0e14', Math.min(dk * 1.1, 0.85));

  for (const [tx, wdt] of [
    [trunkX, 15],
    [trunkX + W * 0.055, 8],
  ]) {
    for (let i = 0; i < 9; i++) {
      const t = i / 8;
      const y0 = groundY + 14 - t * (groundY + 14 - canopyCy);
      ctx.strokeStyle = dim(r() > 0.5 ? '#4a6a8a' : '#2e2418');
      ctx.lineWidth = (wdt - t * (wdt * 0.6)) * wx.thick;
      ctx.lineCap = 'round';
      ctx.globalAlpha = 0.85;
      ctx.beginPath();
      ctx.moveTo(tx + Math.sin(t * 4) * 4, y0 + 18);
      ctx.lineTo(tx + Math.sin((t + 0.15) * 4) * 4, y0 - 18);
      ctx.stroke();
    }
  }

  const greens = ['#1f3d18', '#2e5a22', '#3f7a2a', '#274a1c', '#163012', '#4c7a26'];
  const sparks = ['#c9a23a', '#b0522e', '#6f9a3a', '#8aab4a'];
  const leafCount = Math.round(520 * wx.dense);
  for (let i = 0; i < leafCount; i++) {
    const ang = r() * Math.PI * 2;
    const rad = Math.sqrt(r()) * canopyR;
    const x = canopyCx + Math.cos(ang) * rad * 1.25;
    const y = canopyCy + Math.sin(ang) * rad * 0.9 - rad * 0.1;
    const size = (4 + r() * 10) * wx.thick;
    const warm = r() > 0.86;
    ctx.fillStyle = dim(warm ? sparks[Math.floor(r() * sparks.length)] : greens[Math.floor(r() * greens.length)]);
    ctx.globalAlpha = 0.55 + r() * 0.4;
    ctx.beginPath();
    ctx.ellipse(x, y, size, size * 0.66, r() * Math.PI, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.globalAlpha = 1;
}

function paintMeadow(
  ctx: CanvasRenderingContext2D,
  W: number,
  horizonY: number,
  dk: number,
  wx: WeatherConfig,
  seedBase: number,
) {
  const r = rng(seedBase + 51);
  const dim = (c: string) => mix(c, '#0c101c', Math.min(dk * 1.05, 0.85));
  const bandH = horizonY * 0.075;
  const hedge = ['#1e4a24', '#173a1d', '#2a5c2e'];
  const hedgeCount = Math.round(220 * wx.dense);
  for (let i = 0; i < hedgeCount; i++) {
    const x = r() * W;
    const y = horizonY - 2 + r() * 6;
    ctx.fillStyle = dim(hedge[Math.floor(r() * hedge.length)]);
    ctx.globalAlpha = 0.5 + r() * 0.4;
    ctx.beginPath();
    ctx.ellipse(x, y, (5 + r() * 14) * wx.thick, (3 + r() * 5) * wx.thick, 0, 0, Math.PI * 2);
    ctx.fill();
  }
  const grass = ['#2f7a2e', '#3f9a38', '#25601f', '#57aa42', '#1e6f4a'];
  const grassCount = Math.round(420 * wx.dense);
  for (let i = 0; i < grassCount; i++) {
    const x = r() * W;
    const y = horizonY + 2 + r() * bandH;
    const len = (6 + r() * 14) * (0.7 + wx.amp * 0.4);
    ctx.strokeStyle = dim(grass[Math.floor(r() * grass.length)]);
    ctx.lineWidth = (2 + r() * 3) * wx.thick;
    ctx.lineCap = 'round';
    ctx.globalAlpha = 0.55 + r() * 0.4;
    ctx.beginPath();
    ctx.moveTo(x, y);
    ctx.lineTo(x + (r() - 0.5) * len, y + r() * 5 - 2);
    ctx.stroke();
  }
  const flowerCount = Math.round(90 * wx.dense);
  for (let i = 0; i < flowerCount; i++) {
    const x = r() * W;
    const y = horizonY + bandH * (0.7 + r() * 0.5);
    const poppy = r() > 0.75;
    ctx.fillStyle = dim(poppy ? '#c43a2a' : '#f2ede0');
    ctx.globalAlpha = (0.5 + r() * 0.4) * (1 - dk * 0.6);
    ctx.beginPath();
    ctx.ellipse(x, y, (2.5 + r() * 4) * wx.thick, (2 + r() * 3) * wx.thick, r() * Math.PI, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.globalAlpha = 1;
}

function paintCrows(
  ctx: CanvasRenderingContext2D,
  W: number,
  horizonY: number,
  dk: number,
  wx: WeatherConfig,
  seedBase: number,
) {
  const r = rng(seedBase + 91);
  const base: Record<WeatherName, number> = { clear: 4, radiant: 3, heavy: 11, stormy: 16 };
  const count = Math.round((base[wx.name] ?? 6) * (0.7 + r() * 0.6));
  const ink = mix('#1a1610', '#000000', 0.15);
  for (let i = 0; i < count; i++) {
    const x = r() * W;
    const y = horizonY * (0.12 + r() * 0.55);
    const size = 5 + r() * 7;
    const tilt = (r() - 0.5) * 0.6;
    ctx.strokeStyle = ink;
    ctx.lineWidth = 1.6 + r() * 1.2;
    ctx.lineCap = 'round';
    ctx.globalAlpha = 0.55 + r() * 0.3;
    ctx.beginPath();
    ctx.moveTo(x - size, y + size * 0.3 + tilt * size);
    ctx.quadraticCurveTo(x - size * 0.25, y - size * 0.6, x, y);
    ctx.quadraticCurveTo(x + size * 0.25, y - size * 0.6, x + size, y + size * 0.3 - tilt * size);
    ctx.stroke();
  }
  ctx.globalAlpha = 1;
}

function paintWheatField(
  ctx: CanvasRenderingContext2D,
  W: number,
  H: number,
  groundY: number,
  dk: number,
  wx: WeatherConfig,
  seedBase: number,
) {
  const r = rng(seedBase + 61);
  const dim = (c: string) => mix(c, '#0e1020', Math.min(dk * 0.9, 0.75));
  const wheatTop = groundY + (H - groundY) * 0.18;
  const gold = ['#e8c34a', '#f0d878', '#c9a23a', '#f7e6a0', '#dab13a', '#e6cf6f'];
  const accent = ['#8aa0c9', '#6f86b8', '#a9b8d9'];
  const tufts = ['#5c7a2e', '#48651f', '#6f8a3a'];
  const cell = 16 / Math.sqrt(wx.dense);
  const cols = Math.ceil(W / cell) + 1;
  const rows = Math.ceil((H - wheatTop) / cell) + 1;
  for (let gy = 0; gy < rows; gy++) {
    for (let gx = 0; gx < cols; gx++) {
      const x = gx * cell + (r() - 0.5) * cell * 1.5;
      const y = wheatTop + gy * cell + (r() - 0.5) * cell * 1.5;
      const depth = Math.max(0, Math.min(1, (y - wheatTop) / (H - wheatTop)));
      const len = 12 + depth * 22 + r() * 10;
      const ang = -0.35 + (r() - 0.5) * (0.3 + wx.amp * 0.5) - depth * 0.15;
      const pick = r();
      const col =
        pick > 0.9
          ? accent[Math.floor(r() * accent.length)]
          : pick > 0.82
            ? tufts[Math.floor(r() * tufts.length)]
            : gold[Math.floor(r() * gold.length)];
      ctx.strokeStyle = dim(col);
      ctx.lineWidth = (2 + depth * 3.5 + r() * 2) * wx.thick;
      ctx.lineCap = 'round';
      ctx.globalAlpha = 0.5 + depth * 0.3 + r() * 0.2;
      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.lineTo(x + Math.cos(ang) * len, y + Math.sin(ang) * len);
      ctx.stroke();
    }
  }
  ctx.globalAlpha = 1;
}

// A soft grey veil over the sky for overcast/fog conditions — the real sun
// stays hidden behind it regardless of the hour.
function paintCloudVeil(ctx: CanvasRenderingContext2D, W: number, horizonY: number, amount: number) {
  if (amount <= 0) return;
  ctx.fillStyle = `rgba(180,184,196,${amount})`;
  ctx.fillRect(0, 0, W, horizonY + 2);
}

// Diagonal rain (or near-vertical snow) streaks over the whole scene —
// density tracks real precipitation, angle tracks real wind.
function paintPrecipitation(
  ctx: CanvasRenderingContext2D,
  W: number,
  H: number,
  intensityMm: number,
  windKmh: number,
  isSnow: boolean,
  seedBase: number,
) {
  const r = rng(seedBase + 111);
  const count = Math.round(Math.min(intensityMm, 12) * (isSnow ? 18 : 40) + 20);
  const tilt = Math.max(-0.6, Math.min(0.6, windKmh / 60));
  for (let i = 0; i < count; i++) {
    const x = r() * (W + 200) - 100;
    const y = r() * H;
    if (isSnow) {
      ctx.fillStyle = `rgba(255,255,255,${0.35 + r() * 0.35})`;
      ctx.beginPath();
      ctx.arc(x, y, 1.2 + r() * 2, 0, Math.PI * 2);
      ctx.fill();
    } else {
      const len = 14 + r() * 22;
      ctx.strokeStyle = `rgba(210,220,235,${0.18 + r() * 0.22})`;
      ctx.lineWidth = 1 + r() * 1;
      ctx.lineCap = 'round';
      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.lineTo(x + tilt * len, y + len);
      ctx.stroke();
    }
  }
}

/** Render one frame of the living landscape at hour `h` (0-24) into `ctx`. */
export function renderPainting(
  ctx: CanvasRenderingContext2D,
  W: number,
  H: number,
  h: number,
  weather: WeatherName,
  real?: RealWeatherInput,
): string {
  const [top, mid, hor] = skyAt(h);
  const wx = WEATHER[weather];
  const horizonY = H * 0.62;

  // Real wind tilts the wheat/meadow/path brushwork the same way Inner
  // Weather turbulence does — reuses the existing `amp`-driven angle jitter,
  // just nudged by the actual wind speed outside.
  const windBoost = real ? Math.min(real.windKmh / 50, 1) * 0.7 : 0;
  const groundWx = windBoost > 0 ? { ...wx, amp: wx.amp + windBoost } : wx;

  const g = ctx.createLinearGradient(0, 0, 0, horizonY);
  g.addColorStop(0, top);
  g.addColorStop(0.55, mid);
  g.addColorStop(1, hor);
  ctx.fillStyle = g;
  ctx.fillRect(0, 0, W, horizonY + 2);

  const isSun = h >= 5.5 && h <= 20;
  const tArc = isSun ? (h - 5.5) / 14.5 : (h >= 20 ? h - 20 : h + 4) / 9.5;
  const cx = W * (0.12 + 0.76 * tArc);
  const cy = horizonY - Math.sin(tArc * Math.PI) * horizonY * 0.72;
  if (isSun) {
    const low = 1 - Math.sin(tArc * Math.PI);
    const sunCol = mix('rgb(255,244,214)', 'rgb(255,158,92)', low);
    paintHalo(ctx, cx, cy, sunCol, '#ffd23f', 100, 7);
  } else {
    paintHalo(ctx, cx, cy, '#eef1fb', '#cfd8ff', 70, 6);
  }

  ctx.save();
  paintSwirlSky(ctx, W, horizonY, wx, h, 500);
  ctx.restore();

  const dk = darkness(h);
  paintStars(ctx, W, horizonY, dk, 900);
  paintCrows(ctx, W, horizonY, dk, wx, 500);

  const groundBase = mix('rgb(196,167,74)', 'rgb(14,16,28)', Math.min(dk * 1.15, 0.92));
  const groundGrad = ctx.createLinearGradient(0, horizonY, 0, H);
  groundGrad.addColorStop(0, mix(groundBase, 'rgb(70,120,60)', 0.4));
  groundGrad.addColorStop(0.2, mix(groundBase, 'rgb(120,140,70)', 0.25));
  groundGrad.addColorStop(1, groundBase);
  ctx.fillStyle = groundGrad;
  ctx.fillRect(0, horizonY - 2, W, H - horizonY + 2);

  paintHouse(ctx, W, horizonY, dk, 500);
  paintMeadow(ctx, W, horizonY, dk, groundWx, 500);
  paintWheatField(ctx, W, H, horizonY, dk, groundWx, 500);
  paintPath(ctx, W, H, horizonY, dk, groundWx, 500);
  paintTree(ctx, W, horizonY, dk, wx, 500);

  ctx.save();
  const mist = ctx.createLinearGradient(0, horizonY - 40, 0, horizonY + 120);
  mist.addColorStop(0, 'rgba(235,235,245,0)');
  mist.addColorStop(0.5, `rgba(235,235,245,${wx.mist})`);
  mist.addColorStop(1, 'rgba(235,235,245,0)');
  ctx.fillStyle = mist;
  ctx.fillRect(0, horizonY - 40, W, 180);
  ctx.restore();

  if (real) {
    const cloudAmount =
      real.condition === 'overcast'
        ? 0.22
        : real.condition === 'fog'
          ? 0.32
          : real.condition === 'partly_cloudy'
            ? 0.08
            : real.condition === 'rain' || real.condition === 'storm'
              ? 0.16
              : 0;
    paintCloudVeil(ctx, W, horizonY, cloudAmount);

    if (real.condition === 'rain' || real.condition === 'storm') {
      paintPrecipitation(ctx, W, H, real.precipitationMm, real.windKmh, false, 700);
    } else if (real.condition === 'snow') {
      paintPrecipitation(ctx, W, H, Math.max(real.precipitationMm, 2), real.windKmh, true, 700);
    }
  }

  return wx.filter;
}
