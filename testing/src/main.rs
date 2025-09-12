use std::{fmt::Debug, thread::park, time::Duration};

// use data_types::{prelude::*, si_units::{Distance, Frac, Meter, MeterPerSecond, Second, Si, Time}, vector::{self, Vector2}};
use rust_ib2c::prelude::*;
use data_types::prelude::*;

fn main() {
    let uomlen = uom::si::f64::Length::new::<uom::si::length::meter>(10.0);

    let length: Distance = Distance::meters(10.0);
    let time: Time = Time::seconds(9.0);
    let speed: Speed = length / time;
    println!("Length: {}, Time: {}, Speed: {}", length, time, speed);
    let force = Force::newtons(10.0);
    println!("Froce {}, inverse: {}, inverse_inverse: {}", force, 1.0/force, (1.0/force).inverse());

    let whatever = length * time;
    println!("Whatever: {}", whatever);
    
    let force: Force = Force::newtons(50.0);
    let mass: Mass = Mass::metric_tons(5.0);

    let acceleration = force / mass;
    println!("Force: {}, Mass: {}, Acceleration: {}", force, mass, acceleration);

    let distance = acceleration * time * time / 2.0;
    println!("Distance traveled under constant acceleration: {}", distance);

    let area: Area = length * length;
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
    let power: Power = voltage * current;
    println!("Power: {}", power);
    let energy: Energy = power * time;
    println!("Energy: {}", energy);
    let charge = Charge::coulombs(24.0);
    println!("Charge: {}, Current from Charge/Time: {}", charge, charge / time);

    // let cycle_time = Duration::from_millis(1);

    // port_bench();

    // BehaviorGroup::<MainGroup>::with_name("MainGroup", cycle_time);

    // park();
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
