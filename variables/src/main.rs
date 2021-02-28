fn main() {
    // 再代入可能な変数にするには`mut`キーワードを`let`のあとに置く
    // ただし再代入できるのは同じ型の値のみ
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = b'a';
    println!("The value of x is: {}", x);

    // シャドーイング
    // 前に定義した変数と同じ名前の変数を新しく宣言できる
    // 違う型でもOK
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces length is: {}", spaces);

    // タプル
    let tup = (500, 6.4, 1);
    // 分配
    let (a, b, c) = tup;
    println!("The value of a, b, c are: {}, {}, {}", a, b, c);

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    // タプルの各要素には(.)で直接アクセス可能
    let five_hundred = tup2.0;
    println!("{}", five_hundred);

    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);

    another_function();
}

// 関数と変数の命名規則は、スネークケースを使うのが慣例
fn another_function() {
    println!("Another function.");
}
