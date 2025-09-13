use std::{thread::park, time::Duration};

use data_types::prelude::*;
use rust_ib2c::prelude::*;

mod simulation;
use simulation::*;

pub fn main() {
    nannou::app(model).update(update).run();

    // let cycle_time = Duration::from_millis(400);

    // BehaviorGroup::<BatterySystem>::with_name("BatterySystem", cycle_time);

    loop { park(); }
}

#[group]
struct ControlSystem {
    pub in_front_distance_sensor: ReceivePort<Distance>,
    
    pub out_velocity: SendPort<Velocity>,
}

impl Group for ControlSystem {
    #[spawn]
    fn init(&mut self, cycle_time: Duration) {
        let break_on_obstacle = BehaviorModule::<BreakOnObstacle>::with_name("BreakOnObstacle", cycle_time);
        let constant_velocity = BehaviorModule::<ConstantVelocity>::with_name("ConstantVelocity", cycle_time);
        let mut maximum_fusion = MaximumFusion::with_name("MaxFusion", cycle_time);
        
        break_on_obstacle.in_distance.connect_to_source(&self.in_front_distance_sensor);
        
        connect_fusion! {
            maximum_fusion,
            ports: [
                break_on_obstacle.out_velocity,
                constant_velocity.out_velocity,
            ]
        };

        self.out_velocity.connect_to_source(&maximum_fusion.output);   
    }
}

#[module]
struct BreakOnObstacle {
    pub par_min_distance: ParameterPort<Distance>,
    pub in_distance: ReceivePort<Distance>,
    pub out_velocity: SendPort<Velocity>,
    obstacle_detected: bool,
}

impl Module for BreakOnObstacle {
    fn init() -> Self {
        Self {
            par_min_distance: ParameterPort::with_value(Distance::meters(0.5)),
            obstacle_detected: false,
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        let distance = self.in_distance.get_or_default();
        if distance < *self.par_min_distance.get() {
            self.out_velocity.send(Velocity::meters_per_second(0.0));
            println!("Obstacle detected! Distance: {}", distance);
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


#[module]
struct  ConstantVelocity {
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
        let velocity = *self.par_velocity.get();
        self.out_velocity.send(velocity);
    }

    fn target_rating(&self) -> MetaSignal {
        MetaSignal::HIGH
    }   
}