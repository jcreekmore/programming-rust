struct Foo {
    a: i32,
    b: String
}

fn main() {
    let foo = Foo {
        a: 0,
        b: String::new()
    };

    println!("a: {}, b: {}", foo.a, foo.b);
}
