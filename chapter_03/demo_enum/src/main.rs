#[allow(dead_code)]

enum Bar {
    A { name: String },
    B(i32, i32),
    C
}

fn main() {
    let bar = Bar::B(3, 14);
    match bar {
        Bar::A{ name: val } => println!("A({})", val),
        Bar::B(v1, v2) => println!("B({}, {})", v1, v2),
        Bar::C => println!("C"),
    }
}
