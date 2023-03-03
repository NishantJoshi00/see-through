use see_through::{see_derive::Look, Look, See};

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

#[derive(Look)]
struct Card<T> {
    name: String,
    number: T,
}

#[derive(Look)]
struct Shoe<T> {
    brand: T,
    number: i32,
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

fn change_i32_number<T>(mut var: T, by: i32) -> T
where
    T: Look<see_t::NUMBER, Inner = i32>,
{
    var[see_t::NUMBER] += by;
    var
}

fn look_both_use_get_one<T, U: Copy>(var: T) -> U
where
    T: Look<see_t::X, Inner = U> + Look<see_t::Y, Inner = U>,
{
    var[see_t::X]
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

#[test]
fn test_multiple_look() {
    let p1 = Point { x: 12, y: 1 };
    let v1 = Vector { x: 100, y: 120 };

    assert_eq!(look_both_use_get_one(p1), 12);
    assert_eq!(look_both_use_get_one(v1), 100);
}

#[test]
fn test_generic_happy_case() {
    let shoe = Shoe {
        brand: "my",
        number: 2,
    };
    let card = Card {
        name: "dev".to_string(),
        number: 2,
    };

    let card = change_i32_number(card, 2);
    let shoe = change_i32_number(shoe, 2);

    assert_eq!(card.number, shoe.number);
}
#[test]
fn test_generic_compile_failure() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}

see_derive::auto_load!();
