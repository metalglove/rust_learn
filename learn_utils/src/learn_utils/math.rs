/// Adds the given value `a` and `b` together.
///
/// # Examples
/// 
/// ```
/// let a: i8 = 5;
/// let b: i8 = 5;
/// let c: i8 = learn_utils::math::add(a, b);
/// 
/// assert_eq!(c, 10);
/// ```
pub fn add(a: i8, b: i8) -> i8 {
    a + b
}

/// Subtracts the given value `a` from `b`.
///
/// # Examples
/// 
/// ```
/// let a: i8 = 5;
/// let b: i8 = 5;
/// let c: i8 = learn_utils::math::subtract(a, b);
/// 
/// assert_eq!(c, 0);
/// ```
pub fn subtract(a: i8, b: i8) -> i8 {
    a - b
}