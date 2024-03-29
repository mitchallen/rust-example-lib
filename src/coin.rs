/// Returns a random boolean
///
/// This function will return a random boolean
///
/// ```
/// let result = rust_example_lib::coin::flip();
/// ```
pub fn flip() -> bool {
    rand::random()
}
