#[derive(Clone, Copy, Debug)]
pub enum State {
    Idle,
    HasCard,
    PINEntered,
    SelectAmount,
}

pub struct ATM {
    state: State,
    cash: usize,
}

impl ATM {
    pub fn new(amount: usize) -> ATM {
        ATM { state: State::Idle, cash: amount }
    }

    pub fn insert_card(&mut self) {
        match self.state {
            State::Idle => { self.state = State::HasCard; },
            _ => unreachable!(),
        }
    }

    pub fn enter_pin(&mut self) {
        match self.state {
            State::HasCard => { self.state = State::PINEntered; },
            _ => unreachable!(),
        }
    }

    pub fn withdraw(&mut self) {
        match self.state {
            State::PINEntered => { self.state = State::SelectAmount; },
            _ => unreachable!(),
        }
    }

    pub fn dispense_cash(&mut self, amount: usize) {
        match self.state {
            State::SelectAmount => { self.state = State::Idle; self.cash = amount; },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut atm = ATM::new(100);
        atm.insert_card();
        atm.enter_pin();
        atm.withdraw();
        atm.dispense_cash(25);
    }

    #[test]
    #[should_panic]
    fn check_dispense() {
        let mut atm = ATM::new(100);
        atm.dispense_cash(25);
    }
}
