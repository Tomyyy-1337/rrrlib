use rust_ib2c::prelude::*;
use data_types::si_units::*;

#[module]
pub struct ConstantVelocity {
    pub par_velocity: ParameterPort<Velocity>,
    pub out_velocity: SendPort<Velocity>,
}

impl Module for ConstantVelocity {
    fn init() -> Self {
        Self {
            par_velocity: ParameterPort::with_value(Velocity::meters_per_second(1.0)),
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        let velocity = self.par_velocity.get();
        self.out_velocity.send(velocity);
    }

    fn target_rating(&self) -> MetaSignal {
        MetaSignal::HIGH
    }   
}