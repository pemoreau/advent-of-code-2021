mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let expected = 295;
        let result = part1(include_str!("../input_test.txt").to_string());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 1068781;
        let result = part2(include_str!("../input_test.txt").to_string());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(102, part1(include_str!("../input.txt").to_string()));
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(
            327300950120029,
            part2(include_str!("../input.txt").to_string())
        );
    }
}
