pub trait TrigOps {
    fn sin(self) -> f64;
    fn cos(self) -> f64;
    fn tan(self) -> f64;
}

impl<T: Into<f64>> TrigOps for T {
    fn sin(self) -> f64 {
        self.into().sin()
    }

    fn cos(self) -> f64 {
        self.into().cos()
    }

    fn tan(self) -> f64 {
        self.into().tan()
    }
}

#[cfg(test)]
mod tests {
    use TrigOps;

    #[test]
    fn it_works() {
        println!("sin(1) = {}", 1.sin());
        println!("cos(2) = {}", 2.cos());
        println!("tan(3) = {}", 3.tan());
    }
}
