use crate::utils::temperature::converter::convert_temperature;
use crate::TemperatureUnit;
use anyhow::Result;

#[derive(Debug)]
pub struct Temperature {
    /// Temperature in millidegree Celsius
    temperature: i32,
}

impl Temperature {
    pub fn new(temperature: i32, unit: TemperatureUnit) -> Result<Temperature> {
        let millidegree_celsius_temperature =
            convert_temperature(temperature, unit, TemperatureUnit::MilliCelsius)?;
        Ok(Temperature {
            temperature: millidegree_celsius_temperature,
        })
    }

    /// Returns the temperature in the given unit
    pub fn get_temperature_value(&self, unit: TemperatureUnit) -> Result<i32> {
        let temperature =
            convert_temperature(self.temperature, TemperatureUnit::MilliCelsius, unit)?;
        Ok(temperature)
    }

    /// Returns true if the temperature (caused only by the weather) can be found on Earth
    pub fn is_terrestrial(&self) -> bool {
        if (-100000..=100000).contains(&self.temperature) {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    use super::*;

    #[derive(Clone, Debug)]
    struct TemperatureUnitFixture {
        unit: TemperatureUnit,
    }

    impl Arbitrary for TemperatureUnitFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let unit = match u8::arbitrary(g) % 3 {
                0 => TemperatureUnit::MilliCelsius,
                1 => TemperatureUnit::MilliFahrenheit,
                _ => TemperatureUnit::MilliKelvin,
            };
            TemperatureUnitFixture { unit }
        }
    }

    /// check if temperature is valid after conversion
    /// That is done by taking the min and max values of i32 and converting them backward
    fn conversion_should_raise_error(
        temperature: i32,
        from_unit: TemperatureUnit,
        to_unit: TemperatureUnit,
    ) -> bool {
        !(temperature >= convert_temperature(i32::MIN, to_unit, from_unit).unwrap_or(i32::MIN)
            && temperature <= convert_temperature(i32::MAX, to_unit, from_unit).unwrap_or(i32::MAX))
    }

    #[quickcheck]
    fn test_create_and_get_temperature(
        temperature_value: i32,
        unit_fixture: TemperatureUnitFixture,
    ) {
        let temp = Temperature::new(temperature_value, unit_fixture.unit);

        if conversion_should_raise_error(
            temperature_value,
            unit_fixture.unit,
            TemperatureUnit::MilliCelsius,
        ) {
            assert!(temp.is_err());
            return;
        }
        let temperature = temp.unwrap();

        let retrieved_temp = temperature.get_temperature_value(unit_fixture.unit);
        if conversion_should_raise_error(
            convert_temperature(
                temperature_value,
                unit_fixture.unit,
                TemperatureUnit::MilliCelsius,
            )
            .unwrap(),
            TemperatureUnit::MilliCelsius,
            unit_fixture.unit,
        ) {
            assert!(retrieved_temp.is_err());
            return;
        }

        let retrieved_temperature_value = retrieved_temp.unwrap();
        assert_eq!(temperature_value / 1000, retrieved_temperature_value / 1000);
    }

    #[test]
    fn test_temperature_invalid_if_too_low() {
        let unit = TemperatureUnit::MilliCelsius;
        assert_eq!(Temperature::new(-273000, unit).unwrap().is_terrestrial(), false);
        assert_eq!(Temperature::new(-1000000, unit).unwrap().is_terrestrial(), false);
    }

    #[test]
    fn test_temperature_invalid_if_too_high() {
        let unit = TemperatureUnit::MilliCelsius;
        assert_eq!(Temperature::new(200000, unit).unwrap().is_terrestrial(), false);
        assert_eq!(Temperature::new(1000000, unit).unwrap().is_terrestrial(), false);
    }

    #[test]
    fn test_temperature_valid_in_usual_case() {
        let unit = TemperatureUnit::MilliCelsius;
        assert!(Temperature::new(0, unit).unwrap().is_terrestrial());
        assert!(Temperature::new(-10000, unit).unwrap().is_terrestrial());
        assert!(Temperature::new(40000, unit).unwrap().is_terrestrial());
        assert!(Temperature::new(58000, unit).unwrap().is_terrestrial()); // max temperature seen on Earth
        assert!(Temperature::new(-88000, unit).unwrap().is_terrestrial()); // min temperature seen on Earth
    }
}
