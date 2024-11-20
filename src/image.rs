use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Pixel { r: 0, g: 0, b: 0 }; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x < self.width && y < self.height {
            Some(&self.data[x + y * self.width])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x < self.width && y < self.height {
            Some(&mut self.data[x + y * self.width])
        } else {
            None
        }
    }

    pub fn get_mandelbrot_pixels(&self) -> usize {
        self.data
            .iter()
            .filter(|&p| *p == Pixel { r: 0, g: 0, b: 0 })
            .count()
    }
}