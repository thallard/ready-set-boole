use crate::ex00::adder::add;

// Multiply two natural numbers
pub fn mul(mut a: u32, mut b: u32) -> u32 {
    let mut result = 0 as u32;
  
    while b != 0 {
        if b & 1 == 1 {
            result = add(result, a);
        }
        a <<= 1;
        b >>= 1;
    }
    return result;
}

// Test section
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply0() {
        assert!(mul(1, 1) == 1);
    }

    #[test]
    fn multiply1() {
        assert!(mul(0, 0) == 0);
    }

    #[test]
    fn multiply2() {
        assert!(mul(0, 45) == 0);
    }

    #[test]
    fn multiply3() {
        assert!(mul(3, 2) == 6);
    }

    #[test]
    fn multiply4() {
        assert!(mul(u32::MIN, 0) == 0)
    }

    #[test]
    fn multiply5() {
        assert!(mul(u32::MAX, 0) == 0)
    }
}