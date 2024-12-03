mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let expected = 71;
        let result = part1(include_str!("../input_test.txt").to_string());
        assert_eq!(expected, result);
    }

    // #[test]
    // fn test_part2() {
    //     let expected = 208;
    //     let result = part2(include_str!("../input_test2.txt").to_string());
    //     assert_eq!(expected, result);
    // }

    #[test]
    fn test_input_part1() {
        assert_eq!(23054, part1(include_str!("../input.txt").to_string()));
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(
            51240700105297,
            part2(include_str!("../input.txt").to_string())
        );
    }
}
