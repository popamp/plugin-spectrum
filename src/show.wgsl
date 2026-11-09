@fragment fn fs(@builtin(position) position: vec4f) -> @location(0) vec4f {
    return vec4f(textureSampleLevel(picture, linear_sampler, at_pixel(position), 0.0).rgb, 1.0);
}
