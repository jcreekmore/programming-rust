pub type Address = String;
pub struct Receipt;

pub struct Person {
    name: String,
    addr: Address,
}

impl Person {
    pub fn mail_package(&self) -> Receipt {
        Receipt
    }
}

pub struct PersonBuilder {
    name: String,
    addr: Option<Address>,
}

impl PersonBuilder {
    pub fn new(name: &str) -> Self {
        PersonBuilder {
            name: name.into(),
            addr: None,
        }
    }

    pub fn set_address(mut self, addr: Address) -> Self {
        self.addr = Some(addr);
        self
    }
    pub fn create(self) -> Option<Person> {
        match self.addr {
            None => None,
            Some(addr) => {
                Some(Person {
                    name: self.name,
                    addr: addr,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
