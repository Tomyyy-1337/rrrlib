// use std::{thread::park, time::Duration};

// use std::{thread::park, time::Duration};

// use data_types::prelude::*;
// use rust_ib2c::prelude::*;

mod simulation;
use simulation::*;

// use crate::controll_system::ControlSystem;

mod controll_system;

pub fn main() {    
    // let battery_capacity = Energy::kilowatt_hours(100.0);
    // let battery_voltage = Voltage::volts(12.0);
    // let wire_resistance = Resistance::milliohms(50.0);

    // let short_curcuit_current = battery_voltage / wire_resistance;
    // let power = battery_voltage * short_curcuit_current;
    // let battery_runtime: Time = battery_capacity / power;

    // println!("Battery capacity: {}", battery_capacity);
    // println!("Battery voltage: {}", battery_voltage);
    // println!("Wire resistance: {}", wire_resistance);
    // println!("Short curcuit current: {}", short_curcuit_current);
    // println!("Power: {}", power);
    // println!("Battery runtime: {}", battery_runtime);
    
    nannou::app(model).update(update).run();

    // SpawnMainGroup!(ControlSystem, "MainGroup", Duration::from_millis(10));
    // park();

}
