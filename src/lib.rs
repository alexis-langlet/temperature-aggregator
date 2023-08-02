pub mod utils;
pub mod temperature;

#[derive(Clone, Copy, Debug)]
pub enum TemperatureUnit {
    MilliCelsius,
    MilliFahrenheit,
    MilliKelvin,
}
