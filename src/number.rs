#![allow(dead_code)]
fn add(x: u8, y: u8) -> u8 {
    x + y
}

fn to_float(x: u8) -> f32 {
    x.into()
}

fn str_to_i32(s: impl Into<String>) -> Option<i32> {
    let v = s.into().parse::<i32>();
    match v {
        Ok(i) => Some(i),
        _ => None,
    }
}

#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
}

#[test]
fn test_mut_reference() {
    let mut i: i32 = 1;
    let ref_i: &mut i32 = &mut i;
    *ref_i = 2;
    assert_eq!(i, 2)
}

fn sum(nums: Vec<&str>) -> Option<i32> {
    nums.iter().map(|i| str_to_i32(*i)).sum()
}

#[test]
fn test_sum() {
    assert_eq!(Some(0), sum(vec!["1", "-1"]));
    assert_eq!(Some(10), sum(vec!["5", "1", "1", "3"]));
    assert_eq!(None, sum(vec!["abc", "1", "1", "3"]));
    assert_eq!(11, sum(vec!["5", "1", "1", "4"]).unwrap());
}

fn fib(n: u32) -> u32 {
    let mut current = (0, 1);
    (0..n).for_each(|_| {
        current = dbg!((current.1, current.0 + current.1));
    });
    current.0
}

#[test]
fn test_fib() {
    assert_eq!(0, fib(0));
    assert_eq!(1, fib(1));
    assert_eq!(1, fib(2));
    assert_eq!(2, fib(3));
    assert_eq!(3, fib(4));
    assert_eq!(6765, fib(20));
}

#[test]
fn test_range() {
    assert_eq!([0, 1, 2], (0..=2).collect::<Vec<u16>>()[..]);
    assert_eq!([0, 1, 2], (0..3).collect::<Vec<u16>>()[..]);
    assert_eq!(vec![0, 1, 2], (0..3).collect::<Vec<u16>>());
}

#[test]
fn test_for() {
    for n in 0..=100 {
        if let 0..=50 = n {
            assert!(n <= 50)
        } else {
            assert!(n > 50)
        };
    }
    for n in 0..=100 {
        match n {
            v if v % 2 == 0 => assert!(v % 2 == 0),
            v if v % 2 != 0 => assert!(v % 2 != 0),
            0..=99 => unreachable!(),
            _ => unreachable!(),
        }
    }
}

fn inner_collatz_length(n: i32) -> i32 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn collatz_length(n: &mut i32) -> u32 {
    let mut step = 1;
    while *n > 1 {
        *n = inner_collatz_length(*n);
        step += 1
    }
    step
}

#[test]
fn test_collatz_length() {
    let mut n = 3; // mutable borrow
    assert_eq!(8, collatz_length(&mut n));
    assert_eq!(n, 1);
}

fn transpose(m: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut n = m;
    for y in 0..n.len() {
        for x in 0..n.len() {
            n[x][y] = m[y][x];
        }
    }
    n
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn magnitude(v: &[f64]) -> f64 {
    v.iter().map(|&i| i * i).sum::<f64>().sqrt()
}

fn normalize(v: &mut [f64]) {
    let m = magnitude(v);
    for val in v {
        *val /= m;
    }
}

#[test]
fn test_magnitude_normalize() {
    let mut v = [1.0, 0.0, 0.0];
    assert_eq!(1.0, magnitude(&v));
    normalize(&mut v);
    assert_eq!(1.0, magnitude(&v));
}

#[derive(Debug, PartialEq, Eq)]
/// An event in the elevator system that the controller must react to.
enum Event {
    ButtonPressed(Button),
    CarAt(Floor),
    DoorOpened(bool),
}

/// A direction of travel.
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

type Floor = i32;

#[derive(Debug, PartialEq, Eq)]
enum Button {
    CarFloor(Floor),
    LobbyCall(Floor, Direction),
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarAt(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::DoorOpened(true)
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::DoorOpened(false)
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(floor, dir))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

#[test]
fn test_enum() {
    assert_eq!(
        Event::ButtonPressed(Button::LobbyCall(0, Direction::Up)),
        lobby_call_button_pressed(0, Direction::Up)
    );
    assert_eq!(Event::CarAt(0), car_arrived(0));
    assert_eq!(Event::DoorOpened(true), car_door_opened());
    assert_eq!(
        Event::ButtonPressed(Button::CarFloor(3)),
        car_floor_button_pressed(3)
    );
    assert_eq!(Event::DoorOpened(false), car_door_closed());
    assert_eq!(Event::CarAt(3), car_arrived(3));
}
