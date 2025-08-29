mod camera;
mod hittable;
mod image;
mod interval;
mod math;
mod ray;
mod sphere;

use camera::Camera;
use hittable::HittableList;
use image::Image;
use sphere::Sphere;

// //////////////////////////////////////////////////////
// Entry point
// //////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = Image::from_aspect_ratio(400, 16.0 / 9.0)?;

    let mut camera = Camera::new(img);

    camera.focal_length = 1.0;
    camera.set_viewport_from_height(2.0);
    camera.sample_per_pixel = 100;

    let mut world = HittableList::new();
    world.add(Sphere::from_center_radius(0., 0., -1., 0.5));
    world.add(Sphere::from_center_radius(0., -100.5, -1., 100.0));

    camera.render(&mut std::io::stdout(), &world)?;

    eprintln!("finished");

    Ok(())
}
