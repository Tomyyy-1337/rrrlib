use std::time::Duration;

use data_types::{prelude::Force, vectors::Vector2};
use nannou::{color::white_point::C, prelude::*};
use rust_ib2c::prelude::*;
use data_types::si_units::*;

use crate::{ControlSystem};

pub struct Model {
    _window: window::Id,

    car_position: Vector2<Distance>,

    in_front_distance_sensor: InputPort<Distance>,
    out_velocity: OutputPort<Velocity>,
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let control_system = BehaviorGroup::<ControlSystem>::with_name("ControlSystem", Duration::from_millis(200));

    Model {
        _window,
        in_front_distance_sensor: InputPort::from(&control_system.in_front_distance_sensor),
        out_velocity: OutputPort::from(&control_system.out_velocity),
        car_position: Vector2::default(),
    }
}

pub fn update(_app: &App, model: &mut Model, update: Update) {
    let delta_time = Time::seconds(update.since_last.as_secs_f64());
    let velocity = model.out_velocity.get_or_default();
    println!("Velocity: {}", velocity);
    *model.car_position.x_mut() += velocity * delta_time;

    println!("X Position: {}", model.car_position.x());
    if model.car_position.x() > Distance::meters(10.0) {
        model.in_front_distance_sensor.set(Distance::meters(0.0));
    } else {
        model.in_front_distance_sensor.set(Distance::meters(5.0));
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    
    let elipse_pos = pt2(model.car_position.x().as_meters() as f32 * 50.0 - 250.0, 0.0);
    draw.ellipse().x_y(elipse_pos.x, elipse_pos.y).w_h(30.0, 15.0).color(ORANGE);

    draw.to_frame(app, &frame).unwrap();
}