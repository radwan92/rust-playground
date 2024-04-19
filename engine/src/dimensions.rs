#[derive(Debug)]
pub struct Dimensions {
    point_size: u32,
    width: u32,
    height: u32,
}

pub fn point_at(dimensions: &Dimensions, x: i32, y: i32) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(
        x * dimensions.point_size() as i32,
        y * dimensions.point_size() as i32,
        dimensions.point_size(),
        dimensions.point_size(),
    )
}

pub fn rect_at(dimensions: &Dimensions, x: i32, y: i32, width: u32, height: u32) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(
        x * dimensions.point_size() as i32,
        y * dimensions.point_size() as i32,
        width * dimensions.point_size(),
        height * dimensions.point_size(),
    )
}

impl Dimensions {
    pub fn new(point_size: u32, width: u32, height: u32) -> Dimensions {
        Dimensions { width, height, point_size }
    }

    pub fn default() -> Dimensions {
        Dimensions::new(1, 800, 600)
    }

    /// Size of a point in pixels
    pub fn point_size(&self) -> u32 {
        self.point_size
    }

    /// Width in points
    pub fn point_width(&self) -> u32 {
        self.width
    }

    /// Height in points
    pub fn point_height(&self) -> u32 {
        self.height
    }

    /// Width in pixels
    pub fn pixel_width(&self) -> u32 {
        self.width * self.point_size
    }

    /// Height in pixels
    pub fn pixel_height(&self) -> u32 {
        self.height * self.point_size
    }
}
