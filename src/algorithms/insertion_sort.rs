use std::usize;

pub fn insertion_sort(list: &mut Vec<i32>) {
    for j in 1..list.len() {
        let key = list[j];
        let mut i: i32 = j as i32 - 1;

        while i >= 0 && list[i as usize] > key {
            list[(i + 1) as usize] = list[i as usize];
            i -= 1;
        }

        list[(i + 1) as usize] = key;
    }
}

#[cfg(test)]
mod insertion_sort_tests {
    use super::*;

    #[test]
    fn insertion_sort_test_one() {
        let mut list = vec![5, 2, 4, 6, 1, 3];
        let expected = vec![1, 2, 3, 4, 5, 6];
        insertion_sort(&mut list);
        assert_eq!(expected, list);
    }
}
