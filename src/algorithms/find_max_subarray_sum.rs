use std::cmp;

fn max_cross_subarray(list: &Vec<i32>, low: i32, mid: i32, high: i32) -> i32 {
    let mut sum = 0;
    let mut max_left = std::i32::MIN;

    let mut cnt = mid;
    while cnt >= low {
        sum += list[cnt as usize];
        if sum > max_left {
            max_left = sum;
        }
        cnt -= 1;
    }

    sum = 0;
    let mut max_right = std::i32::MIN;
    cnt = mid + 1;
    while cnt <= high {
        sum += list[cnt as usize];
        if sum > max_right {
            max_right = sum;
        }
        cnt += 1;
    }

    return max_left + max_right;
}

pub fn max_subarray_sum(list: &Vec<i32>, low: i32, high: i32) -> i32 {
    if low == high {
        return list[low as usize];
    } else {
        let mid = low + (high - low) / 2;
        let left_max_sum = max_subarray_sum(list, low, mid);
        let right_max_sum = max_subarray_sum(list, mid + 1, high);
        let cross_max_sum = max_cross_subarray(list, low, mid, high);

        cmp::max(left_max_sum, cmp::max(right_max_sum, cross_max_sum))
    }
}

#[cfg(test)]
mod subarray_sum_tests {
    use super::*;

    #[test]
    fn subarray_sum_test_one() {
        let arr = vec![-4, 5, 7, -6, 10, -15, 3];
        let res = max_subarray_sum(&arr, 0, (arr.len() - 1) as i32);
        assert_eq!(res, 16);
    }

    #[test]
    fn subarray_sum_test_two() {
        let arr = vec![-3, 2, -1, 4, -2];
        let res = max_subarray_sum(&arr, 0, (arr.len() - 1) as i32);
        assert_eq!(res, 5);
    }

    #[test]
    fn subarray_sum_test_three() {
        let arr = vec![5, 7, 6, 10, 3];
        let res = max_subarray_sum(&arr, 0, (arr.len() - 1) as i32);
        assert_eq!(res, 31);
    }
}
