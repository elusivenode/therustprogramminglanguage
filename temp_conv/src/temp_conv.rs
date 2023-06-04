pub fn temp_conv(temp: f64, scale: &str) -> (f64, &str) {
    let scale = scale.to_uppercase();
    if scale == "F" {
        ((temp - 32.0) * 5.0 / 9.0, "C")
    } else {
        (temp * 9.0 / 5.0 + 32.0, "F")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv() {
        let result = temp_conv(30.0, "f");
        assert_eq!(((result.0 * 100.0).round() / 100.0, result.1), (-1.11, "C"));
    }
}
