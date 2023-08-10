use std::collections::HashMap;

pub fn naive_fib(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        naive_fib(n - 1) + naive_fib(n - 2)
    }
}

pub fn memo_fib(n: u32) -> u32 {
    let mut memo: HashMap<u32, u32> = HashMap::new();

    if memo.contains_key(&n) {
        memo.get(&n).unwrap().clone()
    } else {
        let res = naive_fib(n);
        memo.insert(n, res);
        res
    }
}
