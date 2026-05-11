@group(0) @binding(0) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> camera: Camera;
struct Camera {
    width: u32,
    height: u32,
    samples_per_pixel: i32,
    pixels_sample_scale: f32,
    image_center: vec3<f32>,
    pixel_delta_u: vec3<f32>,
    pixel_delta_v: vec3<f32>,
    center: vec3<f32>,
    defocus_disk_u: vec3<f32>,
    defocus_disk_v: vec3<f32>,
    maximum_depth: i32,
    defocus_angle: f32,
    _pad0: f32,
    _pad1: f32,
}

@group(1) @binding(0) var<uniform> util: UtilData;
struct UtilData {
    time: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(1) @binding(1) var random_texture: texture_2d<f32>;
@group(1) @binding(2) var random_sampler: sampler;

var<workgroup> RANDOM_COUNTER_X: u32;
var<workgroup> RANDOM_COUNTER_Y: u32;
const RANDOM_IMAGE_WIDTH = 64;
const RANDOM_IMAGE_HEIGHT = 64;

const SKY_COLOR = vec3(0.5, 0.7, 1.0);

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>,
}

fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.dir;
}

fn get_ray(uv: vec2<f32>) -> Ray {
    let pixel_center = camera.image_center + 
        camera.pixel_delta_u * uv.x + 
        camera.pixel_delta_v * uv.y;

    let dir = pixel_center - camera.center;

    return Ray(camera.center, dir);
}

fn sky_color(uv: vec2<f32>) -> vec3<f32> {
    return uv.y * SKY_COLOR;
}

fn get_color(uv: vec2<f32>) -> vec4<f32> {
    let ray = get_ray(uv);
    return vec4(ray.origin, 1.0);
    // var current_ray = get_ray(x, y);
    // var hit = false;
    // if current_ray
    // var color = vec3(1.0, 1.0, 1.0);

    // for(var i = 0; i < 1000; i++) {
    //     let record = hit_world(current_ray, 0.001, 100);
    //     if (record.hit) {
    //         let scatter = normalize(record.normal + random_unit_vec3());
    //         current_ray = Ray(record.point, scatter);
    //         color *= vec3(0.5, 0.0, 0.0);
    //         hit = true;
    //         // return vec3(1.0, 0.0, 0.0);
    //     } else {
    //         color *= sky_color();
    //         break;
    //     }
    // }

    // if (!hit) {
    //     return sky_color();
    // } else {
    //     return color;
    // }
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) workgroup_size: vec3<u32>) {
    // let uv = vec2<f32>(invocation_id.xy) / vec2<f32>(workgroup_size.xy);
    let uv = vec2(f32(invocation_id.x) / f32(workgroup_size.x), 1.0 - f32(invocation_id.y) / f32(workgroup_size.y));
    let color = get_color(uv);

    textureStore(outputTex, vec2<i32>(invocation_id.xy), color);
}


