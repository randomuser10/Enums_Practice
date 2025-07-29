pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

pub struct Circle {
    radius: f64,
}

#[derive(Debug, PartialEq)]

pub enum Error {
    InvalidWidth,
    InvalidHeight,
    InvalidRadius,
}

impl Rectangle {
    pub fn new(_width: f64, height: f64) -> Result<(), Error> {}
    // pub fn set()
}

impl Circle {
    pub fn set_radius(&mut self, _radius: f64) -> Result((), Error) {
        if _radius <= 0.0 {
            return Err(Error::InvalidRadius);
        }

        Ok(self.radius = get_radius())
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}
