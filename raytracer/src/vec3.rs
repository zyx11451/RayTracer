use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: (f64, f64, f64),
}
impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
        }
    }
}
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: (self.e.0 - rhs.e.0, self.e.1 - rhs.e.1, self.e.2 - rhs.e.2),
        }
    }
}

pub fn mul_vec_dot(lhs: Vec3, rhs: Vec3) -> f64 {
    lhs.e.0 * rhs.e.0 + lhs.e.1 * rhs.e.1 + lhs.e.2 * rhs.e.2
}
pub fn mul_vec_cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        e: (
            lhs.e.1 * rhs.e.2 - lhs.e.2 * rhs.e.1,
            lhs.e.2 * rhs.e.0 - lhs.e.0 * rhs.e.2,
            lhs.e.0 * rhs.e.1 - lhs.e.1 * rhs.e.0,
        ),
    }
}
pub fn mul_num(lhs: Vec3, rhs: f64) -> Vec3 {
    Vec3 {
        e: ((lhs.e.0) * rhs, (lhs.e.1) * rhs, (lhs.e.2) * rhs),
    }
}
fn div_vec(lhs: Vec3, rhs: f64) -> Vec3 {
    mul_num(lhs, 1.0 / rhs)
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: (
                self.e.0 + other.e.0,
                self.e.1 + other.e.1,
                self.e.2 + other.e.2,
            ),
        };
    }
}
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e: (
                self.e.0 - other.e.0,
                self.e.1 - other.e.1,
                self.e.2 - other.e.2,
            ),
        };
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: (-self.e.0, -self.e.1, -self.e.2),
        }
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: (self.e.0 / rhs, self.e.1 / rhs, self.e.2 / rhs),
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: (self.e.0 * rhs, self.e.1 * rhs, self.e.2 * rhs),
        }
    }
}
impl Vec3 {
    pub fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: ((self.e.0) * other, (self.e.1) * other, (self.e.2) * other),
        };
    }
    pub fn length_square(&self) -> f64 {
        self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
    }
    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }
    pub fn unit_vector(&self) -> Vec3 {
        div_vec(*self, self.length())
    }
}
pub type Color = Vec3;
pub type Point3 = Vec3;
