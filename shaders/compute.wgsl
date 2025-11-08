@group(0) @binding(0) var outputTex : texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) id : vec3<u32>, @builtin(num_workgroups) size : vec3<u32>) {
    // let size = vec2<u32>(512u, 512u);
    if (id.x >= size.x || id.y >= size.y) {
        return;
    }

    let color = vec4<f32>(
        f32(id.x) / f32(size.x),
        f32(id.y) / f32(size.y),
        0.3,
        1.0
    );

    textureStore(outputTex, vec2<i32>(id.xy), color);
}
