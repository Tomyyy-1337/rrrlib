use core::fmt;

use std::{marker::PhantomData};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use typenum::{Diff, Integer, Negate, Sum, N1, N2, N3, P1, P2, P3, P4, Z0};

macro_rules! impl_unit_conversions {
    (
        $(
            $unit:ident {
                $( $set_name:ident, $get_name:ident => $factor:expr $(=> offset= $offset:expr)? ),+ $(,)?
                $(=> constants {
                    $( $const_name:ident , $const_value:expr ),* $(,)?
                })?
            }
        )+ $(,)?
    ) => {
        $(
            impl $unit {
                $($(

                    pub const $const_name: Self = SiValue::new($const_value);
                )*)?
                
                $(
                    pub fn $set_name(value: f64) -> Self {
                        SiValue::new((value $(+ $offset)? ) * $factor)
                    }
                    
                    pub fn $get_name(&self) -> f64 {
                        self.value / $factor $( - $offset )?
                    }
                )+
            }
        )+
    };
}

macro_rules! match_type {
    ($l:ty, $m:ty, $t:ty, $a:ty, $k:ty) => {
        std::any::TypeId::of::<L>() == std::any::TypeId::of::<$l>()
        && std::any::TypeId::of::<M>() == std::any::TypeId::of::<$m>()
        && std::any::TypeId::of::<T>() == std::any::TypeId::of::<$t>()
        && std::any::TypeId::of::<A>() == std::any::TypeId::of::<$a>()
        && std::any::TypeId::of::<K>() == std::any::TypeId::of::<$k>()
    };
}    

macro_rules! new_types {
    (
        $(
            $name:ident, $symbol:expr => kg^$mass:ty, m^$length:ty, s^$time:ty, A^$current:ty, K^$kelvin:ty
        ),* $(,)?
    ) => {
        $(
            pub type $name = SiValue<$length, $mass, $time, $current, $kelvin>;
        )*
    
        impl<L, M, T, A, K> SiValue<L, M, T, A, K>
        where
            L: Integer,
            M: Integer,
            T: Integer,
            A: Integer,
            K: Integer,
        {
            fn unit_symbol(&self) -> String {
                $(
                    if match_type!($length, $mass, $time, $current, $kelvin) {
                        if let Some::<&str>(symbol) = $symbol {
                            return String::from(symbol);
                        }
                    }
                )*
                String::new()
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SiValue<L, M, T, A, K> {
    value: f64,
    _type: PhantomData<(L, M, T, A, K)>,
}

new_types!{
    Distance,              Some("m")   => kg^Z0, m^P1, s^Z0, A^Z0, K^Z0, // m^1
    Area,                  None        => kg^Z0, m^P2, s^Z0, A^Z0, K^Z0, // m^2
    Volume,                None        => kg^Z0, m^P3, s^Z0, A^Z0, K^Z0, // m^3
    Time,                  Some("s")   => kg^Z0, m^Z0, s^P1, A^Z0, K^Z0, // s^1
    Frequency,             Some("Hz")  => kg^Z0, m^Z0, s^N1, A^Z0, K^Z0, // s^-1
    Velocity,              None        => kg^Z0, m^P1, s^N1, A^Z0, K^Z0, // m^1 s^-1
    Acceleration,          None        => kg^Z0, m^P1, s^N2, A^Z0, K^Z0, // m^1 s^-2
    Mass,                  Some("kg")  => kg^P1, m^Z0, s^Z0, A^Z0, K^Z0, // kg^1
    Force,                 Some("N")   => kg^P1, m^P1, s^N2, A^Z0, K^Z0, // kg^1 m^1 s^-2
    Torque,                None        => kg^P1, m^P2, s^N2, A^Z0, K^Z0, // kg^1 m^2 s^-2
    Energy,                Some("J")   => kg^P1, m^P2, s^N2, A^Z0, K^Z0, // kg^1 m^2 s^-2
    Power,                 Some("W")   => kg^P1, m^P2, s^N3, A^Z0, K^Z0, // kg^1 m^2 s^-3
    Momentum,              Some("N·s") => kg^P1, m^P1, s^N1, A^Z0, K^Z0, // kg^1 m^1 s^-1
    Pressure,              Some("Pa")  => kg^P1, m^N1, s^N2, A^Z0, K^Z0, // kg^1 m^-1 s^-2
    Radian,                None        => kg^Z0, m^Z0, s^Z0, A^Z0, K^Z0, // dimensionless
    AngularVelocity,       None        => kg^Z0, m^Z0, s^N1, A^Z0, K^Z0, // s^-1
    AngularAcceleration,   None        => kg^Z0, m^Z0, s^N2, A^Z0, K^Z0, // s^-2
    Current,               Some("A")   => kg^Z0, m^Z0, s^Z0, A^P1, K^Z0, // A^1
    Charge,                Some("C")   => kg^Z0, m^Z0, s^P1, A^P1, K^Z0, // A^1 s^1
    Voltage,               Some("V")   => kg^P1, m^P2, s^N3, A^N1, K^Z0, // kg^1 m^2 s^-3 A^-1
    Resistance,            Some("Ω")   => kg^P1, m^P2, s^N3, A^N2, K^Z0, // kg^1 m^2 s^-3 A^-2
    Conductance,           Some("S")   => kg^N1, m^N2, s^P3, A^P2, K^Z0, // kg^-1 m^-2 s^3 A^2
    Capacitance,           Some("F")   => kg^N1, m^N2, s^P4, A^N2, K^Z0, // kg^-1 m^-2 s^4 A^2
    Inductance,            Some("H")   => kg^P1, m^P2, s^N2, A^N2, K^Z0, // kg^1 m^2 s^-2 A^-2
    MagneticFlux,          Some("Wb")  => kg^P1, m^P2, s^N2, A^N1, K^Z0, // kg^1 m^2 s^-2 A^-1
    MagneticFieldStrength, Some("T")   => kg^P1, m^Z0, s^N2, A^N1, K^Z0, // kg^1 s^-2 A^-1
    MagneticPermeability,  Some("H/m") => kg^P1, m^P1, s^N2, A^N2, K^Z0, // kg^1 m^-1 s^-2 A^-2
    Temperature,           Some("K")   => kg^Z0, m^Z0, s^Z0, A^Z0, K^P1, // K^1
    HeatCapacity,          None        => kg^Z0, m^P2, s^N2, A^Z0, K^N1, // kg^2 s^-2 K^-1 
    SpecificHeatCapacity,  None        => kg^N1, m^P2, s^N2, A^Z0, K^N1, // m^2 s^-2 K^-1
    ThermalConductivity,   None        => kg^P1, m^P1, s^N3, A^Z0, K^N1, // kg^1 m^1 s^-3 K^-1
    ThermalExpansionCoefficient, None  => kg^Z0, m^Z0, s^Z0, A^Z0, K^N1, // K^-1
    HeatFluxDensity,       None        => kg^P1, m^Z0, s^N3, A^Z0, K^Z0, // kg^1 s^-3
}

impl_unit_conversions!(
    SpecificHeatCapacity {
        joules_per_kilogram_kelvin , as_joules_per_kilogram_kelvin => 1.0,
        kilojoules_per_kilogram_kelvin , as_kilojoules_per_kilogram_kelvin => 1e3,
        => constants {
            WATER_4C , 4184.0,
            AIR_20C , 1005.0,
            STEEL , 490.0, 
            ALUMINIUM , 897.0, 
        }
    }
    ThermalExpansionCoefficient {
        per_kelvin , as_per_kelvin => 1.0,
        per_celsius , as_per_celsius => 1.0
    }
    HeatFluxDensity {
        watts_per_square_meter , as_watts_per_square_meter => 1.0,
        kilowatts_per_square_meter , as_kilowatts_per_square_meter => 1e3
    }
    Pressure {
        pascals , as_pascals => 1.0,
        kilopascals , as_kilopascals => 1e3,
        megapascals , as_megapascals => 1e6,
        bars , as_bars => 1e5,
    }
    Temperature {
        kelvins , as_kelvins => 1.0,
        celsius , as_celsius => 1.0 => offset = 273.15,
    }
    ThermalConductivity {
        watts_per_meter_kelvin , as_watts_per_meter_kelvin => 1.0,
        milliwatts_per_meter_kelvin , as_milliwatts_per_meter_kelvin => 1e-3
    }
    HeatCapacity {
        joules_per_kelvin , as_joules_per_kelvin => 1.0,
        kilojoules_per_kelvin , as_kilojoules_per_kelvin => 1e3
    }
    Radian {
        radians , as_radians => 1.0,
        degrees , as_degrees => std::f64::consts::PI / 180.0
    }
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
        kilocalories , as_kilocalories => 4.184e3,
        watt_seconds , as_watt_seconds => 1.0,
        watt_hours , as_watt_hours => 3600.0,
        kilowatt_hours , as_kilowatt_hours => 3.6e6,
        gigajoules , as_gigajoules => 1e9,
        gigawatt_hours , as_gigawatt_hours => 3.6e12
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
    Velocity {
        meters_per_second , as_meters_per_second => 1.0,
        kilometers_per_hour , as_kilometers_per_hour => 1000.0 / 3600.0,
        => constants {
            SPEED_OF_LIGHT, 299792458.0
        }
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
        meganewtons , as_meganewtons => 1e6,
        => constants {
            EARTH_GRAVITY, 9.80665,
            MOON_GRAVITY, 1.625,
        }
    }
);

impl<L, M, T, A, K> SiValue<L, M, T, A, K> {
    const fn new(value: f64) -> Self {
        Self {
            value,
            _type: PhantomData,
        }
    }

    pub fn as_value_in_base_units(&self) -> f64 {
        self.value
    }
}

impl<L, M, T, A, K> SiValue<L, M, T, A, K>
where
    L: Integer + Neg,
    M: Integer + Neg,
    T: Integer + Neg,
    A: Integer + Neg,
    K: Integer + Neg,
{
    pub fn inverse(self) -> SiValue<Negate<L>, Negate<M>, Negate<T>, Negate<A>, Negate<K>> {
        SiValue::new(1.0 / self.value)
    }
}

impl<L, M, T, A, K> SiValue<L, M, T, A, K>
where
    L: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    M: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    T: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    A: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    K: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
{
    pub fn sqrt(self) -> SiValue<
        typenum::Quot<L, typenum::P2>,
        typenum::Quot<M, typenum::P2>,
        typenum::Quot<T, typenum::P2>,
        typenum::Quot<A, typenum::P2>,
        typenum::Quot<K, typenum::P2>,
    > {
        SiValue::new(self.value.sqrt())
    }
}

impl<L, M, T, A, K> Add for SiValue<L, M, T, A, K> {
    type Output = SiValue<L, M, T, A, K>;

    fn add(self, rhs: SiValue<L, M, T, A, K>) -> Self::Output {
        SiValue::new(self.value + rhs.value)
    }
}

impl<L, M, T, A, K> AddAssign for SiValue<L, M, T, A, K> {
    fn add_assign(&mut self, rhs: SiValue<L, M, T, A, K>) {
        self.value += rhs.value;
    }
}

impl<L, M, T, A, K> Sub for SiValue<L, M, T, A, K> {
    type Output = SiValue<L, M, T, A, K>;

    fn sub(self, rhs: SiValue<L, M, T, A, K>) -> Self::Output {
        SiValue::new(self.value - rhs.value)
    }
}

impl<L, M, T, A, K> SubAssign for SiValue<L, M, T, A, K> {
    fn sub_assign(&mut self, rhs: SiValue<L, M, T, A, K>) {
        self.value -= rhs.value;
    }
}

impl<L1, M1, T1, A1, K1, L2, M2, T2, A2, K2> Mul<SiValue<L2, M2, T2, A2, K2>> for SiValue<L1, M1, T1, A1, K1>
where
    L1: Integer + std::ops::Add<L2>,
    M1: Integer + std::ops::Add<M2>,
    T1: Integer + std::ops::Add<T2>,
    A1: Integer + std::ops::Add<A2>,
    K1: Integer + std::ops::Add<K2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
{
    type Output = SiValue<Sum<L1, L2>, Sum<M1, M2>, Sum<T1, T2>, Sum<A1, A2>, Sum<K1, K2>>;

    fn mul(self, rhs: SiValue<L2, M2, T2, A2, K2>) -> Self::Output {
        SiValue::new(self.value * rhs.value)
    }
}

impl<L1, M1, T1, A1, K1, L2, M2, T2, A2, K2> Div<SiValue<L2, M2, T2, A2, K2>> for SiValue<L1, M1, T1, A1, K1>
where
    L1: Integer + std::ops::Sub<L2>,
    M1: Integer + std::ops::Sub<M2>,
    T1: Integer + std::ops::Sub<T2>,
    A1: Integer + std::ops::Sub<A2>,
    K1: Integer + std::ops::Sub<K2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
{
    type Output = SiValue<Diff<L1, L2>, Diff<M1, M2>, Diff<T1, T2>, Diff<A1, A2>, Diff<K1, K2>>;

    fn div(self, rhs: SiValue<L2, M2, T2, A2, K2>) -> Self::Output {
        SiValue::new(self.value / rhs.value)
    }
}

impl<L, M, T, A, K> Mul<f64> for SiValue<L, M, T, A, K> {
    type Output = SiValue<L, M, T, A, K>;

    fn mul(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value * rhs)
    }
}

impl<L, M, T, A, K> Mul<SiValue<L, M, T, A, K>> for f64 {
    type Output = SiValue<L, M, T, A, K>;

    fn mul(self, rhs: SiValue<L, M, T, A, K>) -> Self::Output {
        SiValue::new(self * rhs.value)
    }
}

impl<L, M, T, A, K> MulAssign<f64> for SiValue<L, M, T, A, K> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

impl<L, M, T, A, K> Div<f64> for SiValue<L, M, T, A, K> {
    type Output = SiValue<L, M, T, A, K>;

    fn div(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value / rhs)
    }
}

impl<L, M, T, A, K> Div<SiValue<L, M, T, A, K>> for f64 
where 
    L: Integer + Neg,
    M: Integer + Neg,
    T: Integer + Neg,
    A: Integer + Neg,
    K: Integer + Neg,
{
    type Output = SiValue<Negate<L>, Negate<M>, Negate<T>, Negate<A>, Negate<K>>;

    fn div(self, rhs: SiValue<L, M, T, A, K>) -> Self::Output {
        SiValue::new(self / rhs.value)
    }
}

impl<L, M, T, A, K> DivAssign<f64> for SiValue<L, M, T, A, K> {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}

impl<L, M, T, A, K> fmt::Display for SiValue<L, M, T, A, K>
where
    L: Integer,
    M: Integer,
    T: Integer,
    A: Integer,
    K: Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Check for known types and use their symbols
        let unit_str = self.unit_str();

        if f64::abs(self.value) < 1e-4 || f64::abs(self.value) >= 1e6 {
            write!(f, "{:.5e}{}", self.value, unit_str)
        } else {
            write!(f, "{}{}", self.value, unit_str)
        }
    }
}

impl<L, M, T, A, K> SiValue<L, M, T, A, K>
where
    L: Integer,
    M: Integer,
    T: Integer,
    A: Integer,
    K: Integer,
{
    pub fn unit_str(&self) -> String {
        let mut unit_str = self.unit_symbol();
        if unit_str.is_empty() {
            let l_exp = L::to_i64();
            let m_exp = M::to_i64();
            let t_exp = T::to_i64();
            let a_exp = A::to_i64();
            let k_exp = K::to_i64();

            let mut numerator = Vec::new();
            let mut denominator = Vec::new();

            if m_exp > 0 {
                numerator.push(format_unit("kg", m_exp));
            } else if m_exp < 0 {
                denominator.push(format_unit("kg", -m_exp));
            }

            if l_exp > 0 {
                numerator.push(format_unit("m", l_exp));
            } else if l_exp < 0 {
                denominator.push(format_unit("m", -l_exp));
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

            if k_exp > 0 {
                numerator.push(format_unit("K", k_exp));
            } else if k_exp < 0 {
                denominator.push(format_unit("K", -k_exp));
            }

            unit_str = match (numerator.len(), denominator.len()) {
                (0, 0) => "".to_string(),
                (_, 0) => format!(" [{}]", numerator.join("·")),
                (0, 1) => format!(" [1/{}]", denominator.join("·")),
                (0, _) => format!(" [1/({})]", denominator.join("·")),
                (1, 1) => format!(" [{}/{}]", numerator.join("·"), denominator.join("·")),
                (1, _) => format!(" [{}/({})]", numerator.join("·"), denominator.join("·")),
                (_, 1) => format!(" [({})/{}]", numerator.join("·"), denominator.join("·")),
                (_, _) => format!(" [({})/({})]", numerator.join("·"), denominator.join("·")),
            };
        } else {
            unit_str = format!(" [{}]", unit_str);
        }
        unit_str
    }
}

fn format_unit(name: &str, exp: i64) -> String {
    match exp {
        1 => format!("{}", name),
        _ => format!("{}{}", name, to_superscript(exp)),
    }
}

fn to_superscript(n: i64) -> String {
    let digits = n.abs().to_string();
    let mut result = String::new();
    if n < 0 {
        result.push('⁻');
    }
    for c in digits.chars() {
        result.push(match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => c,
        });
    }
    result
}