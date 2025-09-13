use rust_ib2c::prelude::*;
use data_types::si_units::*;

#[module]
pub struct BreakOnObstacle {
    pub par_min_distance: ParameterPort<Distance>,
    pub in_distance: ReceivePort<Distance>,
    pub out_velocity: SendPort<Velocity>,
    obstacle_detected: bool,
}

impl Module for BreakOnObstacle {
    fn init() -> Self {
        Self {
            par_min_distance: ParameterPort::with_value(Distance::meters(1.0)),
            obstacle_detected: false,
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        let distance = self.in_distance.get_or_default();
        if distance < Distance::centimeters(20.0) {
            self.out_velocity.send(Velocity::meters_per_second(0.0));
        } else if distance < *self.par_min_distance.get() {
            let speed_factor = (self.par_min_distance.get() / distance).inverse();
            self.out_velocity.send(Velocity::meters_per_second(1.0) * speed_factor);
            self.obstacle_detected = true;
        } else {
            self.obstacle_detected = false;
        }
    }

    fn target_rating(&self) -> MetaSignal {
        if self.obstacle_detected {
            MetaSignal::HIGH
        } else {
            MetaSignal::LOW
        }
    }   
}

