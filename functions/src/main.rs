fn main() {
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        // 式は終端にセミコロンを含まない
        // 式の終端にセミコロンをつけたら、文に変えてしまう
        x + 1
    };
    println!("The value of y is: {}", y);

    println!("The value of five() is: {}", five());

    println!("The value of plus_one(2) is: {}", plus_one(2));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
