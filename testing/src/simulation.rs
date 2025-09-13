use std::time::Duration;

use nannou::{color::*, geom::Rect, event::Update, window, App, Frame};
use rust_ib2c::prelude::*;
use data_types::prelude::*;

use crate::controll_system::ControlSystem;

pub struct Model {
    _window: window::Id,

    car_position: Vector2<Distance>,
    car_orientation: Rotation2D,

    in_front_distance_sensor: InputPort<Distance>,
    in_left_distance_sensor: InputPort<Distance>,
    in_right_distance_sensor: InputPort<Distance>,
    out_velocity: OutputPort<Velocity>,
    out_turn_rate: OutputPort<AngularVelocity>,
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let control_system = BehaviorGroup::<ControlSystem>::with_name("ControlSystem", Duration::from_millis(10));

    Model {
        _window,
        in_front_distance_sensor: InputPort::from(&control_system.in_front_distance_sensor),
        out_velocity: OutputPort::from(&control_system.out_velocity),
        out_turn_rate: OutputPort::from(&control_system.out_turn_rate),
        in_left_distance_sensor: InputPort::from(&control_system.in_left_distance_sensor),
        in_right_distance_sensor: InputPort::from(&control_system.in_right_distance_sensor),
        car_position: Vector2::default(),
        car_orientation: Rotation2D::from_angle(0.0),
    }
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    let mouse_position: Vector2<Distance> = Vector2::new(Distance::centimeters(app.mouse.x as f64), Distance::centimeters(app.mouse.y as f64));
    
    let wall_distance_front = virtual_sensor_to_wall(model.car_position, model.car_orientation, Distance::meters(100.0), app.window_rect());
    let wall_distance_left = virtual_sensor_to_wall(model.car_position, model.car_orientation + Rotation2D::from_angle(1.0), Distance::meters(100.0), app.window_rect());
    let wall_distance_right = virtual_sensor_to_wall(model.car_position, model.car_orientation + (-1.0 * Rotation2D::from_angle(1.0)), Distance::meters(100.0), app.window_rect());
    
    let mouse_distance = vitrual_sensor_to_mouse_circle(model.car_position, model.car_orientation, Distance::meters(100.0), mouse_position, Distance::centimeters(50.0));
    let mouse_distance_left = vitrual_sensor_to_mouse_circle(model.car_position, model.car_orientation + Rotation2D::from_angle(1.0), Distance::meters(100.0), mouse_position, Distance::centimeters(50.0));
    let mouse_distance_right = vitrual_sensor_to_mouse_circle(model.car_position, model.car_orientation + (-1.0 * Rotation2D::from_angle(1.0)), Distance::meters(100.0), mouse_position, Distance::centimeters(50.0));
    
    let min_distance = Distance::min(wall_distance_front, mouse_distance);
    model.in_front_distance_sensor.set(min_distance);

    let min_distance_left = Distance::min(wall_distance_left, mouse_distance_left);
    model.in_left_distance_sensor.set(min_distance_left);
    let min_distance_right = Distance::min(wall_distance_right, mouse_distance_right);
    model.in_right_distance_sensor.set(min_distance_right);
    
    let velocity = model.out_velocity.get_or_default();
    let delta_time = Time::seconds(update.since_last.as_secs_f64());
    let velocity_vector = model.car_orientation.as_unit_vector() * velocity;
    model.car_position += velocity_vector * delta_time;

    let turn_rate = model.out_turn_rate.get_or_default();
    if velocity > Velocity::meters_per_second(0.1) {
        model.car_orientation = model.car_orientation + (turn_rate * delta_time).as_radians();
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
     
    draw.rect()
        .x_y(
            model.car_position.x().as_centimeters() as f32, 
            model.car_position.y().as_centimeters() as f32
        )
        .w_h(20.0, 10.0)
        .rotate(model.car_orientation.as_radians() as f32)
        .color(ORANGE);

    // let me: Distance = meters::from(1.0).into();

    let mouse_position: Vector2<Distance> = Vector2::new(Distance::centimeters(app.mouse.x as f64), Distance::centimeters(app.mouse.y as f64));
    draw.ellipse()
        .x_y(
            mouse_position.x().as_centimeters() as f32, 
            mouse_position.y().as_centimeters() as f32
        )
        .w_h(100.0, 100.0)
        .color(RED);

    draw.to_frame(app, &frame).unwrap();
}

fn vitrual_sensor_to_mouse_circle(
    position: Vector2<Distance>, 
    orientation: Rotation2D, 
    max_distance: Distance, 
    mouse_pos: Vector2<Distance>, 
    radius: Distance
) -> Distance {
    let direction_vec = orientation.as_unit_vector();
    let inc_distance = Distance::centimeters(10.0);
    let inc_vector = direction_vec * inc_distance;
    let mut sensor_pos = position + inc_vector;
    let mut wall_distance = Distance::ZERO;

    while (sensor_pos - mouse_pos).magnitude_squared() > radius * radius && wall_distance < max_distance {
        sensor_pos += inc_vector;
        wall_distance += inc_distance;
    }

    wall_distance
}

fn virtual_sensor_to_wall(position: Vector2<Distance>, orientation: Rotation2D, max_distance: Distance, canvas_size: Rect) -> Distance {
    let canvas_top_right= Vector2::new(Distance::centimeters(canvas_size.right() as f64), Distance::centimeters(canvas_size.top() as f64));
    let canvas_bottom_left= Vector2::new(Distance::centimeters(canvas_size.left() as f64), Distance::centimeters(canvas_size.bottom() as f64));

    let direction_vec = orientation.as_unit_vector();
    let inc_distance = Distance::centimeters(10.0);
    let inc_vector = direction_vec * inc_distance;
    let mut sensor_pos = position + inc_vector;
    let mut wall_distance = Distance::ZERO;

    while sensor_pos.x() < canvas_top_right.x() && sensor_pos.x() > canvas_bottom_left.x() &&
          sensor_pos.y() < canvas_top_right.y() && sensor_pos.y() > canvas_bottom_left.y() &&
          wall_distance < max_distance {
        sensor_pos += inc_vector;
        wall_distance += inc_distance;
    }

    wall_distance
}