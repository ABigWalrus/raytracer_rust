@group(0) @binding(0) var outputTex : texture_storage_2d<rgba8unorm, write>;
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

@group(1) @binding(0) var<uniform> util : UtilData;
struct UtilData {
    time: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(1) @binding(1) var random_texture: texture_2d<f32>;
@group(1) @binding(2) var random_sampler: sampler;

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>
}

fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.dir;
}

fn get_ray(u: u32, v: u32) -> Ray {
    let pixel_center = camera.image_center + 
        camera.pixel_delta_u * f32(u) + 
        camera.pixel_delta_v * f32(v);
    
    let dir = pixel_center - camera.center;

    return Ray(camera.center, dir);
}

fn hit_sphere(center: vec3<f32>, radius: f32, ray: Ray) -> f32 {
    let oc = center - ray.origin;

    let dir_l = length(ray.dir);
    let a = dir_l * dir_l;
    let h = dot(ray.dir, oc);

    let oc_l = length(oc);
    let c = oc_l * oc_l - radius * radius;

    let discr = h * h - a * c;

    if (discr < 0) {
        return -1.0;
    } else {
        return (h - sqrt(discr)) / a;
    }
}


fn get_color(x: u32, y: u32) -> vec4<f32> {
    let ray = get_ray(x, y);
    let sphere_center = vec3(0.0, 0.0, 0.0);
    let t = hit_sphere(sphere_center, 0.5, ray);
    if (t > 0.0) {
        let normal = normalize(ray_at(ray, t) - sphere_center);
        return vec4(0.5 * vec3(normal.x + 1, normal.y + 1, normal.z + 1), 1.0);
    }
    let unit_dir = normalize(ray.dir);
    let a= 0.5 * (unit_dir.y + 1.0);
    return vec4((1.0 - a) * vec3(1.0) + a * vec3(0.5, 0.7, 1.0), 1.0);
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) id : vec3<u32>, @builtin(num_workgroups) size : vec3<u32>) {
    // let camera = build_camera(size.x, size.y);

    // let pixel = vec2(f32(id.x)/f32(size.x), f32(id.y)/f32(size.y));
    // let color = get_pixel_color(pixel, camera);
    // let ray = get_ray(camera, id.x, id.y);
    let color = get_color(id.x, id.y);

    // textureStore(outputTex, vec2<i32>(id.xy), vec4(vec3(f32(camera.samples_per_pixel), 0.0, 0.0), 1.0));
    // textureStore(outputTex, vec2<i32>(id.xy), vec4(camera.center, 1.0));
    textureStore(outputTex, vec2<i32>(id.xy), color);
}


