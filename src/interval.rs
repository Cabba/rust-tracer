const INF: f64 = f64::MAX;

#[derive(Clone, Copy)] // Copy since < 64 bytes
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: INF,
            max: -INF,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: -INF,
            max: INF,
        }
    }

    pub fn positive() -> Self {
        Self { min: 0., max: INF }
    }

    pub fn negative() -> Self {
        Self { min: -INF, max: 0. }
    }

    pub fn clamp(&self, v: f64) -> f64 {
        if v < self.min {
            return self.min;
        }
        if v > self.max {
            return self.max;
        }

        v
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, v: f64) -> bool {
        self.min <= v && v <= self.max
    }

    pub fn surrounds(&self, v: f64) -> bool {
        self.min < v && v < self.max
    }
}
