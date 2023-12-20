pub fn quick_sort(list: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pivot = partition(list, low, high);

        quick_sort(list, low, pivot - 1);
        quick_sort(list, pivot + 1, high)
    }
}

fn partition(list: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = list[high];
    let mut i = low as i32 - 1;

    for j in low..=high {
        if list[j] < pivot {
            i += 1;

            let tmp = list[i as usize];
            list[i as usize] = list[j];
            list[j] = tmp;
        }
    }

    let tmp = list[i as usize + 1];
    list[i as usize + 1] = list[high];
    list[high] = tmp;

    (i + 1) as usize
}

#[cfg(test)]
mod quick_sort_test {
    use super::*;

    #[test]
    fn quick_sort_test_one() {
        let mut xs = vec![8, 3, 6, 7, 0, 1, 2];
        let size = xs.len() - 1;
        quick_sort(&mut xs, 0, size);
        let expected = vec![0, 1, 2, 3, 6, 7, 8];
        assert_eq!(xs, expected);
    }
}
