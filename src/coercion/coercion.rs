use std::fmt::Display;

fn show(val: &dyn Display) {
    println!("{}", val);
}

pub fn coercion() {
    let x: i32 = 42;
    let r: &i32 = &x;
    show(r); // Coercion implicita: &i32 → &dyn Display

    let mut x = 10;
    let r_mut: &mut i32 = &mut x;
    read(r_mut); // Coercion implicita da &mut i32 a &i32
}

fn read(val: &i32) {
    println!("Valore: {}", val);
}
