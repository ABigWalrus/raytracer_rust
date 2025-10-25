use crate::{math::Vec3, util::Interval};

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
}

impl HitRectord {
    fn build(ray: &Ray, t: f64, point: Vec3, normal: Vec3) -> Self {
        let front_face = ray.dir().dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Self {
            point,
            normal,
            t,
            front_face,
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
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
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
        Some(HitRectord::build(&ray, root, point, normal))
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
