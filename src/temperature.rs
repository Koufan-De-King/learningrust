// formula: °C = (°F - 32) × 5/9
pub fn conv(celsius: f32) -> f32 {
    let fahrenheit: f32 = ((celsius*9.0/5.0) + 32.0) as f32;
    fahrenheit
}