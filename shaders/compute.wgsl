@group(0) @binding(0) var outputTex : texture_storage_2d<rgba8unorm, write>;
@group(1) @binding(0) var<uniform> util : UtilData;

struct UtilData {
    time: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

const PI = 3.14159265359;

struct Camera {
    width: u32,
    height: u32,
    center: vec3<f32>,
    image_center: vec3<f32>,
    pixel_delta_u: vec3<f32>,
    pixel_delta_v: vec3<f32>,
    samples_per_pixel: f32,
    pixels_sample_scale: f32,
    maximum_depth: u32,
    defocus_angle: f32,
    defocus_disk_u: vec3<f32>,
    defocus_disk_v: vec3<f32>,
}

fn build_camera(width: u32, height: u32) -> Camera {
    let up_vector = vec3<f32>(0.0, 1.0, 0.0);

    // User inputs, later I can move to shader inputs
    let vfov = 20.0;
    let center = vec3<f32>(13.0, 2.0, 3.0);
    let look_at = vec3<f32>(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let samples_per_pixel = 1.0;
    let maximum_depth = u32(10);
    let defocus_angle = 0.6;

    let theta = vfov / 180.0 * PI;

    let h = tan(theta/2.0);
    let viewport_height = 2.0 * h * focus_dist;
    let viewport_width = viewport_height * (f32(width) / f32(height));

    let w = normalize(center - look_at);
    let u = normalize(cross(up_vector, w));
    let v = cross(w, u);

    let viewport_u = u * viewport_width;
    let viewport_v = -v * viewport_height;

    let pixel_delta_u = viewport_u / f32(width);
    let pixel_delta_v = viewport_v / f32(height);

    let viewport_upper_left =
        center - w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
    let image_center =
        viewport_upper_left + pixel_delta_u + pixel_delta_v * 0.5;

    let defocus_radius = focus_dist * tan(defocus_angle / 2.0 / 180.0 * PI);
    let defocus_disk_u = u * defocus_radius;
    let defocus_disk_v = v * defocus_radius;

    return Camera(
        width,
        height,
        center,
        image_center,
        pixel_delta_u,
        pixel_delta_v,
        samples_per_pixel,
        1.0,
        maximum_depth,
        defocus_angle,
        defocus_disk_u,
        defocus_disk_v
    );
}

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>
}

fn random_float(x: f32, y: f32) -> f32 {
    return 0.2;
}

fn sample_square(x: f32, y: f32) -> vec3<f32> {
    return vec3(random_float(x, y) - 0.5, random_float(y, x) - 0.5, 0.0);
}

fn get_ray(camera: Camera, i: u32, j: u32) -> Ray {
    let offset = sample_square(f32(i),f32(j));
    let pixel_sample = camera.image_center + camera.pixel_delta_u * (f32(i) + offset.x) + camera.pixel_delta_u * (f32(i) + offset.x);
    let origin = vec3(0.0, 0.0, 0.0);
    if(camera.defocus_angle <= 0.0) {
        origin = camera.center;
    } else {
        origin = defocus_disk_sample(cmaera);
    }
    return Ray();
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) id : vec3<u32>, @builtin(num_workgroups) size : vec3<u32>) {
    let camera = build_camera(size.x, size.y);
    if (id.x >= size.x || id.y >= size.y) {
        return;
    }

    let color = vec4<f32>(
        f32(id.x) / f32(size.x) * sin(f32(util.time) / 1000.0),
        f32(id.y) / f32(size.y) * cos(f32(util.time) / 1000.0),
        0.3,
        1.0
    );

    textureStore(outputTex, vec2<i32>(id.xy), color);
}
