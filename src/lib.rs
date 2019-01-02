//! This library aims to provide a simple interface to react to "clicks" of single button
//! bluetooth devices (shutters/wire free remotes for selfie sticks and the like).

#[macro_use]
extern crate log;

use evdev;
use evdev::Device;
use std::thread;
use std::time::Duration;

mod manager;

pub use self::manager::Manager;
use evdev::raw::input_event;

/// Type alias for readability purposes.
///
/// API Users can pass `Box::new(|| {..})` (or box || {..} in nightly) to methods where this
/// type is expected.
pub type BoxedCallback = Box<FnMut() -> () + std::marker::Send>;

/// Executes a callback whenever a "click" is registered on the specified MAC address.
///
/// Currently a "click" is defined as a `KEY_VOLUMEUP` event (keycode `115`) with value `1` (pressed).
///
/// This function is blocking. It is up to the caller execute it within a new thread or use another
/// async mechanism if the computation should continue.
pub fn execute_on_click(button_mac_address: &str, mut callback: BoxedCallback) {
    // TODO: avoid looping for ever, better error handling when events_no_sync returns Err
    loop {
        if let Some(button) = find_device_with_mac(&button_mac_address) {
            info!("Listening to events from {:?}", &button_mac_address);
            execute_on_every_click_until_events_error(button, &mut callback);
            thread::sleep(Duration::from_millis(500));
        }
    }
}

fn find_device_with_mac(target_mac_address: &str) -> Option<Device> {
    evdev::enumerate().into_iter().find(|device| {
        if let Some(mac_address) = device.unique_name() {
            target_mac_address.as_bytes() == mac_address.as_bytes()
        } else {
            false
        }
    })
}

fn execute_on_every_click_until_events_error(mut button: Device, callback: &mut BoxedCallback) {
    loop {
        match button.events_no_sync() {
            Ok(events) => events
                .filter(is_pressed_volume_up)
                .for_each(|_| {
                    debug!("Callback triggered");
                    callback()
                }),
            Err(error) => {
                error!("Error listening to events ({}).", error.errno().desc());
                break;
            }
        }
    }
}

// TODO: Make this configurable/expose in the API in order to support other devices
fn is_pressed_volume_up(event: &input_event) -> bool {
    evdev::KEY.number::<u16>() == event._type
        && evdev::Key::KEY_VOLUMEUP as u16 == event.code
        && 1 == event.value
}
