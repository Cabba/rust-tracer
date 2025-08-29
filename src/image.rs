use crate::math::Vec3;

/// Computes the ideal ratio (without any rounding) between width and height.
/// The formula is
/// $$
/// aspect_ratio = width / height
/// $$
pub fn ideal_ratio(width: i32, height: i32) -> f64 {
    width as f64 / height as f64
}

/// Computes the aspect ratio (closes mathing integer ratio) between width and height.
pub fn aspect_ratio(width: i32, height: i32) -> i32 {
    ideal_ratio(width, height) as i32
}

#[derive(Copy, Clone)]
pub struct Image {
    pub width: i32,
    pub height: i32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Result<Self, String> {
        let img = Image { width, height };
        if img.is_valid() {
            Ok(img)
        } else {
            Err("Image is not valid".to_string())
        }
    }

    pub fn from_aspect_ratio(width: i32, aspect_ratio: f32) -> Result<Self, String> {
        return Image::new(width, (width as f32 / aspect_ratio) as i32);
    }

    pub fn ideal_ratio(&self) -> f64 {
        ideal_ratio(self.width, self.height)
    }

    pub fn aspect_ratio(&self) -> i32 {
        aspect_ratio(self.width, self.height)
    }

    /// Checks if the generated image is ok. Minimum width and height, etc ...
    pub fn is_valid(&self) -> bool {
        if self.height < 1 || self.width < 1 {
            return false;
        }
        return true;
    }
}

#[test]
fn image_constructors() {
    let i1 = Image::new(800, 600).unwrap();
    assert_eq!(i1.width, 800);
    assert_eq!(i1.height, 600);

    let i2 = Image::from_aspect_ratio(800, 2.0).unwrap();
    assert_eq!(i2.width, 800);
    assert_eq!(i2.height, 400);
}

pub type Color = Vec3;
