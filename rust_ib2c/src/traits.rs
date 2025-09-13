use std::time::Duration;

use crate::prelude::*;

/// Module trait for behavior modules. Can be spawned using the [`BehaviorModule`] struct.
pub trait Module: UpdateReceivePorts + MetaSignals + Default {
    fn init() -> Self {
        Self::default()
    }
    fn transfere(&mut self);
    fn target_rating(&self) -> MetaSignal;
}

/// Module trait for groups of behavior modules. Can be spawned using the [`BehaviorGroup`] struct.
pub trait Group: MetaSignals + UpdateReceivePorts + Default {
    fn init(&mut self, cycle_time: std::time::Duration);
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

/// Trait for updating all receive ports of modules and groups.
pub trait UpdateReceivePorts {
    fn update_all_ports(&mut self);
}