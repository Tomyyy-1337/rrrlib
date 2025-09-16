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
    fn init(&mut self, cycle_time: Duration, parent: &Parent) {
        let mut velocity_control = SpawnGroup!(VelocityControl, "VelocityControl");
        velocity_control.in_front_distance_sensor.connect_to_source(&self.in_front_distance_sensor);
        self.out_velocity.connect_to_source(&velocity_control.out_velocity);   
        
        let turn_away = SpawnModule!(TurnAway,"CurvateControl");
        turn_away.in_distance.connect_to_source(&self.in_front_distance_sensor);
        turn_away.in_left_distance.connect_to_source(&self.in_left_distance_sensor);
        turn_away.in_right_distance.connect_to_source(&self.in_right_distance_sensor);
        self.out_turn_rate.connect_to_source(&turn_away.out_turn_rate);
        
        self.set_characteristic_module(&mut *velocity_control);
    }
}
