trait GBKLength {
    const LENGTH: usize;
}

struct VinLength {}
impl GBKLength for VinLength {
    const LENGTH: usize = 17;
}

struct AccidLength {}
impl GBKLength for AccidLength {
    const LENGTH: usize = 20;
}

trait GBKString {
    const LENGTH: usize;
    fn str(&self) -> &str;
    fn mut_str(&mut self) -> &mut str;
}

struct Vin {
    message: String,
}

impl GBKString for Vin {
    const LENGTH: usize = 17;
    fn str(&self) -> &str {
        self.message.as_str()
    }
    fn mut_str(&mut self) -> &mut str {
        self.message.as_mut_str()
    }
}
