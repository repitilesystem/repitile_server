//! Defines a simple sensor that reads from our DHT22/RHT03 sensors
//! connected to the Pi.

use repitile_core;
use dht22_pi;

/// Simple sensor that reads from a specific GPIO pin.
pub struct SimpleSensor {
    pin: u8,
    temp: u32,
    humid: u32,
}

impl SimpleSensor {
    /// Creates a new `SimpleSensor` with the specified pin number.
    pub fn new(pin: u8) -> SimpleSensor {
        SimpleSensor {
            pin: pin,
            temp: 0,
            humid: 0,
        }
    }
}

impl repitile_core::sensor::Sensor for SimpleSensor {
    fn read(&mut self) {
        loop {
            if let Ok(reading) = dht22_pi::read(self.pin) {
                self.temp = reading.temperature as u32;
                self.humid = reading.humidity as u32;
                break;
            } else {
                println!("Error reading sensor on pin {}, trying again...", self.pin);
            }
        }
    }

    fn temperature(&self) -> u32 {
        self.temp
    }

    fn humidity(&self) -> u32 {
        self.humid
    }
}
