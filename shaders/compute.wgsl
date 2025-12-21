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

var<workgroup> RANDOM_COUNTER_X: u32;
var<workgroup> RANDOM_COUNTER_Y: u32;
const RANDOM_IMAGE_WIDTH = 64;
const RANDOM_IMAGE_HEIGHT = 64;

fn random_float() -> f32 {
    RANDOM_COUNTER_X++;
    RANDOM_COUNTER_Y++;

    if(RANDOM_COUNTER_X > RANDOM_IMAGE_WIDTH) {
        RANDOM_COUNTER_X = 1;
    }

    if(RANDOM_COUNTER_Y > RANDOM_IMAGE_HEIGHT) {
        RANDOM_COUNTER_Y = 1;
    }
    return textureLoad(random_texture, vec2<u32>(RANDOM_COUNTER_X, RANDOM_COUNTER_Y), 0).x;
}

fn random_range_float(min: f32, max: f32) -> f32 {
    return random_float() * (max - min) + min;
}

fn random_vec3() -> vec3<f32> {
    RANDOM_COUNTER_X++;
    RANDOM_COUNTER_Y++;

    if(RANDOM_COUNTER_X > RANDOM_IMAGE_WIDTH) {
        RANDOM_COUNTER_X = 1;
    }

    if(RANDOM_COUNTER_Y > RANDOM_IMAGE_HEIGHT) {
        RANDOM_COUNTER_Y = 1;
    }
    return textureLoad(random_texture, vec2<u32>(RANDOM_COUNTER_X, RANDOM_COUNTER_Y), 0).xyz;
    // return vec3(random_float(), random_float(), random_float());
}

fn random_range_vec3(min: f32, max: f32) -> vec3<f32> {
    return random_vec3() * (max - min) + min;
}

const MAX_RANDOM_UNIT_VEC = 10;

fn random_unit_vec3() -> vec3<f32> {
    for (var i = 0; i < MAX_RANDOM_UNIT_VEC; i++) {
        let p = random_range_vec3(-1.0, 1.0);
        let len = length(p);
        if (1e-160 < len && len <= 1.0) {
            return normalize(p);
        }
    }
    return vec3(1.0, 0.0, 0.0);
    // return normalize(random_range_vec3(-1.0, 1.0));
}

fn random_on_hemisphere(normal: vec3<f32>) -> vec3<f32> {
    let rand = normalize(random_range_vec3(-1.0, 1.0));
    if (dot(rand, normal) > 0.0) {
        return rand;
    } else {
        return -rand;
    }
}


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

struct HitRecord {
    hit: bool,
    point: vec3<f32>,
    normal: vec3<f32>,
    t: f32,
    front_face: bool,
    material_type: u32,
    material_index: u32,
}

fn no_hit() -> HitRecord {
    return HitRecord(
        false,
        vec3(),
        vec3(),
        0,
        false,
        0,
        0
    );
}

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    material_type: u32,
    material_index: u32
}

fn hit_sphere(sphere: Sphere, ray: Ray, range_min: f32, range_max: f32) -> HitRecord{
    let oc = sphere.center - ray.origin;
    let dir_length = length(ray.dir);
    let oc_length = length(oc);

    let a = dir_length * dir_length;
    let h = dot(ray.dir, oc);
    let c = oc_length * oc_length - sphere.radius * sphere.radius;

    let discr = h * h - a * c;
    if(discr < 0.0) {
        return no_hit();
    }

    let sqrt_discr = sqrt(discr);
    var root = (h - sqrt_discr) / a;

    if(root < range_min || root >= range_max) {
        root =  (h + sqrt_discr) / a;

        if(root < range_min || root >= range_max) {
            return no_hit();
        }
    }

    let point = ray.origin + ray.dir * root;
    var normal = (point - sphere.center) / sphere.radius;

    let front_face = dot(ray.dir, normal) < 0.0;
    if(!front_face) {
        normal = -normal;
    }
    return HitRecord(
        true,
        point,
        normal,
        root,
        front_face,
        sphere.material_type,
        sphere.material_index,
    );
}

const SPHERES = array<Sphere, 4>(
    Sphere( // Ground
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        0,  // Lambertian
        0,  // First
    ),
    Sphere(
        vec3(0.0, 1.0, 0.0),
        1.0,
        2,  // Dielectric
        0,  // First
    ),
    Sphere(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        0,  // Lambertian
        1,  // Second
    ),
    Sphere(
        vec3(4.0, 1.0, 0.0),
        1.0,
        1,  // Metal,
        0,  // First
    ),
);


const SPHERES_COUNT = 2;

fn hit_world(ray: Ray, min: f32, max: f32) -> HitRecord {
    var closest = max;
    var closest_record = no_hit();
    for(var i = 0; i < SPHERES_COUNT; i++) {
        let sphere = SPHERES[i];
        let hit_record = hit_sphere(sphere, ray, min, closest);
        if(hit_record.hit) {
            closest = hit_record.t;
            closest_record = hit_record;
        }
    }
    return closest_record;
}


// fn hit_sphere(center: vec3<f32>, radius: f32, ray: Ray) -> f32 {
//     let oc = center - ray.origin;

//     let dir_l = length(ray.dir);
//     let a = dir_l * dir_l;
//     let h = dot(ray.dir, oc);

//     let oc_l = length(oc);
//     let c = oc_l * oc_l - radius * radius;

//     let discr = h * h - a * c;

//     if (discr < 0) {
//         return -1.0;
//     } else {
//         return (h - sqrt(discr)) / a;
//     }
// }

fn sky_color() -> vec3<f32> {
    return vec3(0.5, 0.7, 1.0);
}

fn get_color(x: u32, y: u32) -> vec3<f32> {
    var current_ray = get_ray(x, y);
    var hit = false;
    var color = vec3(1.0, 1.0, 1.0);

    for(var i = 0; i < 10; i++) {
        let record = hit_world(current_ray, 0, 1000);
        if (record.hit) {
            let scatter = record.normal;
            current_ray = Ray(record.point, scatter);
            color *= vec3(0.5, 0.0, 0.0);
            hit = true;
            // return vec3(1.0, 0.0, 0.0);
        } else {
            // color *= sky_color();
            break;
        }
    }
    // let sphere_center = vec3(0.0, 0.0, 0.0);
    // let t = hit_sphere(sphere_center, 0.5, ray);
    // if (t > 0.0) {
    //     let normal = normalize(ray_at(ray, t) - sphere_center);
    //     // return vec4(0.5 * vec3(normal.x + 1, normal.y + 1, normal.z + 1), 1.0);
    //     return vec4(random_on_hemisphere(normal), 1.0);
    // }
    if (!hit) {
        return sky_color();
    } else {
        return color;
    }
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) id : vec3<u32>, @builtin(num_workgroups) size : vec3<u32>) {
    // let camera = build_camera(size.x, size.y);

    // let pixel = vec2(f32(id.x)/f32(size.x), f32(id.y)/f32(size.y));
    // let color = get_pixel_color(pixel, camera);
    // let ray = get_ray(camera, id.x, id.y);
    let color = get_color(id.x, id.y);
    // let color = vec4(random_unit_vec3(), 1.0);

    // textureStore(outputTex, vec2<i32>(id.xy), vec4(vec3(f32(camera.samples_per_pixel), 0.0, 0.0), 1.0));
    // textureStore(outputTex, vec2<i32>(id.xy), vec4(camera.center, 1.0));
    textureStore(outputTex, vec2<i32>(id.xy), vec4(color, 1.0));
}


