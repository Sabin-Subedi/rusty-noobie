pub fn convert_fahrenhiet_to_celcius(number: i32) -> f64 {
    (((number as f64 - 32_f64) / 180_f64) * 100_f64) as f64
}
