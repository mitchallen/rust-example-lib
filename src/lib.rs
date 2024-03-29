pub mod coin;

/// Add two numbers together
///
/// This function takes two numbers and adds them together
/// and returns the result.
///
/// ```
/// let result = rust_example_lib::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_flip() {
        let _result = coin::flip();
    }
}
