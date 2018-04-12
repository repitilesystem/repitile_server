//! Defines a simple regulator that enables/disables a
//! LED connected to the Pi for simple threshold testing.

use repitile_core::{CurrentConditions, regulator::Regulator};
use sysfs_gpio::Pin;

/// Simple "regulator" that enables/disables a temperature
/// indicator LED.
pub struct SimpleTempRegulator {
    pin: Pin,
    threshold: u32,
}

impl SimpleTempRegulator {
    /// Creates a new `SimpleTempRegulator` from a GPIO pin number
    /// and threshold value for activation.
    pub fn new(pin: u8, threshold: u32) -> SimpleTempRegulator {
        let pin = Pin::new(pin as u64);
        let _ = pin.set_direction(::sysfs_gpio::Direction::Out);
        let _ = pin.export();

        SimpleTempRegulator { pin, threshold }
    }
}

impl Regulator for SimpleTempRegulator {
    fn profile_changed(&mut self, profile: &::repitile_core::profile::Profile) {
        self.threshold = profile.temp_range().min as u32;
    }

    fn update(&mut self, cc: &CurrentConditions) {
        if cc.temp < self.threshold {
            let _ = self.pin.set_value(1);
        } else {
            let _ = self.pin.set_value(0);
        }
    }
}

/// Simple "regulator" that enables/disables a humidity
/// indicator LED.
pub struct SimpleHumidityRegulator {
    pin: Pin,
    threshold: u32,
}

impl SimpleHumidityRegulator {
    /// Creates a new `SimpleHumidityRegulator` from a GPIO pin number
    /// and threshold value for activation.
    pub fn new(pin: u8, threshold: u32) -> SimpleHumidityRegulator {
        let pin = Pin::new(pin as u64);
        let _ = pin.set_direction(::sysfs_gpio::Direction::Out);
        let _ = pin.export();
        SimpleHumidityRegulator { pin, threshold }
    }
}

impl Regulator for SimpleHumidityRegulator {
    fn profile_changed(&mut self, profile: &::repitile_core::profile::Profile) {
        self.threshold = profile.humidity_range().min as u32;
    }

    fn update(&mut self, cc: &CurrentConditions) {
        if cc.humidity < self.threshold {
            let _ = self.pin.set_value(1);
        } else {
            let _ = self.pin.set_value(0);
        }
    }
}
