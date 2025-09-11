use core::fmt;

use std::{marker::PhantomData};
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};
use typenum::{Diff, Integer, Sum, N1, N2, N3, P1, P2, P3, P4, Z0};

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

macro_rules! new_types {
    (
        $(
            $name:ident, $symbol:literal => kg^$mass:ty, m^$length:ty, s^$time:ty, A^$current:ty
        ),* $(,)?
    ) => {
        $(
            pub type $name = SiValue<$length, $mass, $time, $current>;
        )*

                
        impl<L, M, T, A> fmt::Display for SiValue<L, M, T, A>
        where
            L: Integer,
            M: Integer,
            T: Integer,
            A: Integer,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // Check for known types and use their symbols
                let mut unit_str = String::new();
                    // Helper macro to match type parameters
                macro_rules! match_type {
                    ($l:ty, $m:ty, $t:ty, $a:ty) => {
                        std::any::TypeId::of::<L>() == std::any::TypeId::of::<$l>()
                        && std::any::TypeId::of::<M>() == std::any::TypeId::of::<$m>()
                        && std::any::TypeId::of::<T>() == std::any::TypeId::of::<$t>()
                        && std::any::TypeId::of::<A>() == std::any::TypeId::of::<$a>()
                    };
                }         

                $(
                    if match_type!($length, $mass, $time, $current) {
                        unit_str = format!(" [{}]", $symbol);
                    }
                )*

                if unit_str.is_empty() {
                    let l_exp = L::to_i64();
                    let m_exp = M::to_i64();
                    let t_exp = T::to_i64();
                    let a_exp = A::to_i64();

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

                    if a_exp > 0 {
                        numerator.push(format_unit("A", a_exp));
                    } else if a_exp < 0 {
                        denominator.push(format_unit("A", -a_exp));
                    }

                    unit_str = match (numerator.len(), denominator.len()) {
                        (0, 0) => "".to_string(),
                        (_, 0) => format!(" [{}]", numerator.join("·")),
                        (0, _) => format!(" [1/{}]", denominator.join("·")),
                        (_, _) => format!(" [{}/{}]", numerator.join("·"), denominator.join("·")),
                    };
                }

                if f64::abs(self.value) < 1e-4 || f64::abs(self.value) >= 1e4 {
                    write!(f, "{:.5e}{}", self.value, unit_str)
                } else {
                    write!(f, "{}{}", self.value, unit_str)
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SiValue<L, M, T, A> {
    value: f64,
    _type: PhantomData<(L, M, T, A)>,
}

new_types!{
    Distance, "m" => kg^Z0, m^P1, s^Z0, A^Z0,
    Area, "m²" => kg^Z0, m^P2, s^Z0, A^Z0,
    Volume, "m³" => kg^Z0, m^P3, s^Z0, A^Z0,
    Time, "s" => kg^Z0, m^Z0, s^P1, A^Z0,
    Frequency, "Hz" => kg^Z0, m^Z0, s^N1, A^Z0,
    Speed, "m/s" => kg^Z0, m^P1, s^N1, A^Z0,
    Acceleration, "m/s²" => kg^Z0, m^P1, s^N2, A^Z0,
    Mass, "kg" => kg^P1, m^Z0, s^Z0, A^Z0,
    Force, "N" => kg^P1, m^P1, s^N2, A^Z0,
    Torque, "Nm" => kg^P1, m^P2, s^N2, A^Z0,
    Energy, "J" => kg^P1, m^P2, s^N2, A^Z0,
    Power, "W" => kg^P1, m^P2, s^N3, A^Z0,
    Momentum, "N·s" => kg^P1, m^P1, s^N1, A^Z0,
    AngularVelocity, "rad/s" => kg^Z0, m^Z0, s^N1, A^Z0,
    AngularAcceleration, "rad/s²" => kg^Z0, m^Z0, s^N2, A^Z0,
    Current, "A" => kg^Z0, m^Z0, s^Z0, A^P1,
    Charge, "C" => kg^Z0, m^Z0, s^P1, A^P1,
    Voltage, "V" => kg^P1, m^P2, s^N3, A^N1,
    Resistance, "Ω" => kg^P1, m^P2, s^N3, A^N2,
    Capacitance, "F" => kg^N1, m^N2, s^P4, A^N2,
    Inductance, "H" => kg^P1, m^P2, s^N2, A^N2,
    MagneticFlux, "Wb" => kg^P1, m^P2, s^N2, A^N1,
    MagneticFieldStrength, "T" => kg^P1, m^Z0, s^N2, A^N1
}

impl_unit_conversions!(
    Inductance {
        henrys , as_henrys => 1.0,
        millihenrys , as_millihenrys => 1e-3,
        microhenrys , as_microhenrys => 1e-6
    }
    MagneticFlux {
        webers , as_webers => 1.0,
        milliwbers , as_milliwebers => 1e-3,
        microwebers , as_microwebers => 1e-6
    }
    MagneticFieldStrength {
        teslas , as_teslas => 1.0,
        milliteslas , as_milliteslas => 1e-3,
        microteslas , as_microteslas => 1e-6
    }
    Current {
        amperes , as_amperes => 1.0,
        milliamperes , as_milliamperes => 1e-3,
        kiloamperes , as_kiloamperes => 1e3
    }
    Charge{
        coulombs , as_coulombs => 1.0,
        millicoulombs , as_millicoulombs => 1e-3,
        kilocoulombs , as_kilocoulombs => 1e3
    }
    Voltage {
        volts , as_volts => 1.0,
        millivolts , as_millivolts => 1e-3,
        kilovolts , as_kilovolts => 1e3
    }
    Resistance {
        ohms , as_ohms => 1.0,
        milliohms , as_milliohms => 1e-3,
        kiloohms , as_kiloohms => 1e3,
        megaohms , as_megaohms => 1e6
    }
    Capacitance {
        farads , as_farads => 1.0,
        millifarads , as_millifarads => 1e-3,
        microfarads , as_microfarads => 1e-6,
        nanofarads , as_nanofarads => 1e-9,
        picofarads , as_picofarads => 1e-12
    }
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

impl<L, M, T, A> SiValue<L, M, T, A> {
    fn new(value: f64) -> Self {
        Self {
            value,
            _type: PhantomData,
        }
    }
}

impl<L, M, T, A> Add for SiValue<L, M, T, A> {
    type Output = SiValue<L, M, T, A>;

    fn add(self, rhs: SiValue<L, M, T, A>) -> Self::Output {
        SiValue::new(self.value + rhs.value)
    }
}

impl<L, M, T, A> Sub for SiValue<L, M, T, A> {
    type Output = SiValue<L, M, T, A>;

    fn sub(self, rhs: SiValue<L, M, T, A>) -> Self::Output {
        SiValue::new(self.value - rhs.value)
    }
}

impl<L1, M1, T1, A1, L2, M2, T2, A2> Mul<SiValue<L2, M2, T2, A2>> for SiValue<L1, M1, T1, A1>
where
    L1: Integer + std::ops::Add<L2>,
    M1: Integer + std::ops::Add<M2>,
    T1: Integer + std::ops::Add<T2>,
    A1: Integer + std::ops::Add<A2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
{
    type Output = SiValue<Sum<L1, L2>, Sum<M1, M2>, Sum<T1, T2>, Sum<A1, A2>>;

    fn mul(self, rhs: SiValue<L2, M2, T2, A2>) -> Self::Output {
        SiValue::new(self.value * rhs.value)
    }
}

impl<L1, M1, T1, A1, L2, M2, T2, A2> Div<SiValue<L2, M2, T2, A2>> for SiValue<L1, M1, T1, A1>
where
    L1: Integer + std::ops::Sub<L2>,
    M1: Integer + std::ops::Sub<M2>,
    T1: Integer + std::ops::Sub<T2>,
    A1: Integer + std::ops::Sub<A2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
{
    type Output = SiValue<Diff<L1, L2>, Diff<M1, M2>, Diff<T1, T2>, Diff<A1, A2>>;

    fn div(self, rhs: SiValue<L2, M2, T2, A2>) -> Self::Output {
        SiValue::new(self.value / rhs.value)
    }
}

impl<L, M, T, A> Mul<f64> for SiValue<L, M, T, A> {
    type Output = SiValue<L, M, T, A>;

    fn mul(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value * rhs)
    }
}

impl<L, M, T, A> Mul<SiValue<L, M, T, A>> for f64 {
    type Output = SiValue<L, M, T, A>;

    fn mul(self, rhs: SiValue<L, M, T, A>) -> Self::Output {
        SiValue::new(self * rhs.value)
    }
}

impl<L, M, T, A> MulAssign<f64> for SiValue<L, M, T, A> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

impl<L, M, T, A> Div<f64> for SiValue<L, M, T, A> {
    type Output = SiValue<L, M, T, A>;

    fn div(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value / rhs)
    }
}

impl<L, M, T, A> Div<SiValue<L, M, T, A>> for f64 {
    type Output = SiValue<L, M, T, A>;

    fn div(self, rhs: SiValue<L, M, T, A>) -> Self::Output {
        SiValue::new(self / rhs.value)
    }
}

impl<L, M, T, A> DivAssign<f64> for SiValue<L, M, T, A> {
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
