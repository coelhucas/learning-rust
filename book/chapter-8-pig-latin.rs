fn main() {
    println!("{}", to_pig_latin(&String::from("catatau")));
    println!("{}", to_pig_latin(&String::from("abracadabra")));
}

fn to_pig_latin(string: &String) -> String {
    let first_char = string.chars().next().unwrap();

    if is_vowel(first_char) {
        return format!("{}-hay", string);
    } else {
        let result = &string[1..string.len()];
        return format!("{}-{}ay", result, first_char);
    }
}

fn is_vowel(c: char) -> bool {
    String::from("aeiou").contains(c)
}
