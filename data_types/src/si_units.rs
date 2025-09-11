use core::fmt;

use std::{marker::PhantomData};
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};
use typenum::{Diff, Integer, Sum, N1, N2, N3, P1, P2, P3, Z0};

macro_rules! impl_unit_conversions {
    (
        $(
            $unit:ident {
                $( $set_name:ident, $get_name:ident => $factor:expr ),+ $(,)?
            }
        )+ $(,)?
    ) => {
        $(
            impl $unit {
                $(
                    pub fn $set_name(value: f64) -> Self {
                        SiValue::new(value * $factor)
                    }
                    
                    pub fn $get_name(&self) -> f64 {
                        self.value / $factor
                    }
                )+
            }
        )+
    };
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SiValue<L, M, T> {
    value: f64,
    _type: PhantomData<(L, M, T)>,
}

pub type Distance = SiValue<P1, Z0, Z0>;
pub type Area = SiValue<P2, Z0, Z0>;
pub type Volume = SiValue<P3, Z0, Z0>;
pub type Time = SiValue<Z0, Z0, P1>;
pub type Frequency = SiValue<Z0, Z0, N1>;
pub type Speed = SiValue<P1, Z0, N1>;
pub type Acceleration = SiValue<P1, Z0, N2>;
pub type Mass = SiValue<Z0, P1, Z0>;
pub type Force = SiValue<P1, P1, N2>;
pub type Energy = SiValue<P2, P1, N2>;
pub type Power = SiValue<P2, P1, N3>;
pub type Torque = SiValue<P2, P1, N2>;
pub type AngularVelocity = SiValue<Z0, Z0, N1>;
pub type AngularAcceleration = SiValue<Z0, Z0, N2>;
pub type Momentum = SiValue<P1, P1, N1>;

impl_unit_conversions!(
    Momentum {
        newton_seconds , as_newton_seconds => 1.0,
        kilonewton_seconds , as_kilonewton_seconds => 1e3,
        meganewton_seconds , as_meganewton_seconds => 1e6
    }
    AngularVelocity {
        radians_per_second , as_radians_per_second => 1.0,
        revolutions_per_minute , as_revolutions_per_minute => 2.0 * std::f64::consts::PI / 60.0
    }
    AngularAcceleration {
        radians_per_second_squared , as_radians_per_second_squared => 1.0,
        revolutions_per_minute_squared , as_revolutions_per_minute_squared => 2.0 * std::f64::consts::PI / 60.0 / 60.0
    }
    Torque {
        newton_meters , as_newton_meters => 1.0,
        kilonewton_meters , as_kilonewton_meters => 1e3,
        meganewton_meters , as_meganewton_meters => 1e6
    }
    Power {
        watts , as_watts => 1.0,
        kilowatts , as_kilowatts => 1e3,
        megawatts , as_megawatts => 1e6,
        gigawatts , as_gigawatts => 1e9,
        milliwatts , as_milliwatts => 1e-3
    }
    Energy {
        joules , as_joules => 1.0,
        kilojoules , as_kilojoules => 1e3,
        megajoules , as_megajoules => 1e6,
        calories , as_calories => 4.184,
        kilocalories , as_kilocalories => 4.184e3
    }
    Frequency {
        hertz , as_hertz => 1.0,
        kilohertz , as_kilohertz => 1e3,
        megahertz , as_megahertz => 1e6,
        gigahertz , as_gigahertz => 1e9
    }
    Distance {
        meters , as_meters => 1.0,
        kilometers , as_kilometers => 1e3,
        centimeters , as_centimeters => 1e-2,
        millimeters , as_millimeters => 1e-3,
        micrometers , as_micrometers => 1e-6,
        nanometers , as_nanometers => 1e-9
    }
    Area {
        square_meters , as_square_meters => 1.0,
        square_kilometers , as_square_kilometers => 1e6,
        square_centimeters , as_square_centimeters => 1e-4,
        square_millimeters , as_square_millimeters => 1e-6
    }
    Volume {
        cubic_meters , as_cubic_meters => 1.0,
        liters , as_liters => 1e-3,
        cubic_kilometers , as_cubic_kilometers => 1e9,
        cubic_centimeters , as_cubic_centimeters => 1e-6,
        cubic_millimeters , as_cubic_millimeters => 1e-9
    }
    Time {
        seconds , as_seconds => 1.0,
        milliseconds , as_milliseconds => 1e-3,
        microseconds , as_microseconds => 1e-6,
        minutes , as_minutes => 60.0,
        hours , as_hours => 3600.0
    }
    Speed {
        meters_per_second , as_meters_per_second => 1.0,
        kilometers_per_hour , as_kilometers_per_hour => 1000.0 / 3600.0
    }
    Acceleration {
        meters_per_second_squared , as_meters_per_second_squared => 1.0,
        kilometers_per_hour_squared , as_kilometers_per_hour_squared => 1000.0 / 3600.0 / 3600.0
    }
    Mass {
        metric_tons , as_metric_tons => 1e3,
        kilograms , as_kilograms => 1.0,
        grams , as_grams => 1e-3,
        milligrams , as_milligrams => 1e-6
    }
    Force {
        micronewtons , as_micronewtons => 1e-6,
        millinewtons , as_millinewtons => 1e-3,
        newtons , as_newtons => 1.0,
        kilonewtons , as_kilonewtons => 1e3,
        meganewtons , as_meganewtons => 1e6
    }
);

impl<L, M, T> SiValue<L, M, T> {
    fn new(value: f64) -> Self {
        Self {
            value,
            _type: PhantomData,
        }
    }
}

impl<L, M, T> Add for SiValue<L, M, T> {
    type Output = SiValue<L, M, T>;

    fn add(self, rhs: SiValue<L, M, T>) -> Self::Output {
        SiValue::new(self.value + rhs.value)
    }
}

impl<L, M, T> Sub for SiValue<L, M, T> {
    type Output = SiValue<L, M, T>;

    fn sub(self, rhs: SiValue<L, M, T>) -> Self::Output {
        SiValue::new(self.value - rhs.value)
    }
}

impl<L1, M1, T1, L2, M2, T2> Mul<SiValue<L2, M2, T2>> for SiValue<L1, M1, T1>
where
    L1: Integer + std::ops::Add<L2>,
    M1: Integer + std::ops::Add<M2>,
    T1: Integer + std::ops::Add<T2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
{
    type Output = SiValue<Sum<L1, L2>, Sum<M1, M2>, Sum<T1, T2>>;

    fn mul(self, rhs: SiValue<L2, M2, T2>) -> Self::Output {
        SiValue::new(self.value * rhs.value)
    }
}

impl<L1, M1, T1, L2, M2, T2> Div<SiValue<L2, M2, T2>> for SiValue<L1, M1, T1>
where
    L1: Integer + std::ops::Sub<L2>,
    M1: Integer + std::ops::Sub<M2>,
    T1: Integer + std::ops::Sub<T2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
{
    type Output = SiValue<Diff<L1, L2>, Diff<M1, M2>, Diff<T1, T2>>;

    fn div(self, rhs: SiValue<L2, M2, T2>) -> Self::Output {
        SiValue::new(self.value / rhs.value)
    }
}

impl<L, M, T> Mul<f64> for SiValue<L, M, T> {
    type Output = SiValue<L, M, T>;

    fn mul(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value * rhs)
    }
} 

impl<L, M, T> Mul<SiValue<L,M,T>> for f64 {
    type Output = SiValue<L, M, T>;

    fn mul(self, rhs: SiValue<L,M,T>) -> Self::Output {
        SiValue::new(self * rhs.value)
    }
} 

impl<L, M, T> MulAssign<f64> for SiValue<L, M, T> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

impl<L, M, T> Div<f64> for SiValue<L, M, T> {
    type Output = SiValue<L, M, T>;

    fn div(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value / rhs)
    }
}

impl<L, M, T> Div<SiValue<L, M, T>> for f64 {
    type Output = SiValue<L, M, T>;

    fn div(self, rhs: SiValue<L, M, T>) -> Self::Output {
        SiValue::new(self / rhs.value)
    }
}

impl<L, M, T> DivAssign<f64> for SiValue<L, M, T> {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}

fn format_unit(name: &str, exp: i64) -> String {
    match exp {
        1 => format!("{}", name),
        _ => format!("{}^{}", name, exp),
    }
}

impl<L, M, T> fmt::Display for SiValue<L, M, T>
where
    L: Integer,
    M: Integer,
    T: Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l_exp = L::to_i64();
        let m_exp = M::to_i64();
        let t_exp = T::to_i64();

        let mut numerator = Vec::new();
        let mut denominator = Vec::new();

        if l_exp > 0 {
            numerator.push(format_unit("m", l_exp));
        } else if l_exp < 0 {
            denominator.push(format_unit("m", -l_exp));
        }

        if m_exp > 0 {
            numerator.push(format_unit("kg", m_exp));
        } else if m_exp < 0 {
            denominator.push(format_unit("kg", -m_exp));
        }

        if t_exp > 0 {
            numerator.push(format_unit("s", t_exp));
        } else if t_exp < 0 {
            denominator.push(format_unit("s", -t_exp));
        }

        let unit_str = match (numerator.len(), denominator.len()) {
            (0, 0) => "".to_string(),
            (_, 0) => format!(" [{}]", numerator.join("·")),
            (0, _) => format!(" [1/{}]", denominator.join("·")),
            (_, _) => format!(" [{}/{}]", numerator.join("·"), denominator.join("·")),
        };

        if f64::abs(self.value) < 1e-4 || f64::abs(self.value) >= 1e4 {
            write!(f, "{:.5e}{}", self.value, unit_str)
        } else {
            write!(f, "{}{}", self.value, unit_str)
        }
    }
}
