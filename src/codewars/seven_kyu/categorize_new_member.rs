pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.into_iter()
        .map(|(age, handcap)| {
            if (age >= 55) && (handcap > 7) {
                String::from("Senior")
            } else {
                String::from("Open")
            }
        })
        .collect()
}
