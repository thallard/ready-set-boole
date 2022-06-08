#[allow(dead_code)]
// Transform a natural number to Gray Code
pub fn gray_code(n: u32) -> u32 {
    return n ^ (n >> 1);
}

// Test section
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_code0() {
        assert!(gray_code(0) == 0);
    }

    #[test]
    fn gray_code1() {
        assert!(gray_code(1) == 1);
    }

    #[test]
    fn gray_code2() {
        assert!(gray_code(2) == 3);
    }

    #[test]
    fn gray_code3() {
        assert!(gray_code(3) == 2);
    }

    #[test]
    fn gray_code4() {
        assert!(gray_code(4) == 6);
    }

    #[test]
    fn gray_code5() {
        assert!(gray_code(5) == 7);
    }

    #[test]
    fn gray_code6() {
        assert!(gray_code(6) == 5);
    }

    #[test]
    fn gray_code7() {
        assert!(gray_code(7) == 4);
    }

    #[test]
    fn gray_code8() {
        assert!(gray_code(8) == 12);
    }
}