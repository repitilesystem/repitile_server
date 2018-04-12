//! `repitile_server` uses the `repitile_core` crate, along with
//! a `gotham` REST API server to communicate with the UI and update
//! profiles, configurations, temperature, humidity, etc.

#![deny(missing_docs)]

extern crate futures;
extern crate gotham;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate dht22_pi;
extern crate mime;
extern crate repitile_core;
extern crate sysfs_gpio;

mod rest_server;
mod simple_sensor;
mod simple_regulator;

use std::thread;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::Mutex;

use repitile_core::CommReq;
use repitile_core::profile::Profile;
use repitile_core::config::Configuration;

use simple_sensor::SimpleSensor;
use simple_regulator::{SimpleHumidityRegulator, SimpleTempRegulator};

lazy_static! {
    static ref GOTHAM_CHANNEL_SERVER_REQ: (
    SyncSender<repitile_core::CommReq>,
    Mutex<Receiver<repitile_core::CommReq>>
) = {
        let (tx, rx) = mpsc::sync_channel(1);
        (tx, Mutex::new(rx))
    };
    static ref GOTHAM_CHANNEL_SERVER_RESP: (
    SyncSender<repitile_core::CommReq>,
    Mutex<Receiver<repitile_core::CommReq>>
) = {
        let (tx, rx) = mpsc::sync_channel(1);
        (tx, Mutex::new(rx))
    };
}

fn main() {
    let (core_send, from_core) = mpsc::sync_channel(1);
    let (to_core, from_server) = mpsc::sync_channel(1);

    let config = Configuration::load_default().unwrap();
    let profile = Profile::read_from_file(&config.default_profile).unwrap();

    let mut core = repitile_core::Core::new(config, profile);

    rest_server::start();

    let temp_min = core.profile().temp_range().min as u32;
    let humid_min = core.profile().humidity_range().min as u32;

    core.add_sensor(SimpleSensor::new(1));
    core.add_sensor(SimpleSensor::new(2));
    core.add_regulator(SimpleHumidityRegulator::new(3, humid_min));
    core.add_regulator(SimpleTempRegulator::new(4, temp_min));

    let tr = thread::spawn(move || {
        core.run(core_send, from_server);
    });

    while let Ok(req) = (*GOTHAM_CHANNEL_SERVER_REQ).1.lock().unwrap().recv() {
        match req {
            CommReq::OpenProfile(s) => {
                to_core.send(CommReq::OpenProfile(s)).unwrap();

                if let Ok(resp) = from_core.recv() {
                    if let CommReq::Ok = resp {
                        (*GOTHAM_CHANNEL_SERVER_RESP).0.send(CommReq::Ok).unwrap();
                    } else {
                        (*GOTHAM_CHANNEL_SERVER_RESP).0.send(CommReq::Err).unwrap();
                    }
                }
            }
            CommReq::OpenConfig(s) => {
                to_core.send(CommReq::OpenConfig(s)).unwrap();

                if let Ok(resp) = from_core.recv() {
                    if let CommReq::Ok = resp {
                        (*GOTHAM_CHANNEL_SERVER_RESP).0.send(CommReq::Ok).unwrap();
                    } else {
                        (*GOTHAM_CHANNEL_SERVER_RESP).0.send(CommReq::Err).unwrap();
                    }
                }
            }
            _ => {}
        }
    }

    tr.join().unwrap();
}
