use std::ops::{Deref, DerefMut};

use rust_ib2c_shared_data::SharedData;

use crate::{prelude::*, tcp_server::Parent};

/// Behavior module wrapper to run modules in their own threads.
pub struct BehaviorModule<M> 
where
    M: Module + Send + 'static
{
    name: String,
    pub module: M,
    cycle_time: std::time::Duration,
    last_update: std::time::Instant,
    parent: Parent,
    loop_count: u64,
}

impl<M> DerefMut for BehaviorModule<M> 
where
    M: Module + Send + 'static
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.module
    }
}

impl<M> Deref for BehaviorModule<M> 
where
    M: Module + Send + 'static
{
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

impl<M: Module> BehaviorModule<M> 
where
    M: Module + Send + 'static
{
    /// Creates a new behavior module with the given name and cycle time.
    pub fn with_name(name: &str, cycle_time: std::time::Duration, parent: &Parent) -> Self {
        Self {
            name: name.to_string(),
            module: M::init(),
            cycle_time,
            last_update: std::time::Instant::now(),
            parent: Parent {
                path: format!("{}/{}", parent.path, name),
                tcp_server: parent.tcp_server.clone(),
            },
            loop_count: 0,
        }
    }

    /// Spawns the behavior module in its own thread. 
    /// Use the [`spawn`] attribute macro to automatically call spawn on the end of the [`init`][Group::init] function of a [`Group`].
    pub fn spawn(mut self) 
    {
        println!("Spawned module: {}", self.name);
        let _ = std::thread::spawn(move || {
            loop {
                let start = std::time::Instant::now();
                let delta_time = start.duration_since(self.last_update);
                self.last_update = start;
                self.set_delta_time(delta_time);
                self.update_all_ports();
                self.transfere();
                let target_rating = self.module.target_rating();

                let stimulation = *self.module.get_stimulation().unwrap_or(&MetaSignal::HIGH);
                let inhibition = *self.get_inhibition().unwrap_or(&MetaSignal::LOW);
                let potential = MetaSignal::min(
                    stimulation, 
                    MetaSignal::HIGH - inhibition
                );
                let activity = MetaSignal::min(
                    potential,
                    target_rating,
                );

                self.set_activity(activity);
                self.set_target_rating(target_rating);

                self.loop_count += 1;

                let port_data = self.module.all_port_data();
                
                let shared_data = SharedData {
                    index: self.loop_count,
                    active_time: start.elapsed(),
                    source: self.parent.path.clone(),
                    activity: *activity,
                    target_rating: *target_rating,
                    stimulation: *stimulation,
                    inhibition: *inhibition,
                    data: port_data.into_iter().map(|(name, data)| (name.to_string(), data)).collect(),
                };
                self.parent.tcp_server.send(shared_data);
                
                let elapsed = start.elapsed();
                // only active with compiler flag "print_state"
                if cfg!(feature = "print_state") {
                    eprintln!("(Module) Elapsed time: {:6?} Activity: {} Target Rating: {} Stimulation: {} Inhibition: {} Path: {}", 
                        elapsed, self.get_activity().unwrap_or(MetaSignal::LOW), target_rating, stimulation, inhibition, self.parent.path);   
                    if elapsed > self.cycle_time {
                        eprintln!("Warning: Module '{}' is running behind schedule! Cycle time: {:?}, Elapsed time: {:?}", self.name, self.cycle_time, elapsed);
                    }
                }

                if elapsed < self.cycle_time {
                    std::thread::sleep(self.cycle_time - elapsed);
                } 
            }
        });

    }
}