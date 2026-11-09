const BARS = 64.0;
const GAP = 0.1;
const RISE_PER_SECOND = 0.25;
const FADE_PER_SECOND = 2.2;
const LAST_BIN = 511.0;

// The loudest bin of a bar. Hearing is logarithmic, so bars widen with pitch.
fn level_of(bar: f32) -> f32 {
    let first = u32(pow(LAST_BIN, bar / BARS));
    let last = max(first + 1u, u32(pow(LAST_BIN, (bar + 1.0) / BARS)));
    var level = 0.0;
    for (var bin = first; bin < min(last, 512u); bin += 1u) {
        level = max(level, audio.spectrum[bin]);
    }
    return level;
}

// The hue drifts on its own, so even a steady sound never leaves a still picture.
fn colour(uv: vec2f) -> vec3f {
    let hue = uv.x * 0.5 + frame.time * 0.03;
    return 0.5 + 0.5 * cos(6.2832 * (hue + vec3f(0.0, 0.33, 0.67)));
}

@fragment fn fs(@builtin(position) position: vec4f) -> @location(0) vec4f {
    let uv = at_pixel(position);
    let across = uv.x * BARS;
    let in_bar = abs(fract(across) - 0.5) < 0.5 - GAP;
    let lit = in_bar && uv.y > 1.0 - level_of(floor(across));

    let from_below = uv + vec2f(0.0, RISE_PER_SECOND * frame.dt);
    let trail = textureSampleLevel(picture, linear_sampler, from_below, 0.0).rgb * exp(-FADE_PER_SECOND * frame.dt);

    return vec4f(select(trail, colour(uv), lit), 1.0);
}
