use data_types::prelude::*;
use rust_ib2c::prelude::*;

#[module]
pub struct TurnAway {
    pub par_min_distance: ParameterPort<Distance>,
    pub in_distance: ReceivePort<Distance>,
    pub in_left_distance: ReceivePort<Distance>,
    pub in_right_distance: ReceivePort<Distance>,
    pub out_turn_rate: SendPort<AngularVelocity>,
    obstacle_detected: bool,
}

impl Module for TurnAway {
    fn init() -> Self {
        Self {
            par_min_distance: ParameterPort::with_value(Distance::meters(2.0)),
            obstacle_detected: false,
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        if let Some(distance) = self.in_distance.get() {
            if distance < self.par_min_distance.get() {
                if self.in_left_distance.get_or_default() < self.in_right_distance.get_or_default() {
                    self.out_turn_rate.send(AngularVelocity::radians_per_second(-1.0));
                } else {
                    self.out_turn_rate.send(AngularVelocity::radians_per_second(1.0));
                }
                self.obstacle_detected = true;
            } else {
                self.out_turn_rate.send(AngularVelocity::radians_per_second(0.0));
                self.obstacle_detected = false;
            }
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