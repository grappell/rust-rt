use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }


    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - (*n * Vec3::dot(v, n) * 2.0)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, ni_over_nt: f32) -> Vec3 {
        let cos_theta = Vec3::dot(&(*uv * -1.0), n).min(1.0);
        let r_out_prep = (*uv + (*n * cos_theta)) * ni_over_nt;
        let r_out_parallel = *n * (1.0 - r_out_prep.length_squared()).abs().sqrt();
        r_out_prep + r_out_parallel
    }

}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, value: f32) -> Vec3 {
        Vec3::new(self.x - value, self.y - value, self.z - value)
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, value: f32) -> Vec3 {
        Vec3::new(self.x * value, self.y * value, self.z * value)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, value: f32) -> Vec3 {
        self * (1.0 / value)
    }
}

// ----------------------------------------------
// Ray definition -------------------------------
// ----------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
}


pub fn linear_to_gamma(value: f32) -> f32 {
    if value > 0.0 {
        return value.sqrt();
    }
    0.0
}

