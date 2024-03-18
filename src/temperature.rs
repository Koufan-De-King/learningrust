// formula: °C = (°F - 32) × 5/9
pub fn conv(celsius: i32) -> i32 {
    let fahrenheit= (celsius*9/5) + 32;
    fahrenheit
}