use std::usize;

pub fn insertion_sort(list: &mut Vec<u32>) {
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

pub fn insertion_sort_rev(list: &mut Vec<u32>) {
    for j in 1..list.len() {
        let key = list[j];
        let mut i: i32 = j as i32 - 1;

        while i >= 0 && list[i as usize] < key {
            list[(i + 1) as usize] = list[i as usize];
            i -= 1;
        }

        list[(i + 1) as usize] = key;
    }
}

// searching problem - linear search
pub fn searching_problem(list: Vec<u32>, target: u32) -> Option<usize> {
    for i in 0..list.len() {
        if list[i] == target {
            return Some(i);
        }
    }
    None
}

pub fn add_binaries(a: u32, b: u32) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    let bin_a = format!("{:b}", a);
    let bin_b = format!("{:b}", b);

    let mut carry = 0;
    let mut a_size: i32 = bin_a.len() as i32 - 1;
    let mut b_size: i32 = bin_b.len() as i32 - 1;

    while a_size > -1 && b_size > -1 {
        let a_num = bin_a.chars().nth(a_size as usize).unwrap() as u32 - '0' as u32;
        let b_num = bin_b.chars().nth(b_size as usize).unwrap() as u32 - '0' as u32;

        if a_num + b_num + carry == 3 {
            carry = 1;
            res.push(1);
        } else if a_num + b_num + carry == 2 {
            carry = 1;
            res.push(0);
        } else if a_num + b_num + carry == 1 {
            carry = 0;
            res.push(1);
        } else {
            carry = 0;
            res.push(0);
        }

        a_size -= 1;
        b_size -= 1;
    }

    while a_size > -1 {
        let a_num = bin_a.chars().nth(a_size as usize).unwrap() as u32 - '0' as u32;

        if a_num + carry == 2 {
            carry = 1;
            res.push(0);
        } else if a_num + carry == 1 {
            carry = 0;
            res.push(1);
        } else {
            carry = 0;
            res.push(0);
        }

        a_size -= 1;
    }

    while b_size > -1 {
        let b_num = bin_b.chars().nth(b_size as usize).unwrap() as u32 - '0' as u32;

        if b_num + carry == 2 {
            carry = 1;
            res.push(0);
        } else if b_num + carry == 1 {
            carry = 0;
            res.push(1);
        } else {
            carry = 0;
            res.push(0);
        }

        b_size -= 1;
    }

    if carry != 0 {
        res.push(carry);
    }

    res.reverse();
    res
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

    #[test]
    fn insertion_sort_test_two() {
        let mut list = vec![31, 41, 59, 26, 41, 58];
        let expected = vec![26, 31, 41, 41, 58, 59];
        insertion_sort(&mut list);
        assert_eq!(expected, list);
    }

    #[test]
    fn insertion_sort_test_rev_one() {
        let mut list = vec![5, 2, 4, 6, 1, 3];
        let expected = vec![6, 5, 4, 3, 2, 1];
        insertion_sort_rev(&mut list);
        assert_eq!(expected, list);
    }

    #[test]
    fn searching_problem_test_one() {
        let list = vec![31, 41, 59, 26, 41, 58];
        assert_eq!(2, searching_problem(list, 59).unwrap());
    }

    #[test]
    fn searching_problem_test_two() {
        let list = vec![31, 41, 59, 26, 41, 58];
        assert_eq!(searching_problem(list, 100), None);
    }

    #[test]
    fn add_binaries_test_one() {
        let a = 2;
        let b = 3;

        let expected = vec![1, 0, 1];
        let res = add_binaries(a, b);

        assert_eq!(expected, res);
    }

    #[test]
    fn add_binaries_test_two() {
        let a = 3; // 11
        let b = 7; // 111

        let expected = vec![1, 0, 1, 0];
        let res = add_binaries(a, b);

        assert_eq!(expected, res);
    }
}
