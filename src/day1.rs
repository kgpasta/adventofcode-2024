pub fn day1() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        // Since day1() just prints "Hello, world!", we can only verify it doesn't panic
        day1(); // Should execute without panicking
        assert!(true); // Dummy assertion since we can't easily test stdout
    }
}
