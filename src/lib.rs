pub mod utils;
pub mod temperature;
pub mod temperature_service;

#[derive(Clone, Copy, Debug)]
pub enum TemperatureUnit {
    MilliCelsius,
    MilliFahrenheit,
    MilliKelvin,
}
