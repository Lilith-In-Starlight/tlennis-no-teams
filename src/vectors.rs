#[derive(Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self{
        Self {
            x: x,
            y: y,
        }
    }

    pub fn dist_to(&self, b: Vec2) -> f32 {
        let xdist = b.x - self.x;
        let ydist = b.y - self.y;
        f32::sqrt(xdist*xdist + ydist*ydist)
    }

    pub fn dist_to3(&self, b: Vec3) -> f32 {
        let xdist = b.x - self.x;
        let ydist = b.y - self.y;
        f32::sqrt(xdist*xdist + ydist*ydist + b.z*b.z)
    }
}


impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn dist_to(&self, b: Vec3) -> f32 {
        let xdist = b.x - self.x;
        let ydist = b.y - self.y;
        let zdist = b.z - self.z;
        f32::sqrt(xdist*xdist + ydist*ydist + zdist*zdist)
    }

    pub fn dist_to2(&self, b: Vec2) -> f32 {
        let xdist = b.x - self.x;
        let ydist = b.y - self.y;
        f32::sqrt(xdist*xdist + ydist*ydist + self.z*self.z)
    }
}
