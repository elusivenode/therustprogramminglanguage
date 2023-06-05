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
        let result = temp_conv(95.6, "f");
        assert_eq!(((result.0 * 100.0).round() / 100.0, result.1), (35.33, "C"));
        let result = temp_conv(41.0, "c");
        assert_eq!(((result.0 * 100.0).round() / 100.0, result.1), (105.8, "F"));
        let result = temp_conv(-1.0, "c");
        assert_eq!(((result.0 * 100.0).round() / 100.0, result.1), (30.2, "F"));
        let result = temp_conv(-25.5, "c");
        assert_eq!(((result.0 * 100.0).round() / 100.0, result.1), (-13.9, "F"));
        let result = temp_conv(-25.5, "f");
        assert_eq!(
            ((result.0 * 100.0).round() / 100.0, result.1),
            (-31.94, "C")
        );
    }
}
