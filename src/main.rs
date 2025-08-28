mod camera;
mod hittable;
mod image;
mod interval;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::HittableList;
use image::Image;
use sphere::Sphere;
use vec3::Vec3;

// //////////////////////////////////////////////////////
// Entry point
// //////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = Image::from_aspect_ratio(1000, 16.0 / 9.0)?;

    let mut camera = Camera::new(img);

    camera.focal_length = 1.0;
    camera.set_viewport_from_height(2.0);

    let mut world = HittableList::new();
    world.add(Sphere::from_center_radius(0., 0., -1., 0.5));
    world.add(Sphere::from_center_radius(0., -100.5, -1., 100.0));

    camera.render(&mut std::io::stdout(), &world)?;

    eprintln!("finished");

    Ok(())
}
