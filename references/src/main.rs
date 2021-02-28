fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut a = String::from("hello");

    change(&mut a);

    println!("{}", a);

    change(&mut a);

    let r1 = &mut a;
    // 特定のスコープで借用は1度しかできない
    // let r2 = &mut a;
    println!("{}", &r1);

    {
        // 新しいスコープでは参照OK
        let r2 = &mut a;
        println!("{}", &r2);
    }

    let b = String::from("hello");

    let b1 = &b;
    // 不変な参照をしている間は、可変な参照をすることはできない
    // let b2 = &mut b;

    // 複数の不変参照は可能
    let b3 = &b;

    println!("{}, {}", b1, b3);
}

// 関数の引数に参照を取ることを借用と呼ぶ
// 変数が標準で不変なのと同様に、参照も不変
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
