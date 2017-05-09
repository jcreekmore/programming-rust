use std::collections::HashMap;

pub struct Value {
    name: String,
    id: isize
}

trait PrintValue {
    fn print(&self);
}

impl PrintValue for Value {
    fn print(&self) {
        print!("({}[{}])", self.name, self.id);
    }
}

impl PrintValue for [Value] {
    fn print(&self) {
        let mut first = true;
        print!("[");
        for val in self {
            if !first {
                print!("\n ");
            }
            val.print();
            first = false;
        }
        print!("]");
    }
}

impl PrintValue for HashMap<String, Value> {
    fn print(&self) {
        let mut first = true;
        print!("{{");
        for (key, val) in self {
            if !first {
                print!("\n ");
            }
            print!("{}: ", key);
            val.print();
            first = false;
        }
        print!("}}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_vec() {
        let v = vec![
            Value { name: "Apple".into(), id: 1234 },
            Value { name: "Cherry".into(), id: 5678 },
            Value { name: "Banana".into(), id: 90 },
        ];
        v.print();
        println!("");
    }

    #[test]
    fn test_map() {
        let mut m = HashMap::new();
        m.insert("A".into(), Value { name: "Apple".into(), id: 1234 });
        m.insert("B".into(), Value { name: "Cherry".into(), id: 5678 });
        m.insert("C".into(), Value { name: "Banana".into(), id: 90 });
        m.print();
        println!("");
    }
}
