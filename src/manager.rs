use crate::{execute_on_click, BoxedCallback};
use rayon::prelude::*;

/// Allows several callbacks to be registered for different devices.
///
/// ```rust
/// Manager::new("FF:FF:C3:17:01:01", Box::new(toggle_reading_light))
///    .add("FF:FF:C3:17:01:02", Box::new(wake_up_tv))
///    .start()
/// ```
#[derive(Default)]
pub struct Manager<'a>(Vec<(&'a str, BoxedCallback)>);

impl<'a> Manager<'a> {

    pub fn new() -> Self {
        Manager(Vec::new())
    }

    /// Adds an additional callback to the given MAC address.
    pub fn add(mut self, button_mac_address: &'a str, callback: BoxedCallback) -> Self {
        self.0.push((button_mac_address, callback));
        self
    }

    /// Starts listening to events on the MAC addresses provided via `new` and `add`.
    /// This method is blocking: all the specified callbacks will run in parallel, but it is up to
    /// the caller to execute `start` within a new thread or use another async mechanism if the
    /// computation should continue.
    pub fn start(self) {
        self.0
            .into_par_iter()
            .for_each(|(button_mac_address, callback)| {
                execute_on_click(&button_mac_address, callback)
            });
    }
}

