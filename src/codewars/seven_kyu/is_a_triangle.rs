pub fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    a + b > c && a + c > b && b + c > a
}

#[cfg(test)]
mod triangle_tests {
    use super::*;

    #[test]
    fn is_triangle_test() {
        assert!(is_triangle(5, 5, 7));
    }
}
