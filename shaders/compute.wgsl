@group(0) @binding(0) var outputTex : texture_storage_2d<rgba8unorm, write>;
@group(1) @binding(0) var<uniform> util : UtilData;
@group(1) @binding(1)
var random_texture: texture_2d<f32>;
@group(1)@binding(2)
var random_sampler: sampler;

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
    maximum_depth: i32,
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
    let maximum_depth = i32(10);
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
    return textureSampleLevel(random_texture, random_sampler, vec2(x, y), 0.0).x;
}

fn sample_square(x: f32, y: f32) -> vec3<f32> {
    return vec3(random_float(x, y) - 0.5, random_float(y, x) - 0.5, 0.0);
}

fn random_float_range(min: f32, max: f32, x: f32, y: f32) -> f32 {
    // let exclusive_max = max - 0.00001;
    return min + (max - min) - random_float(max, min);
}

fn random_unit_vector()  -> vec3<f32> {
    let MAX_ITER = 100;
    for(var i = 0; i < MAX_ITER; i++) {
        let vec = vec3(
            random_float_range(-1.0, 1.0, f32(i), 0.0), 
            random_float_range(-1.0, 1.0, 0.0, f32(i)), 
            random_float_range(-1.0, 1.0, f32(i), f32(i)), 
        );
        let len = length(vec);
        let lensq = len * len;
        if(lensq >= 1e-160 && lensq <= 1.0) {
            return normalize(vec);
        }
    }

    return vec3(1.0, 0.0, 0.0);
}

fn near_zero(vec: vec3<f32>) -> bool {
    let s = 1e-8;
    return abs(vec.x) < s && abs(vec.y) < s && abs(vec.z) < s;
}

fn defocus_disk_sample(camera: Camera) -> vec3<f32> {
    let MAX_ITER = 100;
    for(var i = 0; i < MAX_ITER; i++) {
        let vec = vec3(
            random_float_range(-1.0, 1.0, f32(i), 0.0), 
            random_float_range(-1.0, 1.0, 0.0, f32(i)), 
            0.0
        );
        let l = length(vec);
        if(l * l <= 1.0) {
            return vec;
        }
    }

    return vec3(1.0, 1.0, 0.0);
}

fn get_ray(camera: Camera, i: u32, j: u32) -> Ray {
    let offset = sample_square(f32(i),f32(j));
    let pixel_sample = camera.image_center + camera.pixel_delta_u * (f32(i) + offset.x) + camera.pixel_delta_u * (f32(i) + offset.x);
    var origin = vec3(0.0, 0.0, 0.0);
    if(camera.defocus_angle <= 0.0) {
        origin = camera.center;
    } else {
        origin = defocus_disk_sample(camera);
    }

    let dir = pixel_sample - origin;
    return Ray(origin, dir);
}

// fn get_pixel_color(pixel: vec2<f32>, camera: Camera) -> vec4<f32> {
//     return textureSampleLevel(random_texture, random_sampler, pixel, 0.0);
// }

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
    var normal = point - sphere.center / sphere.radius;
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

struct ScatterReturn {
    ray: Ray,
    albedo: vec3<f32>
}

struct Lambertian {
    albedo: vec3<f32>
}

fn scatter_lambertian(material: Lambertian, ray_in: Ray, record: HitRecord) -> ScatterReturn {
    var scatter = record.normal + random_unit_vector();
    if(near_zero(scatter)) {
        scatter = record.normal;
    }

    let scattered = Ray(record.point, scatter);
    return ScatterReturn(scattered, material.albedo);
}

struct Metal {
    albedo: vec3<f32>,
    fuzz: f32
}

fn scatter_metal(material: Metal, ray_in: Ray, record: HitRecord) -> ScatterReturn {
    let reflect_dir = reflect(ray_in.dir, record.normal);
    let reflect_dir_fuzz = normalize(reflect_dir) + random_unit_vector() * material.fuzz;
    let scattered = Ray(record.point, reflect_dir_fuzz);
    return ScatterReturn(scattered, material.albedo);
}

struct Dielectric {
    refraction_factor: f32
}

fn scatter_dielectric( material: Dielectric, ray_in: Ray, record: HitRecord) -> ScatterReturn {
    let refracted = refract(ray_in.dir, record.normal, material.refraction_factor);
    let scattered = Ray(record.point, refracted);
    return ScatterReturn(
        scattered,
        vec3(1.0, 1.0, 1.0)
    );
}


const LAMBERTIAN_MATERIALS = array<Lambertian, 2>(
    Lambertian(
        vec3(0.5, 0.5, 0.5)
    ),
    Lambertian(
        vec3(0.4, 0.2, 0.1)
    )
);

const METAL_MATERIALS = array<Metal, 1>(
    Metal(
        vec3(0.7, 0.6, 0.5),
        0.0
    ),
);

const DIELECTRIC_MATERIALS = array<Dielectric, 1>(
    Dielectric(
        1.5
    ),
);

const SPHERES_COUNT = 4;

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

fn scatter_material(hit_record: HitRecord, ray: Ray) -> ScatterReturn {
    let material_type = hit_record.material_type;
    if(material_type == 0) {
        let material = LAMBERTIAN_MATERIALS[hit_record.material_index];
        return scatter_lambertian(material, ray, hit_record);
    } else if(material_type == 1) {
        let material = METAL_MATERIALS[hit_record.material_index];
        return scatter_metal(material, ray, hit_record);
    } else {
        let material = DIELECTRIC_MATERIALS[hit_record.material_index];
        return scatter_dielectric(material, ray, hit_record);
    }
}

fn get_world_color(ray: Ray) -> vec3<f32> {
    let unit_dir = normalize(ray.dir);
    let a = 0.5 * (unit_dir.y + 1.0);
    return vec3(1.0) * (1.0 - a) + vec3(0.5, 0.7, 1.0) * a;
}

const INFINITY = f32(0x1.fffffep+127);
fn get_color(camera: Camera, ray: Ray) -> vec4<f32>{
    var cum_attenuation = vec3(1.0, 1.0, 1.0);
    var current_ray = ray;
    for(var i = 0; i < camera.maximum_depth; i ++) {
        let record = hit_world(current_ray, 0, INFINITY);
        if(!record.hit) {
            cum_attenuation = cum_attenuation * get_world_color(current_ray);
            break;
        } else {
            let ret = scatter_material(record, current_ray);
            cum_attenuation = cum_attenuation * ret.albedo; 
            current_ray = ret.ray;
        }
    }
    return vec4(cum_attenuation, 1.0);
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) id : vec3<u32>, @builtin(num_workgroups) size : vec3<u32>) {
    let camera = build_camera(size.x, size.y);

    // let pixel = vec2(f32(id.x)/f32(size.x), f32(id.y)/f32(size.y));
    // let color = get_pixel_color(pixel, camera);
    let ray = get_ray(camera, id.x, id.y);
    let color = get_color(camera, ray);

    textureStore(outputTex, vec2<i32>(id.xy), color);
}
