use std::time::Duration;
use rust_ib2c_shared_data::PortData;

use crate::{prelude::*, tcp_server::Parent};

/// Module trait for behavior modules. Can be spawned using the [`BehaviorModule`] struct.
pub trait Module: UpdateReceivePorts + MetaSignals + PortParsing + Default {
    /// Spawn other modules and groups here and connect them.
    /// 
    /// Use the SpawnModule!, SpawnGroup! and SpawnFusion! macros to create instances.
    /// # Examples
    /// ```rust
    /// fn init(&mut self, cycle_time: Duration, parent: &Parent) {
    ///    let break_on_obstacle = SpawnModule!(BreakOnObstacle, "BreakOnObstacle");
    ///    break_on_obstacle.in_distance.connect_to_source(&self.in_front_distance_sensor);
    ///
    ///    let constant_velocity = SpawnModule!(ConstantVelocity, "ConstantVelocity");
    ///    
    ///    let mut maximum_fusion = SpawnFusion! {
    ///        MaximumFusion,
    ///        "MaximumFusion",
    ///        inputs: [
    ///            break_on_obstacle.out_velocity,
    ///            constant_velocity.out_velocity,
    ///        ]
    ///    };
    ///
    ///    self.out_velocity.connect_to_source(&maximum_fusion.output);   
    ///
    ///    self.set_characteristic_module(&mut maximum_fusion);
    ///}
    fn init() -> Self {
        Self::default()
    }
    fn transfere(&mut self);
    fn target_rating(&self) -> MetaSignal;
}

/// Module trait for groups of behavior modules. Can be spawned using the [`BehaviorGroup`] struct.
pub trait Group: MetaSignals + UpdateReceivePorts + Default {
    fn init(&mut self, cycle_time: std::time::Duration, path: &Parent);
}

/// Trait for access to meta signals: activity, target rating, stimulation, inhibition
pub trait MetaSignals {
    fn set_activity(&mut self, activity: MetaSignal);
    fn get_activity(&self) -> Option<MetaSignal>;
    fn set_target_rating(&mut self, target_rating: MetaSignal);
    fn get_target_rating(&self) -> Option<MetaSignal>;
    fn get_stimulation(&self) -> Option<&MetaSignal>;
    fn get_inhibition(&self) -> Option<&MetaSignal>;
    fn get_activity_port(&self) -> &SendPort<MetaSignal>;
    fn get_target_rating_port(&self) -> &SendPort<MetaSignal>;
    fn get_stimulation_port(&mut self) -> &ReceivePort<MetaSignal>;
    fn get_inhibition_port(&mut self) -> &ReceivePort<MetaSignal>;
    fn set_delta_time(&mut self, delta_time: Duration);
}

/// Internal trait to get all port data of a module for serialization 
pub trait PortParsing {
    fn all_port_data(&self) -> Vec<(&'static str, PortData)>;
}

/// Trait for updating all receive ports of modules and groups.
pub trait UpdateReceivePorts {
    fn update_all_ports(&mut self);
}

/// Required for serialization of port data.
pub trait PortSerialization {
    fn serialize_port_data(&self) -> PortData;
}