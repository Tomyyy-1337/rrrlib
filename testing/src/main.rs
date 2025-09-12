use std::{f64::consts::PI, fmt::Debug, thread::park, time::Duration};

// use data_types::{prelude::*, si_units::{Distance, Frac, Meter, MeterPerSecond, Second, Si, Time}, vector::{self, Vector2}};
use rust_ib2c::prelude::*;
use data_types::{prelude::*, rotations::{self, Rotation2D, Rotation3D}, vector::{self, Vector2, Vector3}};


fn main() {
    // let orientation = Rotation3D::from_euler_angles(0.0, 1.0, PI/2.0);
    // let orientation2 = Rotation3D::from_euler_angles(0.0, -1.0, -PI/2.0);
    // let vector = Vector3::new(1.0, 0.0, 0.0);
    // let rotated_vector = orientation.rotate_vector(vector);
    // let back_vector = orientation2.rotate_vector(rotated_vector);
    // println!("Rotated Vector: {:?}", rotated_vector);
    // println!("Back Vector: {:?}", back_vector);
    let battery_energy = Energy::watt_hours(50.0);
    let motor_power = Power::watts(360.0);

    let operation_time = battery_energy / motor_power;
    println!("With a battery energy of {} and motor power of {}, the operation time is {}", battery_energy, motor_power, operation_time);
    let motor_voltage = Voltage::volts(36.0);
    let motor_current = motor_power / motor_voltage;
    let motor_resistance = motor_voltage / motor_current;
    let efficiency = 0.5;
    let loss = motor_power * (1.0 - efficiency);
    let heat_capacity = HeatCapacity::joules_per_kelvin(100.0);
    let mass = Mass::kilograms(2.0);
    let temperature_rise = loss / heat_capacity;
    let temperature_rise_per_second = loss / (mass * heat_capacity);
    println!("Motor Voltage: {}, Current: {}, Resistance: {}", motor_voltage, motor_current, motor_resistance);
    println!("Motor Power: {}, Loss: {}, Heat Capacity: {}, Temperature Rise: {}, Temperature Rise: {}", motor_power, loss, heat_capacity, temperature_rise, temperature_rise_per_second);
    let delta_t = Time::seconds(10.0);
    let total_temperature_rise = temperature_rise_per_second * delta_t;
    println!("Total Temperature Rise after {}: {}", delta_t, total_temperature_rise);

    let temperature = Temperature::celsius(25.0);
    let c_val = temperature.as_celsius();
    let k_val = temperature.as_kelvins();
    println!("Temperature: {} °C = {} K", c_val, k_val);

    let value = SiValue::meters(10.0);
    let value2 = SiValue::seconds(10.0);
    let frac = value / value2;
    println!("Value: {}, Value2: {}, Frac: {}", value, value2, frac);


    // voltage of lightvulb
    let lightbulb_voltage = Voltage::volts(12.0);
    let lightbulb_current = Current::amperes(2.0);
    let lightbulb_resistance = lightbulb_voltage / lightbulb_current;
    let lightbulb_power = lightbulb_voltage * lightbulb_current;
    let energy_per_hours = lightbulb_power * Time::hours(1.0);
    println!("Lightbulb Voltage: {}, Current: {}, Resistance: {}, Power: {}, Energy per hour: {}", 
        lightbulb_voltage, lightbulb_current, lightbulb_resistance, lightbulb_power, energy_per_hours);

    let length = Distance::meters(10.0);
    let time = Time::seconds(9.0);
    let speed = length / time;
    println!("Length: {}, Time: {}, Speed: {}", length, time, speed);
    let force = Force::newtons(10.0);
    println!("Force {}, inverse: {}, inverse_inverse: {}", force, 1.0/force, (1.0/force).inverse());

    let whatever = length * time;
    println!("Whatever: {}", whatever);
    
    let force = Force::newtons(50.0);
    let mass = Mass::metric_tons(5.0);

    let acceleration = force / mass;
    println!("Force: {}, Mass: {}, Acceleration: {}", force, mass, acceleration);

    let distance = acceleration * time * time / 2.0;
    println!("Distance traveled under constant acceleration: {}", distance);

    let area = length * length;
    let s = area.sqrt();
    println!("Square root of area {} is {}", area, s);

    let volume = area * length;
    let hyper_volume = volume * length;
    println!("Length: {}, Area: {}, Volume: {} HyperVolume: {}", length, area, volume, hyper_volume);

    let dist_vec = Vector2::new(Distance::meters(3.0), Distance::meters(4.0));
    let dist_vec2 = dist_vec * 2.0;

    let sum = dist_vec + dist_vec2;

    println!("Distance Vector: {}, Scaled: {}, Sum: {}", dist_vec, dist_vec2, sum);

    let voltage = Voltage::volts(12.0);
    let current= Current::amperes(2.0);
    let resistance= voltage / current;
    println!("Voltage: {}, Current: {}, Resistance: {}", voltage, current, resistance);
    let power = voltage * current;
    println!("Power: {}", power);
    let energy = power * time;
    println!("Energy: {}", energy);
    let charge = Charge::coulombs(24.0);
    println!("Charge: {}, Current from Charge/Time: {}", charge, charge / time);

    // let cycle_time = Duration::from_millis(1);

    // port_bench();

    // BehaviorGroup::<MainGroup>::with_name("MainGroup", cycle_time);

    // park();
}

#[module] 
struct DistanceSensor {
    pub out_distance: SendPort<Distance>,
}

impl Module for DistanceSensor {
    fn transfere(&mut self) {
        self.out_distance.send(Distance::meters(1.23));
    }

    fn target_rating(&self) -> MetaSignal {
        MetaSignal::HIGH
    }
}

fn port_bench() {
    let send_port: SendPort<i32> = SendPort::default();
    let mut receive_port: ReceivePort<i32> = ReceivePort::default();

    receive_port.connect_to_source(&send_port);

    let start = std::time::Instant::now();
    for i in 0..1_000_000 {
        send_port.send(i);
        receive_port.update();
        assert_eq!(receive_port.get(), Some(&i));
    }
    let duration = start.elapsed();
    println!("Time elapsed for i32 port benchmark: {:?}", duration / 1_000_000);

    let send_port: SendPort<_> = SendPort::default();
    let pass_through_port: SendPort<_> = SendPort::default();
    let mut receive_port: ReceivePort<_> = ReceivePort::default();

    receive_port.connect_to_source(&pass_through_port);
    pass_through_port.connect_to_source(&send_port);
    let start = std::time::Instant::now();
    for i in 0..1_000_000 {
        let data = [i; 100];
        send_port.send(data);
        receive_port.update();
        assert_eq!(receive_port.get(), Some(&data));
    }
    let duration = start.elapsed();
    println!("Time elapsed for [i32; 100] port benchmark: {:?}", duration / 1_000_000);

    let send_port: SendPort<_> = SendPort::default();
    let pass_through_port1: ReceivePort<_> = ReceivePort::default();
    let pass_through_port2: SendPort<_> = SendPort::default();
    let mut receive_port: ReceivePort<_> = ReceivePort::default();

    pass_through_port1.connect_to_source(&send_port);
    pass_through_port2.connect_to_source(&pass_through_port1);
    receive_port.connect_to_source(&pass_through_port2);
    let start = std::time::Instant::now();
    for i in 0..1_000_000 {
        let data = Vec::<i32>::from_iter(0..100);
        send_port.send(data.clone());
        receive_port.update();
        assert_eq!(receive_port.get(), Some(&data));
    }
    let duration = start.elapsed();
    println!("Time elapsed for Vec<i32>(len: 100) port benchmark: {:?}", duration / 1_000_000);
}

#[group]
struct PassThroughGroup {
    pub in_data: ReceivePort<i32>,
    pub out_data: SendPort<i32>,
}

impl Group for PassThroughGroup {
    #[spawn]
    fn init(&mut self, _cycle_time: std::time::Duration) {
        let module = BehaviorModule::<PrintModule<i32>>::with_name("PrintModule", Duration::from_millis(500));
        module.in_data.connect_to_source(&self.in_data);

        self.out_data.connect_to_source(&self.in_data);
    }
}

#[group]
struct MainGroup {
    out_numbers: SendPort<i32>,
}

impl Group for MainGroup {
    #[spawn]
    fn init(&mut self, cycle_time: std::time::Duration) {
        let module1 = BehaviorModule::<Module1>::with_name("Sender 1", cycle_time);
        let module2 = BehaviorModule::<Module2>::with_name("Sender 2", cycle_time);
        // let blocking_module = BehaviorModule::<BlockingModule>::with_name("Blocking", cycle_time);
        let pass_through_module = BehaviorGroup::<PassThroughGroup>::with_name("PassThrough", cycle_time);
    

        let mut fusion_module = MaximumFusion::with_name("Fusion", cycle_time);
        
        connect_fusion!{
            fusion_module,
            ports: [
                // blocking_module.out_number,
                module1.out_numbers,
                module2.out_numbers
                ]
            };
            
        pass_through_module.in_data.connect_to_source(&fusion_module.output);

        self.out_numbers.connect_to_source(&pass_through_module.out_data);
        
        self.set_characteristic_module(&mut fusion_module);
    }
}

#[module]
struct Module1 {
    pub out_numbers: SendPort<i32>,
    counter: i32,
}

impl Module for Module1 {
    fn init() -> Self {
        Self {
            counter: 0,
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        self.counter += 1;
        self.out_numbers.send(self.counter);
    }

    fn target_rating(&self) -> MetaSignal {
        MetaSignal::from((self.counter / 10 % 2) as f32)
    }
}

#[module]
struct Module2 {
    pub out_numbers: SendPort<i32>,
    counter: i32,
}

impl Module for Module2 {
    fn init() -> Self {
        Self {
            counter: 0,
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        self.counter -= 1;
        self.out_numbers.send(self.counter);
    }

    fn target_rating(&self) -> MetaSignal {
        MetaSignal::HIGH
    }
}

#[module] 
struct BlockingModule {
    pub out_number: SendPort<i32>,
    pub out_result: SendPort<f32>,
    counter: i32,
}

impl Module for BlockingModule {
    fn init() -> Self {
        Self {
            counter: 0,
            ..Default::default()
        }
    }

    fn transfere(&mut self) {
        self.counter += 1;
        let c: f32 = (0..1000000000).into_iter().map(|i| f32::sqrt(i as f32)).sum();
        self.out_number.send(self.counter);
        self.out_result.send(c);
    }

    fn target_rating(&self) -> MetaSignal {
        if self.counter % 10 == 0 {
            MetaSignal::HIGH
        } else {
            MetaSignal::LOW
        }
    }
}

#[module]
struct PrintModule<T: Default + Clone + Debug>{
    pub in_data: ReceivePort<T>,
}

impl<T: Default + Clone + Debug> Module for PrintModule<T> {
    fn transfere(&mut self) {
        println!("Received number: {:?}", self.in_data.get());
    }

    fn target_rating(&self) -> MetaSignal {
        MetaSignal::HIGH
    }
}
