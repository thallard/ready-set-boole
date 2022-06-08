use std::u32;

// Add two natural numbers
pub fn add(mut a: u32, mut b: u32) -> u32 {
    let mut carry : u32;
    
    while b != 0 {
        carry = a & b;
        a = a ^ b;
        b = carry << 1;
    }
    return a;
}


// Test section
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add0() {
        assert!(add(1, 1) == 2);
    }

    #[test]
    fn add1() {
        assert!(add(0, 0) == 0);
    }

    #[test]
    fn add2() {
        assert!(add(0, 45) == 45);
    }

    #[test]
    fn add3() {
        assert!(add(3, 2) == 5);
    }
    
    #[test]
    fn add4() {
        assert!(add(u32::MIN, 0) == u32::MIN)
    }

    #[test]
    fn add5() {
        assert!(add(u32::MAX, 0) == u32::MAX)
    }
}
