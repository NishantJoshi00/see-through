use std::ops;

use see_derive::{Load, See};
use see_through::{Look, See};

#[derive(See)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(See)]
struct Vector {
    x: i32,
    y: i32,
}

fn modify<T>(var: &mut T)
where
    T: See<see_t::Y, Inner = i32>,
{
    *var.set() += 1;
}

fn modify_x<T>(var: &mut T)
where
    T: Look<see_t::X, Inner = i32>,
{
    var[see_t::X] += 12;
}

#[test]
fn test_r1() {
    let mut p1 = Point { x: 12, y: 14 };
    let mut v1 = Vector { x: 12, y: 14 };
    modify_x(&mut p1);
    modify_x(&mut v1);
    assert_eq!(p1.x, 24);
    assert_eq!(v1.x, 24);
}

pub(crate) mod see_t {
    use super::Load;
    #[derive(Load)]
    struct SeeT;
}
