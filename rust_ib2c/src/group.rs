use std::{ops::{Deref, DerefMut}};

use crate::{prelude::*};

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
    /// Creates a new behavior group with the given name and cycle time.
    pub fn with_name(name: &str, cycle_time: std::time::Duration) -> Self {
        println!("Initializing BehaviorGroup: {}", name);
        let mut group = M::default();
        group.init(cycle_time);
        Self {
            module: group,
        }
    }
}