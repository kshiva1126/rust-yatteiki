struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// タプル構造体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    println!("user1's email is: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // 構造体更新記法
        // 明示的にセットされていない残りのフィールドが、与えられたインsタンスのフィールドと同じ値になるように指定する
        ..user1
    };

    println!("user2's email is: {}", user2.email);

    // 構造体内のフィールドが同じ型であっても、それ自身が独自の型になる
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
