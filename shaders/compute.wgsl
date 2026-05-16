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

const SPHERES_COUNT = 1;

const SPHERES = array<Sphere, SPHERES_COUNT>(
    Sphere( // Ground
        vec3(0.0, 0.0, 10.0),
        3.0,
        vec3(1.0, 0.0, 0.0)),
    // Sphere(
    //     vec3(0.0, 1.0, 0.0),
    //     0.2,
    //     vec3(0.0, 1.0, 0.0),
    // ),
    // Sphere(
    //     vec3(-4.0, 1.0, 0.0),
    //     1.0,
    //     vec3(0.0, 0.0, 1.0),
    // )
);

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
}

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

fn hit_sphere(ray: Ray) -> vec2<i32> {
    for (var i = 0; i < SPHERES_COUNT; i++) {
        let sphere: Sphere = SPHERES[i];

        let oc = sphere.center - ray.origin;

        let a = dot(ray.dir, ray.dir);
        let b = -2.0 * dot(ray.dir, oc);
        let c = dot(oc, oc) - sphere.radius * sphere.radius;

        let discr = b * b - 4.0 * a * c;

        if discr >= 0.0 {
            return vec2(1, i);
        }
    }

    return vec2(0);
}

fn get_color(ray: Ray) -> vec4<f32> {
    let result = hit_sphere(ray);

    if result.x > 0 {
        return vec4(SPHERES[result.y].color, 1.0);
    }

    return vec4(SKY_COLOR, 1.0);
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
    let aspect_ratio = f32(workgroup_size.x) / f32(workgroup_size.y);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height / aspect_ratio;
    let camera_center = vec3(0.0);

    let viewport_u = vec3(viewport_width, 0.0, 0.0);
    let viewport_v = vec3(0.0, -viewport_width, 0.0);

    let pixel_delta_u = viewport_u / f32(workgroup_size.x);
    let pixel_delta_v = viewport_v / f32(workgroup_size.y);

    let viewport_upper_left = camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // let uv = vec2(f32(invocation_id.x) / f32(workgroup_size.x), 1.0 - f32(invocation_id.y) / f32(workgroup_size.y));
    let pixel_center = pixel00_loc + f32(invocation_id.x) * pixel_delta_u + f32(invocation_id.y) * pixel_delta_v;
    let ray_direction = pixel_center - camera_center;
    let color = get_color(Ray(camera_center, ray_direction));

    textureStore(outputTex, vec2<i32>(invocation_id.xy), color);
}


