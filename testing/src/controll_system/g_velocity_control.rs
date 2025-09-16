use std::time::Duration;

use rust_ib2c::prelude::*;
use data_types::si_units::*;

use crate::controll_system::{BreakOnObstacle, ConstantVelocity};

#[group]
pub struct VelocityControl {
    pub in_front_distance_sensor: ReceivePort<Distance>,
    
    pub out_velocity: SendPort<Velocity>,
}

impl Group for VelocityControl {
    #[spawn]
    fn init(&mut self, cycle_time: Duration, parent: &Parent) {
        let break_on_obstacle = SpawnModule!(BreakOnObstacle, "BreakOnObstacle");
        break_on_obstacle.in_distance.connect_to_source(&self.in_front_distance_sensor);

        let constant_velocity = SpawnModule!(ConstantVelocity, "ConstantVelocity");
        
        let mut maximum_fusion = SpawnFusion! {
            MaximumFusion,
            "MaximumFusion",
            inputs: [
                break_on_obstacle.out_velocity,
                constant_velocity.out_velocity,
            ]
        };

        self.out_velocity.connect_to_source(&maximum_fusion.output);   

        self.set_characteristic_module(&mut maximum_fusion);
    }
}