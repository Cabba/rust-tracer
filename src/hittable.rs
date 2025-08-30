use crate::interval::Interval;
use crate::math::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug)]
pub struct HitRecord {
    /// Point hitted by the ray
    pub point: Point3,
    /// Normal of the surface hitted by the ray
    pub normal: Vec3,
    /// Parametrization of the surface hitted by the ray
    pub t: f64,

    /// This will be computed calling set_face_normal
    pub front_face: Option<bool>,
}

impl HitRecord {
    pub fn new(point: Point3, outward_normal: Vec3, t: f64, ray: &Ray) -> Self {
        let mut rec = HitRecord {
            point,
            normal: outward_normal,
            t,
            front_face: None,
        };
        rec.set_face_normal(ray, &outward_normal);
        return rec;
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        let ff = ray.direction().dot(outward_normal) < 0.0;
        self.front_face = Some(ff);
        self.normal = if ff {
            *outward_normal
        } else {
            -(*outward_normal)
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, bounds: Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        return Self {
            objects: Vec::new(),
        };
    }

    pub fn add<T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, bound: Interval) -> Option<HitRecord> {
        let mut ret: Option<HitRecord> = None;
        let mut closest = bound.max;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(ray, bound) {
                if closest > rec.t {
                    closest = rec.t;
                    ret = Some(rec);
                }
            }
        }

        ret
    }
}
