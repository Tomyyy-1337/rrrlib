use rust_ib2c_shared_data::SharedData;

use crate::{prelude::*, tcp_server::Parent};

/// Macro to connect multiple module output ports to a fusion module.
/// # Example
/// 
/// ```rust ignore
/// let module1 = BehaviorModule::<Module1>::with_name("Sender 1", cycle_time);
/// let module2 = BehaviorModule::<Module2>::with_name("Sender 2", cycle_time);
/// let mut fusion_module = MaximumFusion::with_name("Fusion", cycle_time);
/// connect_fusion!{
///     fusion_module,
///     ports: [
///         module1.out_data,
///         module2.out_data
///     ]
/// }
/// ```
#[macro_export]
macro_rules! connect_fusion {
    (
        $fusion_module:expr,
        ports: [
            $( $module:ident . $port:ident ),+$(,)?
        ]
    ) => {
        $(
            $fusion_module.connect_module(&*$module, &$module.$port);
        )+
    };
}

/// Fusion module that selects the output from the module with the highest activity.
/// If multiple modules have the same activity, the first one encountered is chosen.
/// The order of modules is determined by the order in which they are connected to the fusion module.
#[ports]
pub struct MaximumFusion<D: Clone> {
    name: String,
    pub output: SendPort<D>,
    activitys: Vec<ReceivePort<MetaSignal>>,
    target_ratings: Vec<ReceivePort<MetaSignal>>,
    data_ports: Vec<ReceivePort<D>>,
    cycle_time: std::time::Duration,
    parent: Parent,
    loop_count: u64,
}

impl<D> MaximumFusion<D> 
where
    D: Clone + Default + Send + 'static,
    Self: Send + 'static
{
    /// Creates a new fusion module with the given name and cycle time.
    pub fn with_name(name: &str, cycle_time: std::time::Duration, parent: &Parent) -> Self {
        Self {
            name: name.to_string(),
            output: SendPort::default(),
            activitys: Vec::new(),
            data_ports: Vec::new(),
            cycle_time,
            parent: Parent {
                path: format!("{}/{}", parent.path, name),
                tcp_server: parent.tcp_server.clone(),
            },
            loop_count: 0,
            ..Default::default()
        }
    }

    /// Connects a module's output port to the fusion module. Use the [`connect_fusion!`] macro to connect multiple modules at once.
    pub fn connect_module<M: MetaSignals>(&mut self, module: &M, in_data_port: &SendPort<D>) {
        let activity_port = ReceivePort::default();
        activity_port.connect_to_source(&module.get_activity_port());
        self.activitys.push(activity_port);

        let data_port = ReceivePort::default();
        data_port.connect_to_source(in_data_port);
        self.data_ports.push(data_port);

        let target_rating_port = ReceivePort::default();
        target_rating_port.connect_to_source(&module.get_target_rating_port());
        self.target_ratings.push(target_rating_port);
    }

    fn max_fusion(&self) -> Option<(MetaSignal, MetaSignal, D)> {
        let mut best_data= None;
        let mut max_activity = MetaSignal::LOW;
        let mut best_index = 0;

        
        for (index, activity_port) in self.activitys.iter().enumerate() {
            if let Some(activity) = activity_port.get() {
                if activity > &max_activity {
                    max_activity = *activity;
                    best_data = self.data_ports[index].get();
                    best_index = index;
                }
            }
        }

        if let Some(data) = best_data {
            let target_rating = self.target_ratings[best_index].get().cloned().unwrap_or(MetaSignal::LOW);
            Some((max_activity, target_rating, data.clone()))
        } else {
            None
        }
    }

    /// Spawns the fusion module in its own thread.
    pub fn spawn(mut self) 
    {
        println!("Spawned module: {}", self.name);
        let _ = std::thread::spawn(move || {
            loop {
                let start = std::time::Instant::now();
                for activity_ports in &mut self.activitys {
                    activity_ports.update();
                }
                for data_ports in &mut self.data_ports {
                    data_ports.update();
                }
                for target_rating_ports in &mut self.target_ratings {
                    target_rating_ports.update();
                }
                if let Some((activity, target_rating, output)) = self.max_fusion() {
                    self.set_activity(activity);
                    self.set_target_rating(target_rating);
                    self.output.send(output);
                }

                self.loop_count += 1;

                let shared_data = SharedData {
                    index: self.loop_count,
                    source: self.parent.path.clone(),
                    activity: *self.activity.get().unwrap_or(MetaSignal::LOW),
                    target_rating: *self.target_rating.get().unwrap_or(MetaSignal::LOW),
                    stimulation: 0.0,
                    inhibition: 0.0,
                };
                self.parent.tcp_server.send(shared_data);

                let elapsed = start.elapsed();

                if cfg!(feature = "print_state") {
                    eprintln!("(Fusion) Elapsed time: {:6?} Activity: {} Target Rating: {}                              Path: {}",
                        elapsed, self.get_activity().unwrap_or(MetaSignal::LOW), self.get_target_rating().unwrap_or(MetaSignal::LOW), self.parent.path);
                }

                if elapsed < self.cycle_time {
                    std::thread::sleep(self.cycle_time - elapsed);
                }
            }
        });
    }
}