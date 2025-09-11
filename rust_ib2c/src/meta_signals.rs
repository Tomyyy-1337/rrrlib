use std::{fmt::Display, ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

/// A meta-signal representing a value between 0.0 and 1.0 inclusive.
/// Supports arithmetic operations and comparisons.
/// Construction and all operations ensure the value remains within bounds.
#[derive(Debug, Clone, Copy)]
pub struct MetaSignal {
    value: f32,
}

impl From<f32> for MetaSignal {
    fn from(value: f32) -> Self {
        MetaSignal::new(value)
    }
}

impl Deref for MetaSignal {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MetaSignal {
    /// Creates a new MetaSignal, clamping the value between 0.0 and 1.0.
    pub fn new(value: f32) -> Self {
        Self { value: value.clamp(0.0, 1.0) }
    }

    pub const LOW: MetaSignal = MetaSignal { value: 0.0 };
    pub const HIGH: MetaSignal = MetaSignal { value: 1.0 };
}

impl Add for MetaSignal {
    type Output = MetaSignal;

    /// Adds two MetaSignals, clamping the result to a maximum of 1.0.
    fn add(self, other: Self) -> Self::Output {
        let value = self.value + other.value;
        MetaSignal {
            value: f32::min(value, 1.0),
        }
    }
}

impl Add<MetaSignal> for f32 {
    type Output = MetaSignal;

    /// Adds a MetaSignal to a f32, clamping the result between 0.0 and 1.0.
    fn add(self, other: MetaSignal) -> Self::Output {
        MetaSignal::new(self + other.value)
    }
}

impl Add<f32> for MetaSignal {
    type Output = MetaSignal;

    /// Adds a f32 to a MetaSignal, clamping the result between 0.0 and 1.0.
    fn add(self, other: f32) -> Self::Output {
        MetaSignal::new(self.value + other)
    }
}

impl AddAssign for MetaSignal {
    /// Adds another MetaSignal to this one, clamping the result to a maximum of 1.0.
    fn add_assign(&mut self, other: Self) {
        self.value = f32::min(self.value + other.value, 1.0);
    }
}

impl AddAssign<MetaSignal> for f32 {
    /// Adds a MetaSignal to this f32, clamping the result between 0.0 and 1.0.
    fn add_assign(&mut self, other: MetaSignal) {
        let value = *self + other.value;
        *self = value.clamp(0.0, 1.0);
    }
}

impl AddAssign<f32> for MetaSignal {
    /// Adds a f32 to this MetaSignal, clamping the result between 0.0 and 1.0.
    fn add_assign(&mut self, other: f32) {
        let value = self.value + other;
        self.value = value.clamp(0.0, 1.0);
    }
}

impl Sub for MetaSignal {
    type Output = MetaSignal;

    /// Subtracts one MetaSignal from another, clamping the result to a minimum of 0.0.
    fn sub(self, other: Self) -> Self::Output {
        let value = self.value - other.value;
        MetaSignal {
            value: f32::max(value, 0.0),
        }
    }
}

impl Sub<MetaSignal> for f32 {
    type Output = MetaSignal;

    /// Subtracts a MetaSignal from a f32, clamping the result between 0.0 and 1.0.
    fn sub(self, other: MetaSignal) -> Self::Output {
        MetaSignal::new(self - other.value)
    }
}

impl Sub<f32> for MetaSignal {
    type Output = MetaSignal;

    /// Subtracts a f32 from a MetaSignal, clamping the result between 0.0 and 1.0.
    fn sub(self, other: f32) -> Self::Output {
        MetaSignal::new(self.value - other)
    }
}

impl SubAssign for MetaSignal {
    /// Subtracts another MetaSignal from this one, clamping the result to a minimum of 0.0.
    fn sub_assign(&mut self, other: Self) {
        self.value = f32::max(self.value - other.value, 0.0);
    }
}

impl SubAssign<MetaSignal> for f32 {
    /// Subtracts a MetaSignal from this f32, clamping the result between 0.0 and 1.0.
    fn sub_assign(&mut self, other: MetaSignal) {
        let value = *self - other.value;
        *self = value.clamp(0.0, 1.0);
    }
}

impl SubAssign<f32> for MetaSignal {
    /// Subtracts a f32 from this MetaSignal, clamping the result between 0.0 and 1.0.
    fn sub_assign(&mut self, other: f32) {
        let value = self.value - other;
        self.value = value.clamp(0.0, 1.0);
    }
}

impl Mul for MetaSignal {
    type Output = MetaSignal;
    /// Multiplies two MetaSignals.
    fn mul(self, rhs: MetaSignal) -> Self::Output {
        MetaSignal {
            value: self.value * rhs.value,
        }
    }
}

impl Mul<MetaSignal> for f32 {
    type Output = MetaSignal;

    /// Multiplies a f32 by a MetaSignal, clamping the result between 0.0 and 1.0.
    fn mul(self, rhs: MetaSignal) -> Self::Output {
        MetaSignal::new(self * rhs.value)
    }
}

impl Mul<f32> for MetaSignal {
    type Output = MetaSignal;

    /// Multiplies a MetaSignal by a f32, clamping the result between 0.0 and 1.0.
    fn mul(self, rhs: f32) -> Self::Output {
        MetaSignal::new(self.value * rhs)
    }
}   

impl MulAssign for MetaSignal {
    /// Multiplies this MetaSignal by another. Bounds checking is not necessary
    /// since both values are guaranteed to be between 0.0 and 1.0.
    fn mul_assign(&mut self, rhs: MetaSignal) {
        self.value *= rhs.value;
    }
}

impl MulAssign<MetaSignal> for f32 {
    /// Multiplies this f32 by a MetaSignal, clamping the result between 0.0 and 1.0.
    fn mul_assign(&mut self, rhs: MetaSignal) {
        let value = *self * rhs.value;
        *self = value.clamp(0.0, 1.0);
    }
}

impl MulAssign<f32> for MetaSignal {
    /// Multiplies this MetaSignal by a f32, clamping the result between 0.0 and 1.0.
    fn mul_assign(&mut self, rhs: f32) {
        let value = self.value * rhs;
        self.value = value.clamp(0.0, 1.0);
    }
}

impl Div for MetaSignal {
    type Output = MetaSignal;

    /// Divides one MetaSignal by another, clamping the result to a maximum of 1.0.
    /// Division by zero results in a MetaSignal of value 1.0.
    fn div(self, rhs: MetaSignal) -> Self::Output {
        if rhs == 0.0 {
            return MetaSignal { value: 1.0 };
        }
        let value = self.value / *rhs;
        MetaSignal {
            value: f32::min(value, 1.0)
        }
    }
}

impl Div<MetaSignal> for f32 {
    type Output = MetaSignal;

    /// Divides a f32 by a MetaSignal, clamping the result between 0.0 and 1.0.
    /// Division by zero results in a MetaSignal of value 1.0.
    fn div(self, rhs: MetaSignal) -> Self::Output {
        if rhs == 0.0 {
            return MetaSignal { value: 1.0 };
        }
        MetaSignal::new(self / *rhs)
    }
}

impl Div<f32> for MetaSignal {
    type Output = MetaSignal;

    /// Divides a MetaSignal by a f32, clamping the result between 0.0 and 1.0.
    /// Division by zero results in a MetaSignal of value 1.0.
    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            return MetaSignal { value: 1.0 };
        }
        MetaSignal::new(self.value / rhs)
    }
}

impl DivAssign for MetaSignal {
    /// Divides this MetaSignal by another, clamping the result to a maximum of 1.0.
    fn div_assign(&mut self, rhs: MetaSignal) {
        if rhs == 0.0 {
            self.value = 1.0;
        } else {
            let value = self.value / *rhs;
            self.value = f32::min(value, 1.0);
        }
    }
}

impl DivAssign<MetaSignal> for f32 {
    /// Divides this f32 by a MetaSignal, clamping the result between 0.0 and 1.0.
    /// Division by zero results in a value of 1.0.
    fn div_assign(&mut self, rhs: MetaSignal) {
        if rhs == 0.0 {
            *self = 1.0;
        } else {
            let value = *self / *rhs;
            *self = value.clamp(0.0, 1.0);
        }
    }
}

impl DivAssign<f32> for MetaSignal {
    /// Divides this MetaSignal by a f32, clamping the result between 0.0 and 1.0.
    /// Division by zero results in a MetaSignal of value 1.0.
    fn div_assign(&mut self, rhs: f32) {
        if rhs == 0.0 {
            self.value = 1.0;
        } else {
            let value = self.value / rhs;
            self.value = value.clamp(0.0, 1.0);
        }
    }
}

impl PartialEq for MetaSignal {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<f32> for MetaSignal {
    fn eq(&self, other: &f32) -> bool {
        self.value == *other
    }
}

impl PartialEq<MetaSignal> for f32 {
    fn eq(&self, other: &MetaSignal) -> bool {
        *self == other.value
    }
}

impl Eq for MetaSignal {}

impl PartialOrd for MetaSignal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<f32> for MetaSignal {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<MetaSignal> for f32 {
    fn partial_cmp(&self, other: &MetaSignal) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl Ord for MetaSignal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl Display for MetaSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.>1}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::MetaSignal;
    #[test]
    fn test_meta_signal() {
        let mut a = MetaSignal::new(0.5);
        let b = MetaSignal::new(0.3);
        let c = MetaSignal::new(0.8);
        let d = MetaSignal::new(1.2);
        let e = MetaSignal::new(-0.5);

        assert_eq!(*a, 0.5);
        assert_eq!(*b, 0.3);
        assert_eq!(*c, 0.8);
        assert_eq!(*d, 1.0);
        assert_eq!(*e, 0.0);

        assert_eq!(a + b, f32::clamp(*a + *b, 0.0, 1.0));
        assert_eq!(a + 0.6, f32::clamp(*a + 0.6, 0.0, 1.0));
        assert_eq!(0.6 + a, f32::clamp(0.6 + *a, 0.0, 1.0));
        assert_eq!(a - b, f32::clamp(*a - *b, 0.0, 1.0));
        assert_eq!(a - 0.6, f32::clamp(*a - 0.6, 0.0, 1.0));
        assert_eq!(0.4 - a, f32::clamp(0.4 - *a, 0.0, 1.0));
        assert_eq!(a * b, f32::clamp(*a * *b, 0.0, 1.0));
        assert_eq!(a * 10.0, f32::clamp(*a * 10.0, 0.0, 1.0));
        assert_eq!(10.0 * a, f32::clamp(10.0 * *a, 0.0, 1.0));
        assert_eq!(a / b, f32::clamp(*a / *b, 0.0, 1.0));
        assert_eq!(a / 10.0, f32::clamp(*a / 10.0, 0.0, 1.0));
        assert_eq!(10.0 / a, f32::clamp(10.0 / *a, 0.0, 1.0));
        assert_eq!(a / 0.0, 1.0);
        a += b;
        assert_eq!(*a, f32::clamp(0.5 + 0.3, 0.0, 1.0));
        a -= b;
        assert_eq!(*a, f32::clamp(0.8 - 0.3, 0.0, 1.0));
        a -= 0.6;
        assert_eq!(*a, f32::clamp(0.5 - 0.6, 0.0, 1.0));
        a -= 0.6;
        assert_eq!(*a, 0.0);
        a += 0.5;
        assert_eq!(*a, 0.5);
        a *= b;
        assert_eq!(*a, 0.5 * 0.3);
        a *= 10.0;
        assert_eq!(*a, 1.0);
        a *= 0.1;
        assert_eq!(*a, 0.1);
        a /= b;
        assert_eq!(*a, f32::clamp(0.1 / 0.3, 0.0, 1.0));
        a /= 0.0;
        assert_eq!(*a, 1.0);
        a /= 10.0;
        assert_eq!(*a, f32::clamp(1.0 / 10.0, 0.0, 1.0));
        a /= 0.0;
        assert_eq!(*a, 1.0);
        assert!(a > b);
        assert!(b < a);
        assert!(a >= b);
        assert!(b <= a);
        assert!(a != b);
        assert!(a == a);
        assert!(a >= 0.5);
        assert!(a <= 1.0);
        assert!(a > 0.4);
        assert!(a < 1.1);
        assert!(a != 0.4);
        assert!(a == 1.0);
        assert!(0.4 < a);
        assert!(1.1 > a);
        assert!(0.5 <= a);
        assert!(1.0 >= a);
        assert!(0.4 != a);
        assert!(1.0 == a);
    }
}