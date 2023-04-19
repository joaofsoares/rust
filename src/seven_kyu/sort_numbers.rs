pub fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    sorted_arr
}

#[cfg(test)]
mod sort_numbers_tests {
    use super::*;

    #[test]
    fn sort_numbers_test() {
        let list = vec![1, 5, 2, 3, 4];
        assert_eq!(vec![1, 2, 3, 4, 5], sort_numbers(&list));
    }
}
