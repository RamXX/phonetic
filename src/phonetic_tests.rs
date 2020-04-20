// Unit testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nato_spell() {
        let s = String::from("/$&1Fa_ \\á");
        assert_eq!(nato_spell(&s), ["Slash", "Dollar", "Ampersand", "One", "Foxtrot", "Alfa", 
                                    "Underscore", "Space", "Backslash", "(not implemented)"]);
    }

    #[test]
    fn test_morse() {
        let s = String::from("18Jk0-F");
        assert_eq!(morse(&s), [".----", "---..", ".---", "-.-", "-----", "(not implemented)", 
                                    "..-."]);
    }
}