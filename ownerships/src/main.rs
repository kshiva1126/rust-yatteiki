fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    let s2 = s.clone();
    println!("{}", s);
    println!("{}", s2);

    let a = String::from("hello");

    takes_ownership(a);

    let x = 5;

    makes_copy(x);

    makes_copy(x);

    let b1 = gives_ownership();

    let b2 = String::from("hello");

    let b3 = takes_and_gives_back(b2);

    println!("b1: '{}', b3: '{}'", b1, b3);

    let c1 = String::from("hello");

    let (c2, len) = calculate_length(c1);

    println!("The length of '{}' is {}.", c2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
