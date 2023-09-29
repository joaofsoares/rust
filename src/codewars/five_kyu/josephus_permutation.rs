pub fn josephus<T: Clone + Copy>(xs: Vec<T>, k: usize) -> Vec<T> {
    let mut deads: Vec<T> = Vec::new();
    let mut cnt: usize = k - 1;

    let mut alive: Vec<T> = xs.to_vec();
    let mut size: usize = alive.len();

    loop {
        if alive.is_empty() {
            break;
        }

        while cnt >= size {
            cnt = cnt - size;
        }

        deads.push(alive[cnt]);

        alive.remove(cnt);
        size = alive.len();

        cnt += k - 1;
    }

    deads
}
