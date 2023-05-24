pub fn take_ownership(s: String) {
    println!("{}", s);
}

pub fn take_and_return_ownership(s: String) -> String {
    println!("{}", s);

    s
}