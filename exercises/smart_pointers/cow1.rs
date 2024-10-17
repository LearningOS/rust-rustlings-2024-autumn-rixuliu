// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

// I AM DONE

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(vec) => {
                if *vec == vec![1, 0, 1] {
                    Ok(()) // Ensure values are modified correctly
                } else {
                    Err("Expected mutated values [1, 0, 1]")
                }
            }
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()), // Expecting a borrowed value since no mutation happened
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()), // Expecting an owned value because it was initially owned
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // If a mutation does occur, the values should be modified.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(ref vec) => {
                if vec == &[1, 0, 1] { // Check if values were mutated correctly
                    Ok(())
                } else {
                    Err("Values were not mutated correctly")
                }
            }
            _ => Err("Expected owned value"),
        }
    }
}
