pub fn check_pairs(sequence: &str) -> bool {
    let mut paren_stack = vec![];

    for char in sequence.chars() {
        match char {
            '(' => {
                paren_stack.push(char);
            }
            ')' => {
                match paren_stack.pop() {
                    Some(_) => continue,
                    None => return false
                }
            }
            _ => continue
        }
    }

    paren_stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::check_pairs;
    #[test]
    fn correct_paren() {
        let test_data = "()(())(()())";

        assert!(check_pairs(test_data));
    }

    #[test]
    fn wrong_paren() {
        let test_data = "()(())(()()";

        assert!(!check_pairs(test_data))
    }

    #[test]
    fn correct_paren_symbols() {
        let test_data = "(aa(aaaa))+ = 2 3()(ab(a)())";

        assert!(check_pairs(test_data));
    }

    #[test]
    fn wrong_paren_symbols() {
        let test_data = "(aa(1 - 2))bbba((ab(a)())";

        assert!(!check_pairs(test_data));
    }
}