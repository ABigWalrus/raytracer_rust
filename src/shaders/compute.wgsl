

@group(0) @binding(0) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> camera: Camera;
struct Camera {
    first_pixel_pos: vec3<f32>,
    pixel_delta_u: vec3<f32>,
    pixel_delta_v: vec3<f32>,
    position: vec3<f32>,
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

fn random_vec3(p: vec2<f32>) -> vec3<f32> {
    var x = bitcast<u32>(p.x);
    var y = bitcast<u32>(p.y);

    // PCG Hash steps
    var state = x * 747796405u + 2891336453u;
    var word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    let rx = f32((word >> 22u) ^ word) / 4294967295.0;

    state = y * 747796405u + 2891336453u;
    word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    let ry = f32((word >> 22u) ^ word) / 4294967295.0;

    let rz = fract(sin(dot(p, vec2(12.9898, 78.233))) * 43758.5453);

    // Remap to [-1.0, 1.0] and normalize
    return normalize(vec3(rx, ry, rz) * 2.0 - 1.0);
}

fn next_random(state: ptr<function, u32>) -> f32 {
    *state = *state * 1664525u + 1013904223u;
    return f32(*state) / 4294967296.0;
}

// Generates a perfectly uniform random unit vector on a sphere surface (Archimedes' Method)
fn random_unit_vec3(rng: ptr<function, u32>) -> vec3<f32> {
    let r1 = next_random(rng);
    let r2 = next_random(rng);

    let phi = r1 * 6.283185307179586; // 2 * PI
    let z = r2 * 2.0 - 1.0;           // Height distribution from [-1, 1]
    let r_xy = sqrt(max(0.0, 1.0 - z * z));

    return vec3<f32>(r_xy * cos(phi), r_xy * sin(phi), z);
}

fn random_on_hemisphere(normal: vec3<f32>, uv: vec2<f32>) -> vec3<f32> {
    let random = random_vec3(uv);
    if dot(random, normal) > 0.0 {
        return random;
    } else {
        return -random;
    }
}

fn vec2_to_uv(vec: vec2<f32>) -> vec2<f32> {
    return normalize(vec) / 2.0 + 0.5;
}

const SKY_COLOR = vec3(0.5, 0.7, 1.0);

const SPHERES_COUNT = 5;

const SPHERES = array<Sphere, 5>(
    Sphere(
        vec3(0.0, -100.5, -1.0),
        100,
        vec2(0,
            0),
    ),
    Sphere(
        vec3(0.0, 0.0, -1.2),
        0.5,
        vec2(0,
            1)
    ),
    Sphere(
        vec3(-1.0, 0.0, -1.0),
        0.5,
        vec2(2,
            0)
    ),
    Sphere(
        vec3(-1.0, 0.0, -1.0),
        0.4,
        vec2(2,
            1)
    ),
    Sphere(
        vec3(1.0, 0.0, -1.0),
        0.5,
        vec2(1,
            1)
    ),

    // Sphere(
    //     vec3(-4.0, 1.0, 0.0),
    //     1.0,
    //     vec3(0.0, 0.0, 1.0),
    // )
);

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    material: vec2<u32>, // represents: vec2(material type, material index)
}

struct DiffuseMaterial {
    alpha: vec3<f32>}

const DIFFUSE_MATERIALS = array<DiffuseMaterial, 2>(
    DiffuseMaterial(
        vec3(0.8, 0.8, 0.0)
    ),
    DiffuseMaterial(
        vec3(0.1, 0.2, 0.5)
    )
);

struct MetallicMaterial {
    alpha: vec3<f32>,
    fuzz: f32,
}

const METALLIC_MATERIALS = array<MetallicMaterial, 2>(
    MetallicMaterial(
        vec3(0.8, 0.8, 0.8), 0.0,
    ),
    MetallicMaterial(
        vec3(0.8, 0.6, 0.2), 0.0,
    )
);

struct DielectricMaterial {
    refraction_index: f32,
}

const DIELECTRIC_MATERIALS = array<DielectricMaterial, 2>(
    DielectricMaterial(
        1.5,
    ),
    DielectricMaterial(
        1.0 / 1.5,
    ),
);

struct HitResult {
    hit: bool,
    normal: vec3<f32>,
    collision: vec3<f32>,
    material: vec2<u32>,
    front_face: bool,
}

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>,
}

fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.dir;
}

fn sky_color(dir: vec3<f32>) -> vec3<f32> {
    let a = 0.5 * (normalize(dir).y + 1.0);
    return (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0);
}

fn hit_sphere(ray: Ray) -> HitResult {
    var closest_t = 9999.9;
    var closest_i = 0;
    for (var i = 0; i < SPHERES_COUNT; i++) {
        let sphere: Sphere = SPHERES[i];

        let oc = ray.origin - sphere.center;
        let a = dot(ray.dir, ray.dir);
        let h = dot(oc, ray.dir);
        let c = dot(oc, oc) - sphere.radius * sphere.radius;

        let discriminant = h * h - a * c;

        if discriminant >= 0.0 {
            let sqrtd = sqrt(discriminant);
            var t = (-h - sqrtd) / a;

            if t < 0.001 || t > closest_t {
                t = (-h + sqrtd) / a;
            }

            if t >= 0.001 && t < closest_t {
                closest_t = t;
                closest_i = i;
            }
        }
    }

    if closest_t < 9999.9 {
        let sphere = SPHERES[closest_i];
        let collision = ray_at(ray, closest_t);

        var normal = (collision - sphere.center) / sphere.radius;
        let front_face = dot(ray.dir, normal) < 0.0;
        if !front_face {
            normal *= -1.0;
        }

        return HitResult(true, normal, collision, sphere.material, front_face);
    }

    return HitResult(false, vec3(0.0), vec3(0.0), vec2(0), false);
}

fn reflectance(cos_theta: f32, refraction_index: f32) -> f32 {
    let r0 = (1 - refraction_index) / (1 + refraction_index);
    let r00 = r0 * r0;
    return r00 + (1.0 - r00) * pow((1.0 - cos_theta), 5.0);
}

fn get_color(ray: Ray, workgroup_id: vec2<u32>, sample_id: u32) -> vec3<f32> {
    var current_ray = ray;
    let max_bounce = 10;
    var bounce = 0;
    var attenuation = vec3(1.0, 1.0, 1.0);
    let gamma = 0.2;
    var rng_state = workgroup_id.x * 3128u + workgroup_id.y * 9213u + sample_id * 984711u;
    while bounce <= max_bounce {
        let result = hit_sphere(current_ray);
        if result.hit {
            if result.material.x == 0 {
                // diffuse material

                let material = DIFFUSE_MATERIALS[result.material.y];

                // let random_rng_state = invocation_id * vec2<f32>(f32(bounce) * 7.13, f32(bounce) * 4.18);
                // let dir = normalize(result.normal + random_vec3(random_rng_state));
                let dir = normalize(result.normal + random_unit_vec3(&rng_state));

                let epsilon = 0.001;
                current_ray = Ray(result.collision + epsilon * result.normal, dir);

                attenuation *= material.alpha;
                bounce++;
            } else if result.material.x == 1 {
                // metallic material
                let material = METALLIC_MATERIALS[result.material.y];

                let reflected = reflect(current_ray.dir, result.normal);
                let dir = normalize(reflected) + material.fuzz * random_unit_vec3(&rng_state);

                // let epsilon = 0.001;
                current_ray = Ray(result.collision, dir);

                attenuation *= material.alpha;
                bounce++;
            } else if result.material.x == 2 {
                // dielectric material
                let material = DIELECTRIC_MATERIALS[result.material.y];

                var refraction_index = 0.0;
                if result.front_face {
                    refraction_index = 1.0 / material.refraction_index;
                } else {
                    refraction_index = material.refraction_index;
                }

                let current_dir = normalize(current_ray.dir);

                let cos_theta = min(dot(-current_dir, result.normal), 1.0);
                let sin_theta = sqrt(1.0 - pow(cos_theta, 2.0));

                var dir = vec3(0.0);
                let random = next_random(& rng_state);
                if refraction_index * sin_theta > 1.0 || reflectance(cos_theta, refraction_index) > random {
                    dir = reflect(current_dir, result.normal);
                } else {
                    dir = refract(current_dir, result.normal, refraction_index);
                }

                current_ray = Ray(result.collision, dir);

                bounce++;
            } else {
                break;
            }
        } else {
            attenuation *= sky_color(current_ray.dir);
            break;
        }
    }
    if bounce == 0 {
        attenuation *= sky_color(current_ray.dir);
    }

    return attenuation;
}

const WORKGROUP_WIDTH = 6;
const WORKGROUP_HEIGHT = 6;
const WORKGROUP_DEPTH = 1;

const SAMPLE_SIZE = WORKGROUP_WIDTH * WORKGROUP_HEIGHT * WORKGROUP_DEPTH; 

// 36 samples
var<workgroup> workgroupColors: array<vec3<f32>, SAMPLE_SIZE>; 

@compute @workgroup_size(WORKGROUP_WIDTH, WORKGROUP_HEIGHT, WORKGROUP_DEPTH)
fn main(
    @builtin(local_invocation_index) local_invocation_index: u32,
    @builtin(workgroup_id) workgroup_id: vec3<u32>,
) {
    let texture_dimensions = textureDimensions(outputTex);

    let aspect_ratio = f32(texture_dimensions.x) / f32(texture_dimensions.y);

    var rng_state = local_invocation_index * 19347u;

    let random_sample = random_unit_vec3(& rng_state).xy * 0.5;

    let pixel_center = camera.first_pixel_pos + (f32(workgroup_id.x) + random_sample.x) * camera.pixel_delta_u + (f32(workgroup_id.y) + random_sample.y) * camera.pixel_delta_v;

    let ray_direction = normalize(pixel_center - camera.position);
    workgroupColors[local_invocation_index] = get_color(Ray(camera.position, ray_direction), workgroup_id.xy, local_invocation_index);

    workgroupBarrier();

    if local_invocation_index == 0 {
        var aggregated_color = vec3(0.0);
        for (var i = 0; i < SAMPLE_SIZE; i++) {
            aggregated_color += workgroupColors[i];
        }

        textureStore(outputTex, vec2<i32>(workgroup_id.xy), vec4(aggregated_color / f32(SAMPLE_SIZE), 1.0));
    }
}
