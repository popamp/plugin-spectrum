struct Frame { time: f32, dt: f32, width: f32, height: f32 }
struct Audio { wave: array<f32, 1024>, spectrum: array<f32, 512> }

@group(0) @binding(0) var<uniform> frame: Frame;
@group(0) @binding(1) var<storage, read> audio: Audio;
@group(0) @binding(3) var linear_sampler: sampler;
@group(0) @binding(4) var picture: texture_2d<f32>;

// One triangle that covers the window.
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4f {
    return vec4f(f32(i & 1) * 4.0 - 1.0, f32(i >> 1) * 4.0 - 1.0, 0.0, 1.0);
}

fn at_pixel(position: vec4f) -> vec2f {
    return position.xy / vec2f(frame.width, frame.height);
}
