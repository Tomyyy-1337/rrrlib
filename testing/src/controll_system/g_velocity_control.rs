use std::time::Duration;

use rust_ib2c::prelude::*;
use data_types::si_units::*;

use crate::controll_system::*;

#[group]
pub struct ControlSystem {
    pub in_front_distance_sensor: ReceivePort<Distance>,
    pub in_left_distance_sensor: ReceivePort<Distance>,
    pub in_right_distance_sensor: ReceivePort<Distance>,
    
    pub out_velocity: SendPort<Velocity>,
    pub out_turn_rate: SendPort<AngularVelocity>,
}

impl Group for ControlSystem {
    #[spawn]
    fn init(&mut self, cycle_time: Duration) {
        let break_on_obstacle = BehaviorModule::<BreakOnObstacle>::with_name("BreakOnObstacle", cycle_time);
        let constant_velocity = BehaviorModule::<ConstantVelocity>::with_name("ConstantVelocity", cycle_time);
        let mut maximum_fusion = MaximumFusion::with_name("MaxFusion", cycle_time);

        let turn_away = BehaviorModule::<TurnAway>::with_name("TurnAway", cycle_time);
        turn_away.in_distance.connect_to_source(&self.in_front_distance_sensor);
        turn_away.in_left_distance.connect_to_source(&self.in_left_distance_sensor);
        turn_away.in_right_distance.connect_to_source(&self.in_right_distance_sensor);
        self.out_turn_rate.connect_to_source(&turn_away.out_turn_rate);
        
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
