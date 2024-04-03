/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
/// ```rust
///     is_divisible_by(1, 2) // false
/// ```
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}


// docs are treated as markdown
fn main() {
   let a = is_divisible_by(1, 2);
   println!("{}", a);

   
}