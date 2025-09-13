use std::{thread::park, time::Duration};

use data_types::prelude::*;
use rust_ib2c::prelude::*;

pub fn main() {
    let cycle_time = Duration::from_millis(400);

    BehaviorGroup::<BatterySystem>::with_name("BatterySystem", cycle_time);

    loop { park(); }
}

#[group]
struct BatterySystem {}

impl Group for BatterySystem {
    #[spawn]
    fn init(&mut self, cycle_time: Duration) {
        let battery = BehaviorModule::<Battery>::with_name("Battery", cycle_time);
        let light_bulb = BehaviorModule::<LightBulb>::with_name("LightBulb", cycle_time);
        battery.stimulation.connect_to_source(&light_bulb.activity);

        battery.in_resistance.connect_to_source(&light_bulb.out_resistance);
        light_bulb.in_voltage.connect_to_source(&battery.out_voltage);
        light_bulb.in_current.connect_to_source(&battery.out_current);
    }
}

#[module]
struct LightBulb {
    pub in_voltage: ReceivePort<Voltage>,
    pub in_current: ReceivePort<Current>,

    pub out_power: SendPort<Power>,
    pub out_resistance: SendPort<Resistance>,

    internal_resistance: Resistance,
}

impl Module for LightBulb {
    fn init() -> Self {
        Self {
            internal_resistance: Resistance::ohms(20.0),
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        let voltage = self.in_voltage.get_or_default();
        let current = self.in_current.get_or_default();

        let power = voltage * current;

        if power.as_watts() > 1.0 {
            println!("LightBulb on with power: {}", power);
        }

        self.out_power.send(power);
        self.out_resistance.send(self.internal_resistance)
    }

    fn target_rating(&self) -> MetaSignal {
        if self.out_power.get_or_default().as_watts() > 1.0 {
            MetaSignal::HIGH
        } else {
            MetaSignal::LOW
        }
    }
}

#[module]
struct Battery {
    pub in_resistance: ReceivePort<Resistance>,

    pub out_voltage: SendPort<Voltage>,
    pub out_current: SendPort<Current>,

    remaining_capacity: Energy,
    voltage: Voltage,
}

impl Module for Battery {
    fn init() -> Self {
        Self {
            remaining_capacity: Energy::watt_seconds(20.0),
            voltage: Voltage::volts(9.0),
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        if self.in_resistance.get().is_none() {
            return;
        }
        let resistance = self.in_resistance.get_or_default();
        let current = self.voltage / resistance;        
        let delta_time = Time::seconds(self.delta_time.as_secs_f64());
        let used_energy = current * self.voltage * delta_time;
        self.remaining_capacity -= used_energy;
        if self.remaining_capacity < Energy::watt_hours(0.0) {
            self.remaining_capacity = Energy::watt_hours(0.0);
            self.out_voltage.send(Voltage::volts(0.0));
            self.out_current.send(Current::amperes(0.0));
        } else {
            self.out_voltage.send(self.voltage);
            self.out_current.send(current);
        }
        let operation_time = self.remaining_capacity / (current * self.voltage);
        println!("Remaining capacity: {}, Operation time: {}", self.remaining_capacity, operation_time);
        println!("Stimulus: {:?}", self.get_stimulation());
    }

    fn target_rating(&self) -> MetaSignal {
        if self.remaining_capacity.as_watt_seconds() > 0.1 {
            MetaSignal::HIGH
        } else {
            MetaSignal::LOW
        }
    }
}


#[module]
struct CalcAcceleration {
    pub par_mass: ParameterPort<Mass>,

    pub in_force: ReceivePort<Force>,
    pub in_current_speed: ReceivePort<Velocity>,

    pub out_acceleration: SendPort<Acceleration>,
    pub out_required_power: SendPort<Power>,
}

impl Module for CalcAcceleration {
    fn transfere(&mut self) {
        let mass = self.par_mass.get();
        let force = self.in_force.get_or_default();
        let current_speed = self.in_current_speed.get_or_default();

        let acceleration = force / mass;
        let required_power = force * current_speed;

        self.out_acceleration.send(acceleration);
        self.out_required_power.send(required_power);
    }

    fn target_rating(&self) -> MetaSignal {
        MetaSignal::HIGH
    }
}


