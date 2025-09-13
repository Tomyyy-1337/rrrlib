use std::{ops::Deref, sync::{Arc, RwLock}};

struct PortBuffer<T: Clone> {
    buffer: Option<T>,
}

enum PortMode<T: Clone> {
    Buffer(PortBuffer<T>),
    Passthrough(Port<T>),
}

/// Internal port structure used by [`SendPort`] and [`ReceivePort`]
pub struct Port<T: Clone> {
    mode: Arc<RwLock<PortMode<T>>>,
}

impl<T: Clone> Clone for Port<T> {
    fn clone(&self) -> Self {
        Self {
            mode: Arc::clone(&self.mode),
        }
    }
}

impl<T: Clone> Port<T> {
    fn send(&self, data: T) {
        if let PortMode::Passthrough(source_port) = &*self.mode.read().unwrap() {
            source_port.send(data);
            return;
        }
        let mut writer = self.mode.write().unwrap();
        *writer = PortMode::Buffer(PortBuffer {
            buffer: Some(data),
        });
    }

    fn get(&self) -> Option<T> {
        match &*self.mode.read().unwrap() {
            PortMode::Buffer(buffer) => buffer.buffer.clone(),
            PortMode::Passthrough(source_port) => source_port.get(),
        }
    }

    fn get_or_default(&self) -> T
    where
        T: Default,
    {
        self.get().unwrap_or_default()
    }

    fn connect_to_source(&self, source: &Port<T>) {
        *self.mode.write().unwrap() = PortMode::Passthrough(source.clone());
    }
} 

/// Sending port used to send data to connected [`ReceivePort`]s
pub struct SendPort<T: Clone> {
    inner: Port<T>,
}

/// Receiving port used to receive data from a connected [`SendPort`]
pub struct ReceivePort<T: Clone> {
    inner: Port<T>,
    buffer: Option<T>,
}

impl<T: Clone> Deref for SendPort<T> {
    type Target = Port<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clone> Deref for ReceivePort<T> {
    type Target = Port<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clone> Default for SendPort<T> {
    fn default() -> Self {
        Self {
            inner: Port {
                mode: Arc::new(RwLock::new(PortMode::Buffer(PortBuffer { buffer: None }))),
            },
        }
    }
}

impl<T: Clone> Clone for SendPort<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Clone> Default for ReceivePort<T> {
    fn default() -> Self {
        Self {
            inner: Port {
                mode: Arc::new(RwLock::new(PortMode::Buffer(PortBuffer { buffer: None }))),
            },
            buffer: None,
        }
    }
}

impl<T: Clone> Clone for ReceivePort<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            buffer: self.buffer.clone(),
        }
    }
}

impl<T: Clone> SendPort<T> {
    /// Send data to connected [`ReceivePort`]s
    pub fn send(&self, data: T) {
        self.inner.send(data);
    }

    /// Connect this [`SendPort`] to a source [`Port`] ([`SendPort`] or [`ReceivePort`])
    pub fn connect_to_source(&self, source: &Port<T>) {
        self.inner.connect_to_source(source);
    }

    /// Connect this [`SendPort`] as a source to a target [`Port`] ([`SendPort`] or [`ReceivePort`])
    pub fn connect_as_source(&self, target: &Port<T>) {
        target.connect_to_source(&self.inner);
    }

    /// Get the last sent data 
    pub fn get(&self) -> Option<T> {
        self.inner.get()
    }

    /// Get the last sent data or a default value if no data was sent yet
    pub fn get_or_default(&self) -> T
    where
        T: Default,
    {
        self.inner.get_or_default()
    }
}

impl<T: Clone> ReceivePort<T> {
    /// Connect this [`ReceivePort`] to a source [`Port`] ([`SendPort`] or [`ReceivePort`])
    pub fn connect_to_source(&self, source: &Port<T>) {
        self.inner.connect_to_source(source);
    }

    /// Connect this [`ReceivePort`] as a source to a target [`Port`] ([`SendPort`] or [`ReceivePort`])
    pub fn connect_as_source(&self, target: &Port<T>) {
        target.connect_to_source(&self.inner);
    }

    /// Update the internal buffer with the latest data from the connected SendPort
    /// Is called automatically when used inside a [`BehaviorModule`][`crate::behavior_module::BehaviorModule`]
    /// and does not need to be called manually.
    pub fn update(&mut self) {
        self.buffer = self.inner.get();
    }

    /// Get the last received data from the internal buffer
    pub fn get(&self) -> Option<&T> {
        self.buffer.as_ref()
    }

    pub fn get_or_default(&self) -> T
    where
        T: Default,
    {
        self.inner.get_or_default()
    }
}

pub struct ParameterPort<T: Clone> {
    inner: Port<T>,
    buffer: T,
}

impl<T: Clone + Default> ParameterPort<T> {
    pub fn set(&self, data: T) {
        self.inner.send(data);
    }

    /// Update the internal buffer with the latest data from the connected SendPort
    /// Is called automatically when used inside a [`BehaviorModule`][`crate::behavior_module::BehaviorModule`]
    /// and does not need to be called manually.
    pub fn update(&mut self) {
        self.buffer = self.inner.get().unwrap_or_default();
    }

    pub fn get(&self) -> &T {
        &self.buffer
    }
}

impl<T: Clone + Default> Default for ParameterPort<T> {
    fn default() -> Self {
        Self {
            inner: Port { 
                mode: Arc::new(RwLock::new(PortMode::Buffer(PortBuffer { buffer: Some(T::default()) }))),
            },
            buffer: T::default(),
        }
    }
}

/// Used as outputs of the controll system to read data from modules and groups 
/// from outside the system;
pub struct OutputPort<T: Clone> {
    source: Port<T>,
}

impl<T: Clone> OutputPort<T> {
    pub fn get(&self) -> Option<T> {
        self.source.get()
    }

    pub fn get_or_default(&self) -> T
    where
        T: Default,
    {
        self.source.get_or_default()
    }
}

impl<T: Clone> From<&SendPort<T>> for OutputPort<T> {
    fn from(port: &SendPort<T>) -> Self {
        Self {
            source: port.inner.clone(),
        }
    }
}

/// Used as inputs to the controll system to write data to modules and groups
pub struct InputPort<T: Clone> {
    target: Port<T>
}

impl<T: Clone> InputPort<T> {
    pub fn set(&mut self, data: T) {
        self.target.send(data);
    }
}

impl<T: Clone> From<&ReceivePort<T>> for InputPort<T> {
    fn from(port: &ReceivePort<T>) -> Self {
        Self {
            target: port.inner.clone(),
        }
    }
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_send_receive_port() {
        let send_port: SendPort<i32> = SendPort::default();
        let mut receive_port: ReceivePort<i32> = ReceivePort::default();

        receive_port.connect_to_source(&send_port);


        assert_eq!(receive_port.get(), None);
        send_port.send(42);
        receive_port.update();
        assert_eq!(receive_port.get(), Some(&42));
    }

    #[test]
    fn test_recieve_port_chain() {
        let send_port: SendPort<i32> = SendPort::default();
        let mut receive_port1: ReceivePort<i32> = ReceivePort::default();
        let mut receive_port2: ReceivePort<i32> = ReceivePort::default();

        receive_port1.connect_to_source(&send_port);
        receive_port2.connect_to_source(&receive_port1);

        send_port.send(42);
        receive_port1.update();
        receive_port2.update();

        assert_eq!(receive_port1.get(), Some(&42));
        assert_eq!(receive_port2.get(), Some(&42));
    }

    #[test]
    fn test_send_port_chain() {
        let send_port1: SendPort<i32> = SendPort::default();
        let send_port2: SendPort<i32> = SendPort::default();
        let mut receive_port: ReceivePort<i32> = ReceivePort::default();

        send_port2.connect_to_source(&send_port1);
        receive_port.connect_to_source(&send_port2);

        send_port1.send(42);
        receive_port.update();

        assert_eq!(receive_port.get(), Some(&42));
    }

    #[test]
    fn mixed_chain() {
        let send_port1: SendPort<i32> = SendPort::default();
        let send_port2: SendPort<i32> = SendPort::default();
        let send_port3: SendPort<i32> = SendPort::default();
        let mut receive_port1: ReceivePort<i32> = ReceivePort::default();
        let mut receive_port2: ReceivePort<i32> = ReceivePort::default();
        let mut receive_port3: ReceivePort<i32> = ReceivePort::default();

        send_port1.connect_as_source(&send_port2);
        receive_port1.connect_to_source(&send_port2);
        send_port3.connect_to_source(&receive_port1);
        receive_port2.connect_to_source(&send_port3);
        receive_port2.connect_as_source(&receive_port3);
        send_port1.send(42);

        receive_port1.update();
        receive_port2.update();
        receive_port3.update();
        assert_eq!(receive_port1.get(), Some(&42));
        assert_eq!(receive_port2.get(), Some(&42));
        assert_eq!(receive_port3.get(), Some(&42));
        assert_eq!(send_port3.get(), Some(42));
        assert_eq!(send_port2.get(), Some(42));
        assert_eq!(send_port1.get(), Some(42));
    }

}
