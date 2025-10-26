use std::rc::Rc;

use crate::{
    math::Vec3,
    util::{Color, Interval, random_float},
};

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub const fn origin(&self) -> &Vec3 {
        &self.origin
    }
    pub const fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, scalar: f64) -> Vec3 {
        self.origin.clone() + self.dir.mul(scalar)
    }
}

pub struct HitRectord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRectord {
    fn build(ray: &Ray, t: f64, point: Vec3, normal: Vec3, material: Rc<dyn Material>) -> Self {
        let front_face = ray.dir().dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRectord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: impl Material + 'static) -> Self {
        Self {
            center,
            radius,
            material: Rc::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRectord> {
        let oc = self.center.clone() - ray.origin().clone();

        let a = ray.dir().length_squared();
        let h = ray.dir().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discr = h * h - a * c;

        if discr < 0.0 {
            return None;
        }

        let sqrt_discr = f64::sqrt(discr);

        let mut root = (h - sqrt_discr) / a;

        if !interval.surrounds(root) {
            root = (h + sqrt_discr) / a;

            if !interval.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point.clone() - self.center.clone()).div(self.radius);
        Some(HitRectord::build(
            &ray,
            root,
            point,
            normal,
            self.material.clone(),
        ))
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Box::new(hittable));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRectord> {
        let mut closest = interval.max;
        let mut closest_record = None;

        self.objects.iter().for_each(|hittable| {
            if let Some(record) = hittable.hit(ray, Interval::new(interval.min, closest)) {
                closest = record.t;
                closest_record = Some(record);
            }
        });

        closest_record
    }
}

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRectord,
        attenuation: &Color,
    ) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRectord,
        attenuation: &Color,
    ) -> Option<(Ray, Color)> {
        let mut scatter = record.normal.clone() + Vec3::random_unit();
        if scatter.near_zero() {
            scatter = record.normal.clone();
        }
        let scattered = Ray::new(record.point.clone(), scatter);
        Some((scattered, self.albedo.clone()))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRectord,
        attenuation: &Color,
    ) -> Option<(Ray, Color)> {
        let reflected = ray_in.dir().reflect(&record.normal);
        let reflected = reflected.unit() + Vec3::random_unit().mul(self.fuzz);
        let scattered = Ray::new(record.point.clone(), reflected);

        if scattered.dir().dot(&record.normal) > 0.0 {
            Some((scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_factor: f64,
}

impl Dielectric {
    pub fn new(refraction_factor: f64) -> Self {
        Self { refraction_factor }
    }

    fn reflectance(cosine: f64, refraction_factor: f64) -> f64 {
        let r0 = (1.0 - refraction_factor) / (1.0 + refraction_factor);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRectord,
        attenuation: &Color,
    ) -> Option<(Ray, Color)> {
        let ri = if record.front_face {
            1.0 / self.refraction_factor
        } else {
            self.refraction_factor
        };

        let unit_dir = ray_in.dir().unit();

        let cos_theta = f64::min((-unit_dir.clone()).dot(&record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannnot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannnot_refract || Dielectric::reflectance(cos_theta, ri) > random_float() {
                unit_dir.reflect(&record.normal)
            } else {
                unit_dir.refract(&record.normal, ri)
            };

        Some((
            Ray::new(record.point.clone(), direction),
            Color::new(1.0, 1.0, 1.0),
        ))
    }
}
