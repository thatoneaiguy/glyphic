@vertex
fn vs_main(@builtin(vertex_index) vert_index: u32) -> @builtin(position) vec4<f32> {
    var pos: vec2<f32>;

    switch(vert_index) {
        case 0u: { pos = vec2<f32>(-1.0, -1.0); }
        case 1u: { pos = vec2<f32>(3.0, -1.0); }
        case 2u: { pos = vec2<f32>(-1.0, 3.0); }
        default: { pos = vec2<f32>(0.0, 0.0); }
    }

    return vec4<f32>(pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.7, 0.8, 1.0, 1.0);
}