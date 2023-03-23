//! This is my module documentation. My library is so nice!

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//adding one more function
/// four() is a function that returns `4`
///
/// ```
/// use mylib::four;
/// let x = four();
/// assert_eq!(four(), 4);
/// ```
pub fn four() -> i32 { 4 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    //adding the test for the new function
    #[test]
    fn the_new_function_works() {
        assert_eq!(four(), 4);
    }
}
