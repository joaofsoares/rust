pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = 0;
    let mut sell = 1;

    let mut profit = 0;

    while sell < prices.len() {
        if prices.iter().nth(buy).unwrap() < prices.iter().nth(sell).unwrap() {
            let tmp = prices.iter().nth(sell).unwrap() - prices.iter().nth(buy).unwrap();
            profit = std::cmp::max(profit, tmp);
        } else {
            buy = sell;
        }
        sell += 1;
    }

    profit
}
