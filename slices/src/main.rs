fn main() {
    let s = String::from("hell word");
    let word = first_word(&s[..]);
    println!("{}", word);

    let my_string_literal = "hello rust";
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // 文字列リテラルはすでに文字列スライス*なのでスライス記法なしで機能する
    let word = first_word(my_string_literal);
    println!("{}", word);

    let a = String::from("hello world");

    let hello = &a[0..5];
    let world = &a[6..11];
    println!("{}, {}", hello, world);

    let hello1 = &a[..5];
    let world1 = &a[6..];
    println!("{}, {}", hello1, world1);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
