## See

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Every wanted to write incredibly generic code? and in the middle of it wanted to access a field inside it? We generally end up using a trait and binding the behavior of accessing to certain ambiguous function and then forgetting it. 
It's alright we all have done it, it's so common in fact that other generic and statically typed language provide us with a way to do that. It's call the lens operator. So, why not employ it for rust too.

## Usage

There are exactly 3 traits that we need to concern ourself with, `See`, `Load` and a special trait `Look`. Derive `See` on your structure and bam! now your `struct` has implemented `See` for all it's fields. 
But it won't take effect on the counter part it will throw some error, it's alright thats where `Load` trait comes into picture. The sole purpose of this trait is to load all the visitor for any fields that were found during derivation. So, the this reason consider using it after all the `#[derive(See)]` in your codebase.



## Extending See

Though `See` provides us with ample ways to access inner field states, yet the syntax isn't friendly and easily understandable for someone new to rust. To counteract this `Look` trait is implemented. Though by looking at the macro expansion it isn't obvious. It's actually auto implemented after certain conditions are met.

## Example

Checkout this example, it will make a lot of thing clear

```rust
use see::{See, Look, see_derive::See};
//        ↑                      ↑
//        |                      +------ The derive
//        +----------------------------- The trait           

#[derive(See)]
struct Point2D {
    x: i32,
    y: i32
}

#[derive(See)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

// Once done using the derive or loading modules that have these derive calls do this
pub(crate) mod see_t {
    #[derive(Load)]
    struct SeeT;
}

// Done! Now let's write a function that modifies x coordinate

fn modify_x<T>(loc: &mut T, change: i32) 
where
    T: See<see_t::X, Inner = i32> 
    // use see_t for getting the visitors and just specify your field with 2 underscores, 
    // and if need you can specify the type of inner or a morphism that's allowed on the inner type
{
    *loc.set() = change;
}

fn modify_y<T>(loc: &mut T, change: i32)
where
    T: Look<see_t::Y, Inner = i32>
    // `Look` is exactly similar to `See`
{
    loc[see_t::Y] = change;
}

fn main3d() {
    let p = Point3D {
        x: 0,
        y: 0,
        z: 0
    };
    // go left
    modify_x(&mut p, -1);
    // go right
    modify_x(&mut p, 1);
}
fn main2d() {
    let p = Point2D {
        x: 0,
        y: 0
    };
    // go left
    modify_x(&mut p, -1);
    // go right
    modify_x(&mut p, 1);
}
```

## Features

- Use the `See` trait to get raw access to the methods
    1. `fn get(&self) -> &Self::Inner` here the inner is the type of the data in question
    2. `fn set(&mut self) -> &mut Self::Inner`
- Use the `Look` trait to have similar access, as it extends the `See` trait. But also allows more verbose way of access control while also allowing a simpler interface when trying to access multiple fields from a generic struct


## Contributing

- Follow conventional comments.
- Create issue for any change request
- Provide proper reference and description for PR 