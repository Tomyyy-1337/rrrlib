use core::fmt;
use std::{marker::PhantomData};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use typenum::{Diff, Integer, Negate, Sum, N1, N2, N3, P1, P2, P3, P4, Z0};

use crate::si_units::si_macros::{impl_unit_conversions, new_types};

/// A value with associated SI unit dimensions. 
/// Unit dimensions are checked at compile time preventing unit mismatch errors.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SiValue<Meter, Kg, Second, Ampere, Kelvin, Mol, Candela> {
    value: f64,
    _type: PhantomData<(Meter, Kg, Second, Ampere, Kelvin, Mol, Candela)>,
}

new_types!{
    Scalar,                None        => kg^Z0, m^Z0, s^Z0, A^Z0, K^Z0, mol^Z0, cd^Z0, // dimensionless
    Distance,              Some("m")   => kg^Z0, m^P1, s^Z0, A^Z0, K^Z0, mol^Z0, cd^Z0, // m^1
    Area,                  None        => kg^Z0, m^P2, s^Z0, A^Z0, K^Z0, mol^Z0, cd^Z0, // m^2
    Volume,                None        => kg^Z0, m^P3, s^Z0, A^Z0, K^Z0, mol^Z0, cd^Z0, // m^3
    Time,                  Some("s")   => kg^Z0, m^Z0, s^P1, A^Z0, K^Z0, mol^Z0, cd^Z0, // s^1
    Frequency,             None        => kg^Z0, m^Z0, s^N1, A^Z0, K^Z0, mol^Z0, cd^Z0, // s^-1
    Velocity,              None        => kg^Z0, m^P1, s^N1, A^Z0, K^Z0, mol^Z0, cd^Z0, // m^1 s^-1
    Acceleration,          None        => kg^Z0, m^P1, s^N2, A^Z0, K^Z0, mol^Z0, cd^Z0, // m^1 s^-2
    Mass,                  Some("kg")  => kg^P1, m^Z0, s^Z0, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1
    Force,                 Some("N")   => kg^P1, m^P1, s^N2, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1 m^1 s^-2
    Torque,                None        => kg^P1, m^P2, s^N2, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1 m^2 s^-2
    Energy,                Some("J")   => kg^P1, m^P2, s^N2, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1 m^2 s^-2
    Power,                 Some("W")   => kg^P1, m^P2, s^N3, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1 m^2 s^-3
    Momentum,              Some("N·s") => kg^P1, m^P1, s^N1, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1 m^1 s^-1
    Pressure,              Some("Pa")  => kg^P1, m^N1, s^N2, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1 m^-1 s^-2
    Radian,                None        => kg^Z0, m^Z0, s^Z0, A^Z0, K^Z0, mol^Z0, cd^Z0, // dimensionless
    AngularVelocity,       None        => kg^Z0, m^Z0, s^N1, A^Z0, K^Z0, mol^Z0, cd^Z0, // s^-1
    AngularAcceleration,   None        => kg^Z0, m^Z0, s^N2, A^Z0, K^Z0, mol^Z0, cd^Z0, // s^-2
    Current,               Some("A")   => kg^Z0, m^Z0, s^Z0, A^P1, K^Z0, mol^Z0, cd^Z0, // A^1
    Charge,                Some("C")   => kg^Z0, m^Z0, s^P1, A^P1, K^Z0, mol^Z0, cd^Z0, // A^1 s^1
    Voltage,               Some("V")   => kg^P1, m^P2, s^N3, A^N1, K^Z0, mol^Z0, cd^Z0, // kg^1 m^2 s^-3 A^-1
    Resistance,            Some("Ω")   => kg^P1, m^P2, s^N3, A^N2, K^Z0, mol^Z0, cd^Z0, // kg^1 m^2 s^-3 A^-2
    Conductance,           Some("S")   => kg^N1, m^N2, s^P3, A^P2, K^Z0, mol^Z0, cd^Z0, // kg^-1 m^-2 s^3 A^2
    Capacitance,           Some("F")   => kg^N1, m^N2, s^P4, A^N2, K^Z0, mol^Z0, cd^Z0, // kg^-1 m^-2 s^4 A^2
    Inductance,            Some("H")   => kg^P1, m^P2, s^N2, A^N2, K^Z0, mol^Z0, cd^Z0, // kg^1 m^2 s^-2 A^-2
    MagneticFlux,          Some("Wb")  => kg^P1, m^P2, s^N2, A^N1, K^Z0, mol^Z0, cd^Z0, // kg^1 m^2 s^-2 A^-1
    MagneticFieldStrength, Some("T")   => kg^P1, m^Z0, s^N2, A^N1, K^Z0, mol^Z0, cd^Z0, // kg^1 s^-2 A^-1
    MagneticPermeability,  Some("H/m") => kg^P1, m^P1, s^N2, A^N2, K^Z0, mol^Z0, cd^Z0, // kg^1 m^-1 s^-2 A^-2
    Temperature,           Some("K")   => kg^Z0, m^Z0, s^Z0, A^Z0, K^P1, mol^Z0, cd^Z0, // K^1
    HeatCapacity,          None        => kg^Z0, m^P2, s^N2, A^Z0, K^N1, mol^Z0, cd^Z0, // kg^2 s^-2 K^-1 
    SpecificHeatCapacity,  None        => kg^N1, m^P2, s^N2, A^Z0, K^N1, mol^Z0, cd^Z0, // m^2 s^-2 K^-1
    ThermalConductivity,   None        => kg^P1, m^P1, s^N3, A^Z0, K^N1, mol^Z0, cd^Z0, // kg^1 m^1 s^-3 K^-1
    ThermalExpansionCoefficient, None  => kg^Z0, m^Z0, s^Z0, A^Z0, K^N1, mol^Z0, cd^Z0, // K^-1
    HeatFluxDensity,       None        => kg^P1, m^Z0, s^N3, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1 s^-3
    Mole,                  Some("mol") => kg^Z0, m^Z0, s^Z0, A^Z0, K^Z0, mol^P1, cd^Z0, // mol^1
    MolarMass,             None        => kg^P1, m^Z0, s^Z0, A^Z0, K^Z0, mol^N1, cd^Z0, // kg^1 mol^-1
    MolarConcentration,    None        => kg^Z0, m^N3, s^Z0, A^Z0, K^Z0, mol^P1, cd^Z0, // m^-3 mol^1
    MolarHeatCapacity,     None        => kg^P1, m^P2, s^N2, A^Z0, K^N1, mol^N1, cd^Z0, // m^2 s^-2 K^-1 mol^-1
    CatalyticActivity,     None        => kg^Z0, m^Z0, s^N1, A^Z0, K^Z0, mol^P1, cd^Z0, // s^-1 mol^1
    LuminousIntensity,     Some("cd")  => kg^Z0, m^Z0, s^Z0, A^Z0, K^Z0, mol^Z0, cd^P1, // cd^1
    LuminousFlux,          None        => kg^Z0, m^Z0, s^Z0, A^Z0, K^Z0, mol^Z0, cd^P1, // cd^1 * sr (steradian, dimensionless)
    Luminance,             Some("lx")  => kg^Z0, m^N2, s^Z0, A^Z0, K^Z0, mol^Z0, cd^P1, // cd^1 * sr / m^2
    Density,               None        => kg^P1, m^N3, s^Z0, A^Z0, K^Z0, mol^Z0, cd^Z0, // kg^1 m^-3
}

impl_unit_conversions!(
    Scalar {
        scalar , as_scalar => 1.0
    }
    Density {
        kilograms_per_cubic_meter , as_kilograms_per_cubic_meter => 1.0,
        grams_per_cubic_centimeter , as_grams_per_cubic_centimeter => 1e3,
        grams_per_liter , as_grams_per_liter => 1.0,
        pounds_per_cubic_foot , as_pounds_per_cubic_foot => 16.0185,
        pounds_per_gallon , as_pounds_per_gallon => 119.826
    }
    Luminance {
        lux , as_lux => 1.0,
        millilux , as_millilux => 1e-3,
        kilolux , as_kilolux => 1e3
    }
    LuminousIntensity {
        candelas , as_candelas => 1.0,
        millicandelas , as_millicandelas => 1e-3,
        kilocandelas , as_kilocandelas => 1e3
    }
    LuminousFlux {
        lumens , as_lumens => 1.0,
        millilumens , as_millilumens => 1e-3,
        kilolumens , as_kilolumens => 1e3
    }
    Mole {
        moles , as_moles => 1.0,
        millimoles , as_millimoles => 1e-3,
        kilomoles , as_kilomoles => 1e3
    }
    MolarMass {
        kilograms_per_mole , as_kilograms_per_mole => 1.0,
        grams_per_mole , as_grams_per_mole => 1e-3,
        milligrams_per_mole , as_milligrams_per_mole => 1e-6
    }
    MolarConcentration {
        moles_per_cubic_meter , as_moles_per_cubic_meter => 1.0,
        millimoles_per_cubic_meter , as_millimoles_per_cubic_meter => 1e-3,
        kilomoles_per_cubic_meter , as_kilomoles_per_cubic_meter => 1e3,
        moles_per_liter , as_moles_per_liter => 1e3,
        millimoles_per_liter , as_millimoles_per_liter => 1.0,
        kilomoles_per_liter , as_kilomoles_per_liter => 1e6
    }
    MolarHeatCapacity {
        joules_per_mole_kelvin , as_joules_per_mole_kelvin => 1.0,
        kilojoules_per_mole_kelvin , as_kilojoules_per_mole_kelvin => 1e3,
        calories_per_mole_kelvin , as_calories_per_mole_kelvin => 4.184,
        kilocalories_per_mole_kelvin , as_kilocalories_per_mole_kelvin => 4.184e3
    }
    CatalyticActivity {
        katal , as_katal => 1.0,
        millikatal , as_millikatal => 1e-3,
        microkatal , as_microkatal => 1e-6
    }
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

impl Add<f64> for SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0> {
    type Output = SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

    fn add(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value + rhs)
    }
}

impl Add<f64> for &SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0> {
    type Output = SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

    fn add(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value + rhs)
    }
}

impl Add<SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>> for f64 {
    type Output = SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

    fn add(self, rhs: SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>) -> Self::Output {
        SiValue::new(self + rhs.value)
    }
}

impl Add<&SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>> for f64 {
    type Output = SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

    fn add(self, rhs: &SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>) -> Self::Output {
        SiValue::new(self + rhs.value)
    }
}   

impl AddAssign<f64> for SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0> {
    fn add_assign(&mut self, rhs: f64) {
        self.value += rhs;
    }
}

impl Sub<f64> for SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0> {
    type Output = SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

    fn sub(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value - rhs)
    }
}

impl Sub<f64> for &SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0> {
    type Output = SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

    fn sub(self, rhs: f64) -> Self::Output {
        SiValue::new(self.value - rhs)
    }
}

impl Sub<SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>> for f64 {
    type Output = SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

    fn sub(self, rhs: SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>) -> Self::Output {
        SiValue::new(self - rhs.value)
    }
}

impl Sub<&SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>> for f64 {
    type Output = SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

    fn sub(self, rhs: &SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0>) -> Self::Output {
        SiValue::new(self - rhs.value)
    }
}

impl PartialEq<f64> for SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0> {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl PartialOrd<f64> for SiValue<Z0, Z0, Z0, Z0, Z0, Z0, Z0> {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<L, M, T, A, K, Mol, Cd> SiValue<L, M, T, A, K, Mol, Cd> {
    const fn new(value: f64) -> Self {
        Self {
            value,
            _type: PhantomData,
        }
    }

    pub const ZERO: Self = SiValue::new(0.0);

    pub fn as_value_in_base_units(&self) -> f64 {
        self.value
    }
}

impl<A: Eq, B: Eq, C: Eq, D: Eq, E: Eq, F: Eq, G: Eq> Eq for SiValue<A, B, C, D, E, F, G> {}

impl<A, B, C, D, E, F, G> Ord for SiValue<A, B, C, D, E, F, G> 
where
    A: PartialOrd + Eq,
    B: PartialOrd + Eq,
    C: PartialOrd + Eq,
    D: PartialOrd + Eq,
    E: PartialOrd + Eq,
    F: PartialOrd + Eq,
    G: PartialOrd + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.partial_cmp(&other.value).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl<L, M, T, A, K, Mol, Cd> SiValue<L, M, T, A, K, Mol, Cd>
where
    L: Integer + Neg,
    M: Integer + Neg,
    T: Integer + Neg,
    A: Integer + Neg,
    K: Integer + Neg,
    Mol: Integer + Neg,
    Cd: Integer + Neg,
{
    pub fn inverse(&self) -> SiValue<Negate<L>, Negate<M>, Negate<T>, Negate<A>, Negate<K>, Negate<Mol>, Negate<Cd>> {
        SiValue::new(1.0 / self.value)
    }
}

impl<L, M, T, A, K, Mol, Cd> SiValue<L, M, T, A, K, Mol, Cd>
where
    L: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    M: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    T: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    A: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    K: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    Mol: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
    Cd: Integer + std::ops::Div<typenum::P2> + Rem<P2, Output = Z0>,
{
    pub fn sqrt(&self) -> SiValue<
        typenum::Quot<L, typenum::P2>,
        typenum::Quot<M, typenum::P2>,
        typenum::Quot<T, typenum::P2>,
        typenum::Quot<A, typenum::P2>,
        typenum::Quot<K, typenum::P2>,
        typenum::Quot<Mol, typenum::P2>,
        typenum::Quot<Cd, typenum::P2>,
    > {
        SiValue::new(self.value.sqrt())
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> Add<Rhs> for SiValue<L, M, T, A, K, Mol, Cd>
where
    Rhs: std::borrow::Borrow<SiValue<L, M, T, A, K, Mol, Cd>>,
{
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn add(self, rhs: Rhs) -> Self::Output {
        SiValue::new(self.value + rhs.borrow().value)
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> Add<Rhs> for &SiValue<L, M, T, A, K, Mol, Cd>
where
    Rhs: std::borrow::Borrow<SiValue<L, M, T, A, K, Mol, Cd>>,
{
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn add(self, rhs: Rhs) -> Self::Output {
        SiValue::new(self.value + rhs.borrow().value)
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> AddAssign<Rhs> for SiValue<L, M, T, A, K, Mol, Cd> 
where
    Rhs: std::borrow::Borrow<SiValue<L, M, T, A, K, Mol, Cd>>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        self.value += rhs.borrow().value;
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> Sub<Rhs> for SiValue<L, M, T, A, K, Mol, Cd>
where
    Rhs: std::borrow::Borrow<SiValue<L, M, T, A, K, Mol, Cd>>,
{
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        SiValue::new(self.value - rhs.borrow().value)
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> Sub<Rhs> for &SiValue<L, M, T, A, K, Mol, Cd>
where
    Rhs: std::borrow::Borrow<SiValue<L, M, T, A, K, Mol, Cd>>,
{
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        SiValue::new(self.value - rhs.borrow().value)
    }
}

impl <L, M, T, A, K, Mol, Cd, Rhs> SubAssign<Rhs> for SiValue<L, M, T, A, K, Mol, Cd> 
where
    Rhs: std::borrow::Borrow<SiValue<L, M, T, A, K, Mol, Cd>>,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        self.value -= rhs.borrow().value;
    }
}

impl<L1, M1, T1, A1, K1, Mol1, Cd1, L2, M2, T2, A2, K2, Mol2, Cd2> Mul<SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>> for SiValue<L1, M1, T1, A1, K1, Mol1, Cd1>
where
    L1: Integer + std::ops::Add<L2>,
    M1: Integer + std::ops::Add<M2>,
    T1: Integer + std::ops::Add<T2>,
    A1: Integer + std::ops::Add<A2>,
    K1: Integer + std::ops::Add<K2>,
    Mol1: Integer + std::ops::Add<Mol2>,
    Cd1: Integer + std::ops::Add<Cd2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
    Mol2: Integer,
    Cd2: Integer,
{
    type Output = SiValue<Sum<L1, L2>, Sum<M1, M2>, Sum<T1, T2>, Sum<A1, A2>, Sum<K1, K2>, Sum<Mol1, Mol2>, Sum<Cd1, Cd2>>;

    fn mul(self, rhs: SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SiValue::new(self.value * rhs.value)
    }
}

impl<L1, M1, T1, A1, K1, Mol1, Cd1, L2, M2, T2, A2, K2, Mol2, Cd2> Mul<&SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>> for SiValue<L1, M1, T1, A1, K1, Mol1, Cd1>
where
    L1: Integer + std::ops::Add<L2>,
    M1: Integer + std::ops::Add<M2>,
    T1: Integer + std::ops::Add<T2>,
    A1: Integer + std::ops::Add<A2>,
    K1: Integer + std::ops::Add<K2>,
    Mol1: Integer + std::ops::Add<Mol2>,
    Cd1: Integer + std::ops::Add<Cd2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
    Mol2: Integer,
    Cd2: Integer,
{
    type Output = SiValue<Sum<L1, L2>, Sum<M1, M2>, Sum<T1, T2>, Sum<A1, A2>, Sum<K1, K2>, Sum<Mol1, Mol2>, Sum<Cd1, Cd2>>;

    fn mul(self, rhs: &SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SiValue::new(self.value * rhs.value)
    }
}

impl<L1, M1, T1, A1, K1, Mol1, Cd1, L2, M2, T2, A2, K2, Mol2, Cd2> Mul<SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>> for &SiValue<L1, M1, T1, A1, K1, Mol1, Cd1>
where
    L1: Integer + std::ops::Add<L2>,
    M1: Integer + std::ops::Add<M2>,
    T1: Integer + std::ops::Add<T2>,
    A1: Integer + std::ops::Add<A2>,
    K1: Integer + std::ops::Add<K2>,
    Mol1: Integer + std::ops::Add<Mol2>,
    Cd1: Integer + std::ops::Add<Cd2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
    Mol2: Integer,
    Cd2: Integer,
{
    type Output = SiValue<Sum<L1, L2>, Sum<M1, M2>, Sum<T1, T2>, Sum<A1, A2>, Sum<K1, K2>, Sum<Mol1, Mol2>, Sum<Cd1, Cd2>>;

    fn mul(self, rhs: SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SiValue::new(self.value * rhs.value)
    }
}

impl<L1, M1, T1, A1, K1, Mol1, Cd1, L2, M2, T2, A2, K2, Mol2, Cd2> Mul<&SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>> for &SiValue<L1, M1, T1, A1, K1, Mol1, Cd1>
where
    L1: Integer + std::ops::Add<L2>,
    M1: Integer + std::ops::Add<M2>,
    T1: Integer + std::ops::Add<T2>,
    A1: Integer + std::ops::Add<A2>,
    K1: Integer + std::ops::Add<K2>,
    Mol1: Integer + std::ops::Add<Mol2>,
    Cd1: Integer + std::ops::Add<Cd2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
    Mol2: Integer,
    Cd2: Integer,
{
    type Output = SiValue<Sum<L1, L2>, Sum<M1, M2>, Sum<T1, T2>, Sum<A1, A2>, Sum<K1, K2>, Sum<Mol1, Mol2>, Sum<Cd1, Cd2>>;

    fn mul(self, rhs: &SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SiValue::new(self.value * rhs.value)
    }
}

impl<L1, M1, T1, A1, K1, Mol1, Cd1, L2, M2, T2, A2, K2, Mol2, Cd2> Div<SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>> for SiValue<L1, M1, T1, A1, K1, Mol1, Cd1>
where
    L1: Integer + std::ops::Sub<L2>,
    M1: Integer + std::ops::Sub<M2>,
    T1: Integer + std::ops::Sub<T2>,
    A1: Integer + std::ops::Sub<A2>,
    K1: Integer + std::ops::Sub<K2>,
    Mol1: Integer + std::ops::Sub<Mol2>,
    Cd1: Integer + std::ops::Sub<Cd2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
    Mol2: Integer,
    Cd2: Integer,
{
    type Output = SiValue<Diff<L1, L2>, Diff<M1, M2>, Diff<T1, T2>, Diff<A1, A2>, Diff<K1, K2>, Diff<Mol1, Mol2>, Diff<Cd1, Cd2>>;

    fn div(self, rhs: SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SiValue::new(self.value / rhs.value)
    }
}

impl<L1, M1, T1, A1, K1, Mol1, Cd1, L2, M2, T2, A2, K2, Mol2, Cd2> Div<&SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>> for SiValue<L1, M1, T1, A1, K1, Mol1, Cd1>
where
    L1: Integer + std::ops::Sub<L2>,
    M1: Integer + std::ops::Sub<M2>,
    T1: Integer + std::ops::Sub<T2>,
    A1: Integer + std::ops::Sub<A2>,
    K1: Integer + std::ops::Sub<K2>,
    Mol1: Integer + std::ops::Sub<Mol2>,
    Cd1: Integer + std::ops::Sub<Cd2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
    Mol2: Integer,
    Cd2: Integer,
{
    type Output = SiValue<Diff<L1, L2>, Diff<M1, M2>, Diff<T1, T2>, Diff<A1, A2>, Diff<K1, K2>, Diff<Mol1, Mol2>, Diff<Cd1, Cd2>>;

    fn div(self, rhs: &SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SiValue::new(self.value / rhs.value)
    }
}

impl<L1, M1, T1, A1, K1, Mol1, Cd1, L2, M2, T2, A2, K2, Mol2, Cd2> Div<SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>> for &SiValue<L1, M1, T1, A1, K1, Mol1, Cd1>
where
    L1: Integer + std::ops::Sub<L2>,
    M1: Integer + std::ops::Sub<M2>,
    T1: Integer + std::ops::Sub<T2>,
    A1: Integer + std::ops::Sub<A2>,
    K1: Integer + std::ops::Sub<K2>,
    Mol1: Integer + std::ops::Sub<Mol2>,
    Cd1: Integer + std::ops::Sub<Cd2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
    Mol2: Integer,
    Cd2: Integer,
{
    type Output = SiValue<Diff<L1, L2>, Diff<M1, M2>, Diff<T1, T2>, Diff<A1, A2>, Diff<K1, K2>, Diff<Mol1, Mol2>, Diff<Cd1, Cd2>>;

    fn div(self, rhs: SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SiValue::new(self.value / rhs.value)
    }
}

impl<L1, M1, T1, A1, K1, Mol1, Cd1, L2, M2, T2, A2, K2, Mol2, Cd2> Div<&SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>> for &SiValue<L1, M1, T1, A1, K1, Mol1, Cd1>
where
    L1: Integer + std::ops::Sub<L2>,
    M1: Integer + std::ops::Sub<M2>,
    T1: Integer + std::ops::Sub<T2>,
    A1: Integer + std::ops::Sub<A2>,
    K1: Integer + std::ops::Sub<K2>,
    Mol1: Integer + std::ops::Sub<Mol2>,
    Cd1: Integer + std::ops::Sub<Cd2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    A2: Integer,
    K2: Integer,
    Mol2: Integer,
    Cd2: Integer,
{
    type Output = SiValue<Diff<L1, L2>, Diff<M1, M2>, Diff<T1, T2>, Diff<A1, A2>, Diff<K1, K2>, Diff<Mol1, Mol2>, Diff<Cd1, Cd2>>;

    fn div(self, rhs: &SiValue<L2, M2, T2, A2, K2, Mol2, Cd2>) -> Self::Output {
        SiValue::new(self.value / rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> Mul<Rhs> for SiValue<L, M, T, A, K, Mol, Cd> 
where
    Rhs: std::borrow::Borrow<f64>,
{
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        SiValue::new(self.value * rhs.borrow())
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> Mul<Rhs> for &SiValue<L, M, T, A, K, Mol, Cd> 
where
    Rhs: std::borrow::Borrow<f64>,
{
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        SiValue::new(self.value * rhs.borrow())
    }
}

impl<L, M, T, A, K, Mol, Cd> Mul<SiValue<L, M, T, A, K, Mol, Cd>> for f64 {
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn mul(self, rhs: SiValue<L, M, T, A, K, Mol, Cd>) -> Self::Output {
        SiValue::new(self * rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd> Mul<SiValue<L, M, T, A, K, Mol, Cd>> for &f64 {
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn mul(self, rhs: SiValue<L, M, T, A, K, Mol, Cd>) -> Self::Output {
        SiValue::new(self * rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd> Mul<&SiValue<L, M, T, A, K, Mol, Cd>> for f64 {
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn mul(self, rhs: &SiValue<L, M, T, A, K, Mol, Cd>) -> Self::Output {
        SiValue::new(self * rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd> Mul<&SiValue<L, M, T, A, K, Mol, Cd>> for &f64 {
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn mul(self, rhs: &SiValue<L, M, T, A, K, Mol, Cd>) -> Self::Output {
        SiValue::new(self * rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> MulAssign<Rhs> for SiValue<L, M, T, A, K, Mol, Cd> 
where 
    Rhs: std::borrow::Borrow<f64>
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.value *= rhs.borrow();
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> Div<Rhs> for SiValue<L, M, T, A, K, Mol, Cd> 
where
    Rhs: std::borrow::Borrow<f64>,
{
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn div(self, rhs: Rhs) -> Self::Output {
        SiValue::new(self.value / rhs.borrow())
    }
}

impl<L, M, T, A, K, Mol, Cd, Rhs> Div<Rhs> for &SiValue<L, M, T, A, K, Mol, Cd> 
where
    Rhs: std::borrow::Borrow<f64>,
{
    type Output = SiValue<L, M, T, A, K, Mol, Cd>;

    fn div(self, rhs: Rhs) -> Self::Output {
        SiValue::new(self.value / rhs.borrow())
    }
}

impl<L, M, T, A, K, Mol, Cd> Div<SiValue<L, M, T, A, K, Mol, Cd>> for f64
where
    L: Integer + Neg,
    M: Integer + Neg,
    T: Integer + Neg,
    A: Integer + Neg,
    K: Integer + Neg,
    Mol: Integer + Neg,
    Cd: Integer + Neg,
{
    type Output = SiValue<Negate<L>, Negate<M>, Negate<T>, Negate<A>, Negate<K>, Negate<Mol>, Negate<Cd>>;

    fn div(self, rhs: SiValue<L, M, T, A, K, Mol, Cd>) -> Self::Output {
        SiValue::new(self / rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd> Div<SiValue<L, M, T, A, K, Mol, Cd>> for &f64
where
    L: Integer + Neg,
    M: Integer + Neg,
    T: Integer + Neg,
    A: Integer + Neg,
    K: Integer + Neg,
    Mol: Integer + Neg,
    Cd: Integer + Neg,
{
    type Output = SiValue<Negate<L>, Negate<M>, Negate<T>, Negate<A>, Negate<K>, Negate<Mol>, Negate<Cd>>;

    fn div(self, rhs: SiValue<L, M, T, A, K, Mol, Cd>) -> Self::Output {
        SiValue::new(self / rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd> Div<&SiValue<L, M, T, A, K, Mol, Cd>> for f64
where
    L: Integer + Neg,
    M: Integer + Neg,
    T: Integer + Neg,
    A: Integer + Neg,
    K: Integer + Neg,
    Mol: Integer + Neg,
    Cd: Integer + Neg,
{
    type Output = SiValue<Negate<L>, Negate<M>, Negate<T>, Negate<A>, Negate<K>, Negate<Mol>, Negate<Cd>>;

    fn div(self, rhs: &SiValue<L, M, T, A, K, Mol, Cd>) -> Self::Output {
        SiValue::new(self / rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd> Div<&SiValue<L, M, T, A, K, Mol, Cd>> for &f64
where
    L: Integer + Neg,
    M: Integer + Neg,
    T: Integer + Neg,
    A: Integer + Neg,
    K: Integer + Neg,
    Mol: Integer + Neg,
    Cd: Integer + Neg,
{
    type Output = SiValue<Negate<L>, Negate<M>, Negate<T>, Negate<A>, Negate<K>, Negate<Mol>, Negate<Cd>>;

    fn div(self, rhs: &SiValue<L, M, T, A, K, Mol, Cd>) -> Self::Output {
        SiValue::new(self / rhs.value)
    }
}

impl<L, M, T, A, K, Mol, Cd> DivAssign<f64> for SiValue<L, M, T, A, K, Mol, Cd> {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}

impl<L, M, T, A, K, Mol, Cd> DivAssign<&f64> for SiValue<L, M, T, A, K, Mol, Cd> {
    fn div_assign(&mut self, rhs: &f64) {
        self.value /= rhs;
    }
}

impl<L, M, T, A, K, Mol, Cd> fmt::Display for SiValue<L, M, T, A, K, Mol, Cd>
where
    L: Integer,
    M: Integer,
    T: Integer,
    A: Integer,
    K: Integer,
    Mol: Integer,
    Cd: Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_str = self.unit_str();

        if f64::abs(self.value) < 1e-4 || f64::abs(self.value) >= 1e6 {
            write!(f, "{:.5e}{}", self.value, unit_str)
        } else {
            write!(f, "{}{}", self.value, unit_str)
        }
    }
}

impl<L, M, T, A, K, Mol, Cd> SiValue<L, M, T, A, K, Mol, Cd>
where
    L: Integer,
    M: Integer,
    T: Integer,
    A: Integer,
    K: Integer,
    Mol: Integer,
    Cd: Integer,
{
    pub fn unit_str(&self) -> String {
        match self.unit_symbol() {
            Some(s) => format!(" [{}]", s),
            None => {
                let l_exp = L::to_i32();
                let m_exp = M::to_i32();
                let t_exp = T::to_i32();
                let a_exp = A::to_i32();
                let k_exp = K::to_i32();
                let mol_exp = Mol::to_i32();
                let cd_exp = Cd::to_i32();
                
                
                let mut numerator = Vec::new();
                let mut denominator = Vec::new();

                let units = [("kg", m_exp), ("m", l_exp), ("s", t_exp), ("A", a_exp), ("K", k_exp), ("mol", mol_exp), ("cd", cd_exp)];
                
                for &(name, exp) in &units {
                    if exp > 0 {
                        numerator.push(format_unit(name, exp));
                    } else if exp < 0 {
                        denominator.push(format_unit(name, -exp));
                    }
                }
                
                match (numerator.len(), denominator.len()) {
                    (0, 0) => "".to_string(),
                    (_, 0) => format!(" [{}]", numerator.join("·")),
                    (0, 1) => format!(" [1/{}]", denominator.join("·")),
                    (0, _) => format!(" [1/({})]", denominator.join("·")),
                    (1, 1) => format!(" [{}/{}]", numerator.join("·"), denominator.join("·")),
                    (1, _) => format!(" [{}/({})]", numerator.join("·"), denominator.join("·")),
                    (_, 1) => format!(" [({})/{}]", numerator.join("·"), denominator.join("·")),
                    (_, _) => format!(" [({})/({})]", numerator.join("·"), denominator.join("·")),
                }
            }
        }
    }
}

fn format_unit(name: &str, exp: i32) -> String {
    match exp {
        1 => format!("{}", name),
        _ => format!("{}{}", name, to_superscript(exp)),
    }
}

fn to_superscript(n: i32) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let length1 = Distance::meters(5.0);
        let length2 = Distance::meters(3.0);
        let result = length1 + length2;
        let result2 = &length1 + &length2;
        let result3 = length1 + &length2;
        let result4 = &length1 + length2;
        assert_eq!(result.as_meters(), 8.0);
        assert_eq!(result2.as_meters(), 8.0);
        assert_eq!(result3.as_meters(), 8.0);
        assert_eq!(result4.as_meters(), 8.0);
    }

    #[test]
    fn test_subtraction() {
        let length1 = Distance::meters(5.0);
        let length2 = Distance::meters(3.0);
        let result = length1 - length2;
        let result2 = &length1 - &length2;
        let result3 = length1 - &length2;
        let result4 = &length1 - length2;
        assert_eq!(result.as_meters(), 2.0);
        assert_eq!(result2.as_meters(), 2.0);
        assert_eq!(result3.as_meters(), 2.0);
        assert_eq!(result4.as_meters(), 2.0);
    }

    #[test]
    fn test_multiplication_and_division() {
        let length = Distance::meters(5.0);
        let time = Time::seconds(2.0);
        let speed = length / time; // m/s
        assert_eq!(speed.as_meters_per_second(), 2.5);

        let area = length * length; // m^2
        assert_eq!(area.as_square_meters(), 25.0);

        let volume = area * length; // m^3
        assert_eq!(volume.as_cubic_meters(), 125.0);

        let force = Force::newtons(10.0);
        let distance = Distance::meters(3.0);
        let work = force * distance; // Joules (kg·m²/s²)
        assert_eq!(work.as_joules(), 30.0);

        let power = work / time; // Watts (kg·m²/s³)
        assert_eq!(power.as_watts(), 15.0);

        let acceleration = Acceleration::meters_per_second_squared(9.81);
        let mass = Mass::kilograms(2.0);
        let force2 = mass * acceleration; // Newtons (kg·m/s²)
        assert_eq!(force2.as_newtons(), 19.62);

        let inverse_speed = 1.0 / speed; // s/m
        assert_eq!(inverse_speed.as_value_in_base_units(), 0.4);
        
        let sqrt_area = area.sqrt(); // m
        assert_eq!(sqrt_area.as_meters(), 5.0);
    }       

    #[test]
    fn test_sqrt() {
        let area = Area::square_meters(16.0);
        let length = area.sqrt();
        assert_eq!(length.as_meters(), 4.0);
    }

    #[test]
    fn test_inverse() {
        let speed = Velocity::meters_per_second(2.0);
        let inverse_speed = speed.inverse();
        assert_eq!(inverse_speed.as_value_in_base_units(), 0.5);

        let time = Time::seconds(4.0);
        let frequency = time.inverse();

        assert_eq!(frequency.as_value_in_base_units(), 0.25);
        assert_eq!(frequency.unit_str(), " [1/s]");

    }


}