use see_derive::{Load, Look};
use see_through::{Look, See};

#[derive(Look)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Look)]
struct Vector {
    x: i32,
    y: i32,
}

fn modify_y<T>(var: &mut T)
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
fn test_look() {
    let mut p1 = Point { x: 12, y: 14 };
    let mut v1 = Vector { x: 12, y: 14 };
    modify_x(&mut v1);
    modify_x(&mut p1);
    assert_eq!(p1.x, 24);
    assert_eq!(v1.x, 24);
}

#[test]
fn test_see() {
    let mut p1 = Point { x: 12, y: 1 };
    let mut v1 = Vector { x: 100, y: 120 };
    modify_y(&mut v1);
    assert_eq!(v1.y, 121);
    modify_y(&mut p1);
    assert_eq!(p1.y, 2);
}

pub(crate) mod see_t {
    use super::Load;
    #[derive(Load)]
    struct _SeeT;
}
