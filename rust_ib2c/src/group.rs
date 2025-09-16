use std::ops::{Deref, DerefMut};

use crate::{prelude::*, tcp_server::{Parent, TcpServer}};

/// Macro to spawn the main behavior group.
/// # Example
/// ```rust
/// use std::time::Duration;
/// use rust_ib2c::prelude::*;
/// 
/// [group]
/// struct MyMainGroup {
///     // Define your ports here
/// }
/// 
/// impl Group for MyMainGroup {
///   [spawn]
///   fn init(&mut self, _cycle_time: Duration, _parent: &Parent) {
///       // Initialize your group here
///   }
/// }
/// fn main() {
///     let main_group = SpawnMainGroup!(MyMainGroup, "MyMainGroup", Duration::from_millis(100));
/// }
#[macro_export]
macro_rules! SpawnMainGroup {
    ($group_type:ty, $name:expr, $cycle_time:expr) => {
        BehaviorGroup::<$group_type>::main_group($name, $cycle_time)
    };
}

/// Behavior group wrapper to run groups in their own threads.
pub struct BehaviorGroup<M> 
where
    M: Group + Default + Send + 'static
{
    pub module: M,
}

impl<M> DerefMut for BehaviorGroup<M> 
where
    M: Group + Default + Send + 'static
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.module
    }
}

impl<M> Deref for BehaviorGroup<M> 
where
    M: Group + Default + Send + 'static
{
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

impl<M> BehaviorGroup<M> 
where
    M: Group + Default + Send + 'static
{
    /// Creates a new behavior group with the given name and cycle time from a parent module or group.
    pub fn with_name(name: &str, cycle_time: std::time::Duration, parent: &Parent) -> Self {
        println!("Initializing BehaviorGroup: {}", name);
        let mut group = M::default();
        let parent = Parent {
            path: format!("{}/{}", parent.path, name),
            tcp_server: parent.tcp_server.clone(),
        };
        group.init(cycle_time, &parent);
        Self {
            module: group,
        }
    }

    /// Creates a new main behavior group with the given name and cycle time.
    pub fn main_group(name: &str, cycle_time: std::time::Duration) -> Self {
        println!("Initializing  Main BehaviorGroup: {}", name);
        let tcp_server = TcpServer::new();
        tcp_server.start(); 
        let parent = Parent {
            path: name.to_string(),
            tcp_server,
        };
        let mut group = M::default();
        group.init(cycle_time, &parent);
        Self {
            module: group,
        }
    }
}