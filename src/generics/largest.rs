pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod generic_tests {
    use super::*;

    #[test]
    fn test_largest() {
        let nums = vec![1, 23, 47, 35];
        let chars = vec!['a', 'j', 'b', 'v', 'z'];

        assert_eq!(&47, largest(&nums));
        assert_eq!(&'z', largest(&chars));
    }
}
