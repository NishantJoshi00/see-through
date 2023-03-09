use see_through::{see_derive::Look, Look, See};

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


fn change_i32_number<T>(mut var: T, by: i32) -> T
where
    T: Look<see_t::number, Inner = i32>,
{
    var[see_t::number] += by;
    var
}


fn main() {
    
    let shoe = Shoe {
        brand: "my",
        number: 2,
    };
    let card = Card {
        name: "dev".to_string(),
        number: "2",
    };

    let card = change_i32_number(card, 2); // <-- This won't work
    let shoe = change_i32_number(shoe, 2);
}


see_derive::auto_load!();
