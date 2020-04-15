

const LCASE_A: u8 = 97; // ASCII value for 'a'
const ONE: u8 = 48; // ASCII value for '0'

const LETTER_SYMBOL_TABLE: [(char, &str, &str); 26] = 
                       [('a', "Alfa", ".-"), ('b', "Bravo", "-..."), ('c', "Charlie", "-.-."), ('d', "Delta", "-.."),
                        ('e', "Echo", "."), ('f', "Foxtrot", "..-."), ('g', "Golf", "--."), ('h', "Hotel", "...."),
                        ('i', "India", ".."), ('j', "Juliett", ".---"), ('k', "Kilo", "-.-"), ('l', "Lima", ".-.."),
                        ('m', "Mike", "--"), ('n', "November", "-."), ('o', "Oscar", "---"), ('p', "Papa", ".--."),
                        ('q', "Quebec", "--.-"), ('r', "Romeo", ".-."), ('s', "Sierra", "..."), ('t', "Tango", "-"),
                        ('u', "Uniform", "..-"), ('v', "Victor", "...-"), ('w', "Whiskey", ".--"), ('x', "X-Ray", "-..-"),
                        ('y', "Yankee", "-.--"), ('z', "Zulu", "--..")
                        ];

// Numbers are spelled verbally without special representation.
const NUMBER_SYMBOL_TABLE: [(char, &str, &str); 10] = 
                       [('0', "Zero", "-----"), ('1', "One", ".----"), ('2', "Two", "..---"), ('3', "Three", "...--"), 
                        ('4', "Four", "....-"), ('5', "Five", "....."), ('6', "Six", "-...."), ('7', "Seven", "--..."), 
                        ('8', "Eight", "---.."), ('9', "Niner", "----.")];

const SYMBOL_TABLE: [(char, &str); 12] = 
                        [('-', "Dash"), ('_', "Underscore"), ('$', "Dollar"), ('.', "Dot"), 
                         ('&', "Ampersand"), ('#', "Pound"), ('*', "Asterisk"), ('%', "Percent"),
                         ('!', "Exclamation"), (' ', "Space"), ('/', "Slash"), ('\\', "Backslash")
                        ];

//
// nato_spell: expects a reference to a unicode string, and returns
// a vector of strings corresponding to the respective NATO codes for each character.
// If the character does not have a match, it will return "(not represented)".
// A few symbols are supported for convenience as standard English spelling. These are NOT
// officially part of the NATO alphabet.
//
pub fn nato_spell(source_text: &String) -> Vec<String> {
    //Building return vector
    let mut ret: Vec<String> = Vec::with_capacity(source_text.len());
    
    // Instead of sequentially looking for matches in a table, or match against each character,
    // we use the ASCII value minus its baseline as direct index into the array, which is much faster.
    // We only loop match for special cases (symbols).
    // Anything that is not an ASCII letter, a number or a supported symbol is returned as "(not implemented)".

    for c in source_text.to_ascii_lowercase().chars() {
        if c >= 'a' && c <= 'z' {
            let i: usize = (c as u8 - LCASE_A) as usize ;
            ret.push(LETTER_SYMBOL_TABLE[i].1.to_string());
        } else if c >= '0' && c <= '9' {
            let i: usize = (c as u8 - ONE) as usize ;
            ret.push(NUMBER_SYMBOL_TABLE[i].1.to_string());
        } else {
            let mut found_in_symbols = false;
            for (d, s) in &SYMBOL_TABLE {
                if &c == d {
                    ret.push(s.to_string());
                    found_in_symbols = true;
                    break;
                }
            }
            if ! found_in_symbols {
                ret.push("(not implemented)".to_string());
            }
        }
    }
    ret 
}

// morse() will return a vector of Strings with the morse representation for each
// input character. It leverages the same tables already present for the NATO spelling function.

pub fn morse(source_text: &String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::with_capacity(source_text.len());

    for c in source_text.to_ascii_lowercase().chars() {
        if c >= 'a' && c <= 'z' {
            let i: usize = (c as u8 - LCASE_A) as usize ;
            ret.push(LETTER_SYMBOL_TABLE[i].2.to_string());
        } else if c >= '0' && c <= '9' {
            let i: usize = (c as u8 - ONE) as usize ;
            ret.push(NUMBER_SYMBOL_TABLE[i].2.to_string());
        } else {
                ret.push("(not implemented)".to_string());
            }
    }
    ret 
}

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