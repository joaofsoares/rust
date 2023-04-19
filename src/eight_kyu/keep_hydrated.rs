pub fn litres(time: f64) -> i32 {
    time as i32 / 2
}

#[cfg(test)]
mod keep_hydrated_tests {
    use super::*;

    #[test]
    fn test_liters() {
        assert_eq!(1, litres(3.0));
    }
}
