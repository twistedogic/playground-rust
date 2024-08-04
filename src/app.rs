use std::time::Duration;

#[derive(Debug)]
#[allow(dead_code)]
struct Group {
    name: String,
    weight: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Item {
    name: String,
    tags: Vec<Group>,
    estimate: Duration,
}
