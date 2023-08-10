mod actix;
mod async_messages;
mod codewars;
mod dynamic_programming;
mod generic;
mod guessing_game;
mod leetcode;
mod pointers;
mod rectangles;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_naive_test() {
        let result = dynamic_programming::fibonacci::naive_fib(10);
        assert_eq!(result, 55);
    }

    #[test]
    fn fibonacci_memo_test() {
        let result = dynamic_programming::fibonacci::memo_fib(10);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_liters() {
        let result = codewars::eight_kyu::keep_hydrated::litres(3.0);
        assert_eq!(1, result);
    }

    #[test]
    fn test_largest() {
        let nums = vec![1, 23, 47, 35];
        let chars = vec!['a', 'j', 'b', 'v', 'z'];

        assert_eq!(&47, generic::largest::largest(&nums));
        assert_eq!(&'z', generic::largest::largest(&chars));
    }
}
