#[no_mangle]
pub extern fn the_answer() -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_answer() {
        let result = the_answer();
        assert_eq!(result, 42);
    }
}
