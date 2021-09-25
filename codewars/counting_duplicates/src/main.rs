fn main() {
    println!("{}", count_duplicates("abcdeaB"));
}

fn count_duplicates(text: &str) -> u32 {
    let comp = text.to_lowercase();
    let mut duplicates = 0;
    let mut pos = 0;
    let mut verificated: String = String::with_capacity(comp.len());
    for letter in comp.chars() {
        pos += 1;
        match comp.get(pos..comp.len()) {
            None => break,
            Some(substr) => {
                if substr.contains(letter) && !verificated.contains(letter) {
                    duplicates += 1;
                }
                verificated.insert(verificated.len(), letter);
            }
        };
    }
    
    return duplicates;
}

/*
    Another solutions:

    use std::collections::HashMap;

    fn count_duplicates(text: &str) -> u32 {
        let mut char_count: HashMap<char, u32> = HashMap::new();
        for c in text.to_lowercase().chars() {
            let mut e = char_count.entry(c).or_default();
            *e += 1;
        }
        char_count.values().filter(|&&v| v > 1).count() as u32
    }

    ------------------------------------------------------

    use itertools::Itertools;

    fn count_duplicates(text: &str) -> u32 {
        text.to_lowercase().chars().counts().values().filter(|&&i| i > 1).count() as u32
    }
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }
    
    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_abcdeab() {
        assert_eq!(count_duplicates("abcdeaB"), 2);
    }
    
    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}