pub use crate::serde::from_str;
pub use crate::serde::to_string;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod packet;
mod parser;
mod serde;
