struct Foo(i32, String);

fn main() {
    let foo = Foo(0, String::new());

    println!("0: {}, 1: {}", foo.0, foo.1);
}
