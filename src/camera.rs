use crate::hittable::{Hittable, HittableList};
use crate::image::{Color, Image};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, lerp};

use std::io;

/// PPM extension functions
pub mod ppm {
    use super::*;

    pub fn header(w: &mut impl io::Write, img: &Image) -> io::Result<()> {
        write!(w, "P3\n{} {}\n255\n", img.width, img.height)?;
        Ok(())
    }

    pub fn write_color(w: &mut impl io::Write, c: &Color) -> io::Result<()> {
        write!(
            w,
            "{} {} {} ",
            (255.0 * c[0]) as i16,
            (255.0 * c[1]) as i16,
            (255.0 * c[2]) as i16,
        )?;
        Ok(())
    }

    pub fn new_line(w: &mut impl io::Write) -> io::Result<()> {
        writeln!(w, "")?;
        Ok(())
    }
}

pub struct Camera {
    pub focal_length: f64,
    pub center: Point3,

    pub viewport_height: f64,
    pub viewport_width: f64,

    pub image: Image,
}

impl Camera {
    pub fn new(img: Image) -> Self {
        Camera {
            focal_length: 0.0,
            viewport_height: 0.0,
            viewport_width: 0.0,
            image: img,
            center: Point3::zero(),
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

    pub fn render(&self, target: &mut impl io::Write, world: &HittableList) -> io::Result<()> {
        ppm::header(target, &self.image)?;

        let px00_loc = self.upper_left_pixel();
        let delta_u = self.delta_u();
        let delta_v = self.delta_v();

        for v in 0..self.image.height {
            for u in 0..self.image.width {
                // Compute ray
                let pixel_center = px00_loc + (u as f64 * delta_u + v as f64 * delta_v);
                let dir = pixel_center - self.center;
                let ray = Ray::new(self.center, dir);

                let color = Camera::ray_color(&ray, &world);

                ppm::write_color(target, &color)?;
            }
            ppm::new_line(target)?;
        }

        Ok(())
    }

    pub fn ray_color(ray: &Ray, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(ray, Interval::positive()) {
            return 0.5 * (rec.normal + Vec3::unit());
        }

        let unit_direction = ray.direction().normal();
        let blue = Color::new(0.5, 0.7, 1.0);
        let white = Color::new(1.0, 1.0, 1.0);

        let t = 0.5 * (unit_direction.y() + 1.0);

        lerp(&white, &blue, t)
    }
}
