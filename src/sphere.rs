use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::vec3::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: f64::max(radius, 0.0),
        }
    }

    pub fn from_center_radius(x: f64, y: f64, z: f64, radius: f64) -> Self {
        Self::new(Point3::new(x, y, z), radius)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, bounds: Interval) -> Option<HitRecord> {
        let d = *ray.direction();
        let c_q = self.center - *ray.origin(); // (C-Q)

        let a = d.length2();
        let h = d.dot(&c_q); // d * (C - Q)
        let c = c_q.length2() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Check if root in range [t_min, t_max]
        let mut root = (h - sqrtd) / a;
        if !bounds.contains(root) {
            root = (h + sqrtd) / a;
            if !bounds.contains(root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;

        let rec = HitRecord::new(hit_point, outward_normal, root, ray);

        Some(rec)
    }
}
