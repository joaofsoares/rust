pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut total = 0;
    let mut tank = 0;

    if gas.len() != cost.len() {
        return -1;
    }

    for n in 0..gas.len() {
        tank += gas.iter().nth(n).unwrap() - cost.iter().nth(n).unwrap();
        if tank < 0 {
            start = n + 1;
            total += tank;
            tank = 0;
        }
    }

    if total + tank < 0 {
        -1
    } else {
        start as i32
    }
}
