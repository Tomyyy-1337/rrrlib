pub mod position;
pub mod vector;
pub mod si_units;

pub mod prelude {
    pub use crate::position::{Position2D, Position3D, Position4D};
    pub use crate::vector::{Vector, Vector2, Vector3, Vector4};
    pub use crate::si_units::{
        Distance,
        Area,
        Volume,
        Time,
        Frequency,
        Speed,
        Acceleration,
        Mass,
        Force,
        Energy,
        Power,
        Torque,
        Momentum,
        Radian,
        AngularVelocity,
        AngularAcceleration,
        Current,
        Charge,
        Voltage,
        Resistance,
        Capacitance,
        Inductance,
        MagneticFlux,
        MagneticFieldStrength
    };
}