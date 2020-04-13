//
// nato_spell: expect a reference to a unicode string, and returns
// a vector of strings corresponding to the respective NATO codes for each character.
// If the character does not have a match, it will return "(not represented)".
//

const LCASE_A: u8 = 97; // ASCII value for 'a'
const ONE: u8 = 48; // ASCII value for '0'
const NEWLINE: u8 = 10; //ASCII value for newline

pub fn nato_spell(source_text: &String) -> Vec<String> {
    // Array of tuples representing each letter with its official NATO call.
    let letter_symbol_table = 
                       [('a', "Alfa"), ('b', "Bravo"), ('c', "Charlie"), ('d', "Delta"),
                        ('e', "Echo"), ('f', "Foxtrot"), ('g', "Golf"), ('h', "Hotel"),
                        ('i', "India"), ('j', "Juliett"), ('k', "Kilo"), ('l', "Lima"),
                        ('m', "Mike"), ('n', "November"), ('o', "Oscar"), ('p', "Papa"),
                        ('q', "Quebec"), ('r', "Romeo"), ('s', "Sierra"), ('t', "Tango"),
                        ('u', "Uniform"), ('v', "Victor"), ('w', "Whiskey"), ('x', "X-Ray"),
                        ('y', "Yankee"), ('z', "Zulu")
                        ];

    // Numbers are spelled verbally without special representation.
    let number_symbol_table = 
                       [('0', "Zero"), ('1', "One"), ('2', "Two"), ('3', "Three"), ('4', "Four"),
                        ('5', "Five"), ('6', "Six"), ('7', "Seven"), ('8', "Eight"), ('9', "Niner")];

    //Building return vector
    let mut ret: Vec<String> = Vec::with_capacity(source_text.len());
    
    // Instead of sequentially looking for matches in a table, or match against each character,
    // we use the ASCII value minus its baseline as direct index into the array, which is much faster.
    // We only match for special cases (symbols).
    for c in source_text.to_ascii_lowercase().chars() {
        if c >= 'a' && c <= 'z' {
            let i: usize = (c as u8 - LCASE_A) as usize ;
            ret.push(letter_symbol_table[i].1.to_string());
        } else if c >= '0' && c <= '9' {
            let i: usize = (c as u8 - ONE) as usize ;
            ret.push(number_symbol_table[i].1.to_string());
        } else if (c as u8) != NEWLINE { 
            match c {
            '-' => ret.push("Dash".to_string()),
            '_' => ret.push("Underscore".to_string()),
            '$' => ret.push("Dollar".to_string()),
            '.' => ret.push("Dot".to_string()),
            '&' => ret.push("Ampersand".to_string()),
            '#' => ret.push("Pound".to_string()),
            '*' => ret.push("Asterisk".to_string()),
            '%' => ret.push("Percent".to_string()),
            '!' => ret.push("Exclamation".to_string()),
            ' ' => ret.push("Space".to_string()),
            _   => ret.push("(not represented)".to_string()),
            } 
        }
    }
    ret 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nato_spell() {
        let s = String::from("$&1Fa_ ");
        assert_eq!(nato_spell(&s), ["Dollar", "Ampersand", "One", "Foxtrot", "Alfa", "Underscore", "Space"]);
    }
}