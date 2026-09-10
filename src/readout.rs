use crate::sensor_reading;

use std::env;
use linux_embedded_hal::{Delay, I2cdev};
use xca9548a::{SlaveAddr, Xca9548a};
use bme280::i2c::BME280;
use sensor_reading::SensorReading;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn data_readout() -> Vec<SensorReading> {
  let system_id = env::var("SYSTEM_ID").unwrap();
  let system_type = env::var("SYSTEM_TYPE").unwrap();

  let mut sensor_readings: Vec<SensorReading> = Vec::new();
  let start = SystemTime::now();
  let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
  let unix_timestamp = since_epoch.as_secs();
  
  for channel in 0..8 {
    init_channel(channel);
    let measurements = read_channel();
    let sensor_message = SensorReading {
      device_id: system_id.clone(),
      device_type: system_type.clone(),
      channel: channel,
      temperature: measurements[0],
      humidity: measurements[1],
      pressure: measurements[2],
      timestamp: unix_timestamp
    };
    sensor_readings.push(sensor_message);
  }
  sensor_readings
}

pub fn read_channel() -> Vec<f32> {
  let mut delay = Delay;
  let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
  let mut bme280 = BME280::new_primary(i2c_bus);
  bme280.init(&mut delay).unwrap();
  let reading = bme280.measure(&mut delay).unwrap();

  let mut measurements: Vec<f32> = Vec::with_capacity(3);
  measurements.push(reading.temperature);
  measurements.push(reading.humidity);
  measurements.push(reading.pressure);
  measurements
}

pub fn init_channel(channel: u8) {
  let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
  let address = SlaveAddr::default();
  let mut mux = Xca9548a::new(i2c_bus, address);
  mux.select_channels(channel).unwrap();
}