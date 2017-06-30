pub mod atm {
    pub struct Idle {
        cash: usize,
    }
    pub struct HasCard {
        cash: usize,
    }
    pub struct PINEntered {
        cash: usize,
    }
    pub struct SelectAmount {
        cash: usize,
    }

    pub fn new(amount: usize) -> Idle {
        Idle { cash: amount }
    }

    impl Idle {
        pub fn insert_card(self) -> HasCard {
            HasCard { cash: self.cash }
        }
    }

    impl HasCard {
        pub fn enter_pin(self) -> PINEntered {
            PINEntered { cash: self.cash }
        }
    }

    impl PINEntered {
        pub fn withdraw(self) -> SelectAmount {
            SelectAmount { cash: self.cash }
        }
    }

    impl SelectAmount {
        pub fn dispense_cash(self, amount: usize) -> Idle {
            Idle { cash: self.cash - amount }
        }
    }
}

enum State {
    Idle(atm::Idle),
    HasCard(atm::HasCard),
    PINEntered(atm::PINEntered),
    SelectAmount(atm::SelectAmount),
}

pub struct Foo {
    state: State,
}

#[derive(Debug)]
pub enum Msg {
    InsertCard,
    EnterPin,
    Withdraw,
    Dispense(usize),
}

pub fn get_msg() -> Msg {
    Msg::InsertCard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let atm = atm::new(100);
        let atm = atm.insert_card();
        let atm = atm.enter_pin();
        let atm = atm.withdraw();
        atm.dispense_cash(25);
    }

    #[test]
    fn check_dispense() {
        let atm = atm::new(100);
        // COMPILE ERROR
        // atm.dispense_cash(25);
    }

    #[test]
    fn test_dynamic() {
        let mut foo = Foo { state: State::Idle(atm::new(100)) };
        foo.state = match (get_msg(), foo.state) {
            (Msg::InsertCard, State::Idle(st)) => State::HasCard(st.insert_card()),
            (Msg::EnterPin, State::HasCard(st)) => State::PINEntered(st.enter_pin()),
            (Msg::Withdraw, State::PINEntered(st)) => State::SelectAmount(st.withdraw()),
            (Msg::Dispense(x), State::SelectAmount(st)) => State::Idle(st.dispense_cash(x)),
            (msg, _) => panic!("invalid state for {:?}", msg),
        };
    }
}
