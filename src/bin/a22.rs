// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
  if n < lower {
      lower
  } else if n > upper {
      upper
  } else {
      n
  }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
  if b == 0 {
    return None
  }

  Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
  format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
  use crate::*;

  #[test]
  fn concat_string() {
    assert_eq!("sameday".to_owned(), concat("same", "day"), "string must come immediately after one another")
  }

  #[test]
  fn divide() {
      assert_eq!(Some(2), div(4, 2), "division seems to be having issues");
  }

  #[test]
  fn divide_zero() {
      assert_eq!(None, div(4, 0), "should return None since we don't divide by zero");
  }

  #[test]
  fn clamp_lower() {
    assert_eq!(100, clamp(10, 100, 1000), "should be 100")
  }

  #[test]
  fn clamp_upper() {
    assert_eq!(1000, clamp(5000, 100, 1000), "should be 1000")
  }
}