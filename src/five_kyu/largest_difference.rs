pub fn largest_difference(data: &[i32]) -> usize {
    let mut diff: usize = 0;

    for (ik, iv) in data.into_iter().enumerate() {
        for (jk, kv) in data.into_iter().enumerate() {
            if iv <= kv {
                let d: usize = if jk > ik { jk - ik } else { 0 };

                if d > diff {
                    diff = d
                };
            }
        }
    }

    diff
}
