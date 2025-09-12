pub mod vector;
pub mod si_units;
pub mod rotations;

pub mod prelude {
    pub use crate::vector::{Vector, Vector2, Vector3, Vector4};
    pub use crate::si_units::{
        SiValue,
        Distance,
        Area,
        Volume,
        Time,
        Frequency,
        Velocity,
        Acceleration,
        Mass,
        Force,
        Energy,
        Power,
        Torque,
        Momentum,
        Pressure,
        Radian,
        AngularVelocity,
        AngularAcceleration,
        Current,
        Charge,
        Voltage,
        Resistance,
        Conductance,
        Capacitance,
        Inductance,
        MagneticFlux,
        MagneticFieldStrength,
        MagneticPermeability,
        Temperature,
        HeatCapacity,
        SpecificHeatCapacity,
        ThermalConductivity,
        ThermalExpansionCoefficient,
        HeatFluxDensity,
    };
}