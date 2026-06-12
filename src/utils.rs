fn is_number(test_char: char) -> bool {
    let test_num = test_char as u8;
    test_num >= '0' as u8 && test_num <= '9' as u8
}

