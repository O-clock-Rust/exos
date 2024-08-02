fn sqrt(value: f64) -> Result<f64, String> {
    if value < 0.0 {
        Err(String::from(
            "Cannot calculate the square root of a negative number",
        ))
    } else {
        Ok(value.sqrt())
    }
}

fn main() {
    let values = vec![4.0, -4.0, 9.0, -9.0, 16.0];

    for &value in &values {
        match sqrt(value) {
            Ok(result) => println!("The square root of {} is {}", value, result),
            Err(e) => println!("Error calculating the square root of {}: {}", value, e),
        }
    }
}
