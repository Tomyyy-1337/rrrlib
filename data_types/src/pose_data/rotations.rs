use std::ops::{Add, Mul};

use crate::{pose_data::{Vector2, Vector3}, prelude::Radian};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation2D {
    // Angle in radians in range [0, 2π]
    pub angle: f64,
}

impl Rotation2D {
    pub const PI: Self = Self { angle: std::f64::consts::PI };
    pub const PI_2: Self = Self { angle: std::f64::consts::FRAC_PI_2 }; 
    pub const ZERO: Self = Self { angle: 0.0 };

    pub fn from_angle(angle: f64) -> Self {
        let mut a = angle % (2.0 * std::f64::consts::PI);
        if a < 0.0 {
            a += 2.0 * std::f64::consts::PI;
        }
        Self { angle: a }
    }

    pub fn rotate_vector<T>(&self, v: Vector2<T>) -> Vector2<T> 
    where 
        T: Default + Copy + std::ops::Add<T, Output = T> + std::ops::Sub<T, Output = T> + std::ops::Mul<f64, Output = T>
    {
        let (s, c) = self.angle.sin_cos();
        Vector2::new(
            v.x() * c - v.y() * s,
            v.x() * s + v.y() * c,
        )
    }

    pub fn as_radians(&self) -> f64 {
        self.angle
    }

    pub fn as_degrees(&self) -> f64 {
        self.angle.to_degrees()
    }

    pub fn as_unit_vector(&self) -> Vector2<f64> {
        let (s, c) = self.angle.sin_cos();
        Vector2::new(c, s)
    }
}

impl Add for &Rotation2D {
    type Output = Rotation2D;

    fn add(self, rhs: Self) -> Self::Output {
        Rotation2D::from_angle(self.angle + rhs.angle)
    }
}

impl Add<Rotation2D> for Rotation2D {
    type Output = Self;

    fn add(self, rhs: Rotation2D) -> Self::Output {
        Self::from_angle(self.angle + rhs.angle)
    }
}

impl Add<&Radian> for Rotation2D {
    type Output = Self;

    fn add(self, rhs: &Radian) -> Self::Output {
        Self::from_angle(self.angle + rhs.as_radians())
    }
}

impl<T> Add<T> for Rotation2D 
where
    T: std::borrow::Borrow<f64>
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::from_angle(self.angle + rhs.borrow())
    }
}

impl Mul<f64> for Rotation2D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_angle(self.angle * rhs)
    }
}

impl Mul<Rotation2D> for f64 {
    type Output = Rotation2D;

    fn mul(self, rhs: Rotation2D) -> Self::Output {
        Rotation2D::from_angle(self * rhs.angle)
    }
}

pub struct Rotation3D {
    // Unit quaternion representing orientation
    // w : scalar part
    pub w: f64,
    // x, y, z : vector part
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Rotation3D {
    pub fn from_quaternion(w: f64, x: f64, y: f64, z: f64) -> Self {
        let norm_squared = w*w + x*x + y*y + z*z;
        if norm_squared == 0.0 {
            return Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };
        }
        let scale = if norm_squared == 1.0 { 1.0 } else { norm_squared.sqrt().recip() };
        Self { w: w*scale, x: x*scale, y: y*scale, z: z*scale }
    }

    pub fn from_euler_angles(x: f64, y: f64, z: f64) -> Self {
        let (sr, cr) = (0.5 * x).sin_cos();
        let (sp, cp) = (0.5 * y).sin_cos();
        let (sy, cy) = (0.5 * z).sin_cos();

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = cr * sp * cy + sr * cp * sy;
        let z = cr * cp * sy - sr * sp * cy;

        Self::from_quaternion(w, x, y, z)
    }

    pub fn rotate_vector(&self, v: Vector3<f64>) -> Vector3<f64> {
        let qv = Vector3::new(self.x, self.y, self.z);
        let t = qv.cross(v) * 2.0;
        v + t * self.w + qv.cross(t)
    }

    pub fn x_rotation(&self) -> f64 {
        (2.0 * (self.w * self.x + self.y * self.z)).atan2(1.0 - 2.0 * (self.x * self.x + self.y * self.y))
    }

    pub fn y_rotation(&self) -> f64 {
        let sp = 2.0 * (self.w * self.y - self.z * self.x);
        if sp.abs() >= 1.0 {
            sp.signum() * (std::f64::consts::FRAC_PI_2) 
        } else {
            sp.asin()
        }
    }

    pub fn z_rotation(&self) -> f64 {
        (2.0 * (self.w * self.z + self.x * self.y)).atan2(1.0 - 2.0 * (self.y * self.y + self.z * self.z))
    }

    pub fn as_quaternion(&self) -> (f64, f64, f64, f64) {
        (self.w, self.x, self.y, self.z)
    }
}

