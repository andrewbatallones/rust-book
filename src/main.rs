fn main() {
    let dyn_string: String = String::from("hi");

    rust_book::ownership::take_ownership(dyn_string);

    // println!("{}", dyn_string);
}
