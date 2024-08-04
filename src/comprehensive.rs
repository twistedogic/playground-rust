#![allow(dead_code)]

struct Something {
    x: (u32, u32),
    y: u32,
}

#[test]
fn test_something() {
    let s = Something { x: (1, 2), y: 3 };
    match s {
        Something { x: (1, b), y: 3 } => assert_eq!(b, 2),
        Something { y: 2, x: i } => assert_eq!(i, (1, 2)),
        Something { y, .. } => assert_eq!(y, 3),
    }
}
