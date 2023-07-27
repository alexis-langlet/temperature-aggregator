use anyhow::{Result, anyhow};
use crate::TemperatureUnit;

/// Takes a temperature in milli-units and converts it to the given milli-unit
pub fn convert_temperature(
    temperature: i32,
    from_unit: TemperatureUnit,
    to_unit: TemperatureUnit,
) -> Result<i32> {
    match to_unit {
        TemperatureUnit::MilliCelsius => convert_to_millicelsius(temperature, from_unit),
        TemperatureUnit::MilliFahrenheit => convert_to_millifahrenheit(temperature, from_unit),
        TemperatureUnit::MilliKelvin => convert_to_millikelvin(temperature, from_unit),
    }
}

/// Takes a temperature in milli-units and converts it to millidegree celsius
fn convert_to_millicelsius(
    temperature: i32,
    unit: TemperatureUnit,
) -> Result<i32> {
    let converted_temperature = match unit {
        TemperatureUnit::MilliCelsius => temperature as i64,
        TemperatureUnit::MilliFahrenheit => (temperature as i64 - 32000) * 5 / 9,
        TemperatureUnit::MilliKelvin => temperature as i64 - 273150,
    };
    try_convert(converted_temperature)
}

/// Takes a temperature in milli-units and converts it to millidegree fahrenheit
fn convert_to_millifahrenheit(
    temperature: i32,
    unit: TemperatureUnit,
) -> Result<i32> {
    let converted_temperature = match unit {
        TemperatureUnit::MilliCelsius => temperature as i64 * 9 / 5 + 32000,
        TemperatureUnit::MilliFahrenheit => temperature as i64,
        TemperatureUnit::MilliKelvin => (temperature as i64 - 273150) * 9 / 5 + 32000,
    };
    try_convert(converted_temperature)
}

/// Takes a temperature in milli-units and converts it to milli-kelvin
fn convert_to_millikelvin(temperature: i32, unit: TemperatureUnit) -> Result<i32> {
    let converted_temperature = match unit {
        TemperatureUnit::MilliCelsius => temperature as i64 + 273150,
        TemperatureUnit::MilliFahrenheit => (temperature as i64 - 32000) * 5 / 9 + 273150,
        TemperatureUnit::MilliKelvin => temperature as i64,
    };
    try_convert(converted_temperature)
}

/// Try to convert a temperature from i64 to i32
/// If the temperature is too high or too low, return an error
fn try_convert(temperature: i64) -> Result<i32> {
    temperature.try_into().map_err(|_| {
        if temperature < 0 {
            anyhow!("Temperature {} is too low to be converted to this unit", temperature)
        } else {
            anyhow!("Temperature {} is too high to be converted to this unit", temperature)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

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
    fn temperature_will_be_valid_after_conversion(
        temperature: i32,
        from_unit: TemperatureUnit,
        to_unit: TemperatureUnit,
    ) -> bool {
        temperature >= convert_temperature(i32::MIN, to_unit, from_unit).unwrap_or(i32::MIN)
            && temperature <= convert_temperature(i32::MAX, to_unit, from_unit).unwrap_or(i32::MAX)
    }

    #[quickcheck]
    fn test_conversion_raise_error_when_needed(
        temperature: i32,
        from_unit_fixture: TemperatureUnitFixture,
        to_unit_fixture: TemperatureUnitFixture,
    ) {
        let from_unit = from_unit_fixture.unit;
        let to_unit = to_unit_fixture.unit;
        if !temperature_will_be_valid_after_conversion(temperature, from_unit, to_unit) {
            assert!(convert_temperature(temperature, from_unit, to_unit).is_err());
        }
    }

    #[quickcheck]
    fn test_conversion_does_not_raise_error_in_normal_case(
        temperature: i32,
        from_unit_fixture: TemperatureUnitFixture,
        to_unit_fixture: TemperatureUnitFixture,
    ) {
        let from_unit = from_unit_fixture.unit;
        let to_unit = to_unit_fixture.unit;
        if temperature_will_be_valid_after_conversion(temperature, from_unit, to_unit) {
            assert!(convert_temperature(temperature, from_unit, to_unit).is_ok());
        }
    }

    #[quickcheck]
    fn test_temperature_conversion(
        temperature: i32,
        from_unit_fixture: TemperatureUnitFixture,
        to_unit_fixture: TemperatureUnitFixture,
    ) {
        let from_unit = from_unit_fixture.unit;
        let to_unit = to_unit_fixture.unit;
        if !temperature_will_be_valid_after_conversion(temperature, from_unit, to_unit) {
            return;
        }
        let converted_temperature = convert_temperature(temperature, from_unit, to_unit).unwrap();
        if !temperature_will_be_valid_after_conversion(converted_temperature, from_unit, to_unit) {
            return;
        }
        let converted_back_temperature =
            convert_temperature(converted_temperature, to_unit, from_unit).unwrap();
        // let's say that the conversion is correct if the result is within 1 degree of the original temperature
        assert_eq!(temperature as i32 / 1000, converted_back_temperature / 1000);
        return;
    }
}
