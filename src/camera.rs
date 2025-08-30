use crate::hittable::{Hittable, HittableList};
use crate::image::{Color, Image};
use crate::interval::Interval;
use crate::math::{Point3, Vec3, lerp};
use crate::random::normal_random;
use crate::ray::Ray;

use std::io;

/// Transform a component from linear to gamma using "gamma 2" transform
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

/// PPM extension functions
pub mod ppm {
    use super::*;

    pub fn header(w: &mut impl io::Write, img: &Image) -> io::Result<()> {
        write!(w, "P3\n{} {}\n255\n", img.width, img.height)?;
        Ok(())
    }

    pub fn write_color(w: &mut impl io::Write, c: &Color) -> io::Result<()> {
        let r = linear_to_gamma(c.x());
        let g = linear_to_gamma(c.y());
        let b = linear_to_gamma(c.z());

        let intensity = Interval::new(0.0, 0.999);

        let rbyte = 255.0 * intensity.clamp(r);
        let gbyte = 255.0 * intensity.clamp(g);
        let bbyte = 255.0 * intensity.clamp(b);

        write!(w, "{} {} {} ", rbyte as i16, gbyte as i16, bbyte as i16,)?;
        Ok(())
    }

    pub fn new_line(w: &mut impl io::Write) -> io::Result<()> {
        writeln!(w, "")?;
        Ok(())
    }
}
pub struct ViewportContext {
    pub delta_u: Vec3,
    pub delta_v: Vec3,
    pub upper_left_pixel: Point3,
}

pub struct Camera {
    pub focal_length: f64,
    pub center: Point3,

    pub viewport_height: f64,
    pub viewport_width: f64,

    pub image: Image,

    /// Count of random samples for each pixel used for antialiasing
    pub sample_per_pixel: i16,

    pub max_recursion_depth: i16,
}

impl Camera {
    pub fn new(img: Image) -> Self {
        Camera {
            focal_length: 0.0,
            viewport_height: 0.0,
            viewport_width: 0.0,
            image: img,
            center: Point3::zero(),
            sample_per_pixel: 100,
            max_recursion_depth: 10,
        }
    }

    pub fn set_viewport_from_height(&mut self, viewport_height: f64) {
        self.viewport_height = viewport_height;
        self.viewport_width = viewport_height * self.image.ideal_ratio();
    }

    pub fn viewport_u(&self) -> Vec3 {
        Vec3::new(self.viewport_width, 0., 0.)
    }

    pub fn viewport_v(&self) -> Vec3 {
        Vec3::new(0., -self.viewport_height, 0.)
    }

    pub fn delta_u(&self) -> Vec3 {
        self.viewport_u() / self.image.width as f64
    }

    pub fn delta_v(&self) -> Vec3 {
        self.viewport_v() / self.image.height as f64
    }

    pub fn upper_left_viewport(&self) -> Vec3 {
        self.center
            - Vec3::new(0., 0., self.focal_length)
            - 0.5 * (self.viewport_u() + self.viewport_v())
    }

    pub fn upper_left_pixel(&self) -> Vec3 {
        self.upper_left_viewport() + 0.5 * (self.delta_u() + self.delta_v())
    }

    pub fn viewport_context(&self) -> ViewportContext {
        ViewportContext {
            upper_left_pixel: self.upper_left_pixel(),
            delta_u: self.delta_u(),
            delta_v: self.delta_v(),
        }
    }

    pub fn render(&self, target: &mut impl io::Write, world: &HittableList) -> io::Result<()> {
        let viewport_ctx = self.viewport_context();

        ppm::header(target, &self.image)?;
        for v in 0..self.image.height {
            eprint!("\rScanning lines [{}/{}]", v + 1, self.image.height);
            for u in 0..self.image.width {
                let mut color = Color::zero();
                for _ in 0..self.sample_per_pixel {
                    let ray = self.get_ray(u, v, &viewport_ctx);
                    color += Camera::ray_color(&ray, &world, self.max_recursion_depth);
                }
                color = color / self.sample_per_pixel as f64;

                ppm::write_color(target, &color)?;
            }
            ppm::new_line(target)?;
        }
        eprint!("\n");

        Ok(())
    }

    pub fn ray_color(ray: &Ray, world: &HittableList, depth: i16) -> Color {
        if depth == 0 {
            return Color::zero();
        }

        if let Some(rec) = world.hit(ray, Interval::positive()) {
            let direction = rec.normal + Vec3::unit_random_on_sphere();
            return 0.5 * Camera::ray_color(&Ray::new(rec.point, direction), world, depth - 1);
        }

        let unit_direction = ray.direction().normal();
        let blue = Color::new(0.5, 0.7, 1.0);
        let white = Color::new(1.0, 1.0, 1.0);

        let t = 0.5 * (unit_direction.y() + 1.0);

        lerp(&white, &blue, t)
    }

    pub fn get_ray(&self, u: i32, v: i32, viewport_ctx: &ViewportContext) -> Ray {
        let offset = Camera::sample_square();

        let pixel_sample = viewport_ctx.upper_left_pixel
            + ((u as f64 + offset.x()) * viewport_ctx.delta_u
                + (v as f64 + offset.y()) * viewport_ctx.delta_v);

        let ray_origin = self.center;
        let ray_dir = pixel_sample - self.center;

        let ray = Ray::new(ray_origin, ray_dir);

        ray
    }

    /// Returns a random point in the square `[-0.5, 0.5] x [-0.5, 0.5] x {0}`
    pub fn sample_square() -> Vec3 {
        Vec3::new(normal_random() - 0.5, normal_random() - 0.5, 0.)
    }
}
