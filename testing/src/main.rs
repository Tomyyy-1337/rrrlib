// use std::{thread::park, time::Duration};

// use data_types::prelude::*;
// use rust_ib2c::prelude::*;

mod simulation;
use data_types::prelude::*;
use simulation::*;

mod controll_system;

pub fn main() {

    let max_power = Power::watts(400.0);
    let current_acceleration = Acceleration::meters_per_second_squared(2.0);
    let mass = Mass::kilograms(50.0);
    let current_speed = max_power / (mass * current_acceleration);
    println!("Max Power: {}, Acceleration: {}, Mass: {}, Current Speed: {}", max_power, current_acceleration, mass, current_speed);

    nannou::app(model).update(update).run();

    // let cycle_time = Duration::from_millis(400);
    // BehaviorGroup::<BatterySystem>::with_name("BatterySystem", cycle_time);
    // loop { park(); }
}
