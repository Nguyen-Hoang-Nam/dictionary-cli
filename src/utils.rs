pub fn get_bit_at(input: u8, n: u8) -> bool {
    input & (1 << n) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit_at_1() {
        assert_eq!(true, get_bit_at(1, 0));
    }

    #[test]
    fn test_get_bit_at_2() {
        assert_eq!(false, get_bit_at(1, 1));
    }
}
