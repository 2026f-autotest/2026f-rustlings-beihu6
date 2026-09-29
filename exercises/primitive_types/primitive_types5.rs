// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = (String::from("Furry McFurson"), 3.5);
    let temp = get_name_and_age(cat);
    println!("{} is {} years old.", temp.0, temp.1);
}
fn get_name_and_age(cat: (String, f64)) -> (String, f64) {
    let (name, age) = cat;
    (name, age)
}
