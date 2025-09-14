// use std::{thread::park, time::Duration};

// use data_types::prelude::*;
// use rust_ib2c::prelude::*;

mod simulation;
use data_types::prelude::*;
use simulation::*;

mod controll_system;

pub fn main() {
    nannou::app(model).update(update).run();

    // let cycle_time = Duration::from_millis(400);
    // BehaviorGroup::<BatterySystem>::with_name("BatterySystem", cycle_time);
    // loop { park(); }
}
