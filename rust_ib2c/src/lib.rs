/// Traits defining the behavior of modules and groups.
pub mod traits;
/// Send and Receive Ports for data transfer between modules.
pub mod port;
/// Behavior module wrapper to run modules in their own threads.
pub mod behavior_module;
/// Behavior group wrapper to run groups of modules in their own threads.
pub mod group;
/// Fusion modules to combine data from multiple modules.
pub mod fusion_module;
/// Data structures and utilities for meta-signals.
pub mod meta_signals;
/// TCP server for remote monitoring and control of modules.
pub(crate) mod tcp_server;

/// Re-exports commonly used items for easier access.
pub mod prelude {
    pub use crate::traits::{Module, Group, MetaSignals, UpdateReceivePorts, PortSerialization, PortParsing};
    pub use crate::port::{SendPort, ReceivePort, OutputPort, InputPort, ParameterPort};
    pub use crate::behavior_module::BehaviorModule;
    pub use crate::group::BehaviorGroup;
    pub use crate::fusion_module::MaximumFusion;
    pub use crate::meta_signals::MetaSignal;
    pub use crate::tcp_server::Parent;
    pub use ib2c_macros::module;
    pub use ib2c_macros::group;
    pub use ib2c_macros::ports;
    pub use ib2c_macros::spawn;
    pub use rust_ib2c_shared_data::PortData;
    pub use crate::SpawnMainGroup;
}