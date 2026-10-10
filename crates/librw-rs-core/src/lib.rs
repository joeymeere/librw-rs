//! librw-rs-core

pub fn hello() -> &'static str {
    "Hello from librw-rs-core!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello from librw-rs-core!");
    }
}
