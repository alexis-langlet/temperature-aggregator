use crate::temperature::Temperature;
use crate::TemperatureUnit;
use anyhow::{Result, bail};

#[derive(Debug, Default)]
pub struct TemperatureService{
    temperatures: Vec<Temperature>,
}

impl TemperatureService {
    pub fn new() -> TemperatureService {
        TemperatureService {
            temperatures: Vec::new(),
        }
    }

    pub fn add_temperature(&mut self, temperature_value: i32, unit: TemperatureUnit) -> Result<()>  {
        let temperature = Temperature::new(temperature_value, unit)?;
        if !temperature.is_terrestrial()
        {
            bail!("Submitted temperature is probably not terrestrial")
        }
        self.temperatures.push(temperature);
        Ok(())
    }

    pub fn get_average(&self) -> i32 {
        let mut sum = 0;
        for temperature in &self.temperatures {
            sum += temperature.get_temperature_value(TemperatureUnit::MilliCelsius).unwrap();
        }
        sum / self.temperatures.len() as i32
    }
}