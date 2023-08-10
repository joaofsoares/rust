use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_counter() {
    // Mutex usage
    let cnt = 0;
    let mutex_cnt = Mutex::new(cnt);

    for _ in 0..10 {
        let mut num = mutex_cnt.lock().unwrap();
        *num = *num + 1;
    }

    println!("m = {:?}", mutex_cnt);
    println!("mutex result = {}", mutex_cnt.lock().unwrap());
}

pub fn arc_mutex_increment() {
    // Arc Mutex usage
    let data = vec![1, 2, 3, 4, 5];
    let arc_data = Arc::new(Mutex::new(data));
    let mut handles = vec![];

    for _ in 0..10 {
        let data_clone = Arc::clone(&arc_data);

        let handle = thread::spawn(move || {
            let mut d = data_clone.lock().unwrap();

            let last = *d.last().unwrap();

            d.push(last + 1);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("arc data = {:?}", arc_data);
    println!("arc data result = {:?}", arc_data.lock().unwrap());
}
