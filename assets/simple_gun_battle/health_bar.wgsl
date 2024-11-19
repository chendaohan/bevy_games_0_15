#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> ratio: f32;

@fragment fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let uv_abs = abs(mesh.uv - 0.5);
    var color = vec4f(vec3f(0.), 1.);
    if uv_abs.x < 0.49 && uv_abs.y < 0.4 {
        if mesh.uv.x < ratio {
            color = vec4f(0., 0.65, 0., 1.);
        } else {
            color = vec4f(vec3f(0.3), 1.);
        }
    }
    return color;
}