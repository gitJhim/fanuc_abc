use std::{collections::HashMap};

pub fn alphabetHashMap() -> HashMap<char, String> {
    let mut alphabet = HashMap::<char, String>::new();
    let letters: [char; 27] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z', ' '];

    for i in 0..letters.len() {
        alphabet.insert(
            letters[i],
            format!("    1: J P[{}] 50% FINE    ;", i)
        );
    }
    return alphabet;
}

pub fn alphabetPositions() -> String {
    let mut positions = "";


    return positions.to_string();
}