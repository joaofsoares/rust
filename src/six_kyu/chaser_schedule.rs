pub fn run(speed: u32, time: u32) -> u32 {
    let mut distance = speed * time;
    let max_sprints = (time as f32 / 2 as f32).ceil() as u32;

    for i in 0..max_sprints {
        if (speed as i32) - 3 * (i as i32) > 0 {
            distance += speed - 3 * i;
        }
    }

    distance
}
