fn main() {
    let s = "first apple.".to_string();
    let words: Vec<&str> = s.split_whitespace().collect();
    for w in words.iter() {
        let current_word = w.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
        pig_latin(&current_word);
    }
}

fn pig_latin(w: &str) {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut pig_latin_word: String;

    if vowels.iter().any(|v| w.starts_with(*v)) {
        pig_latin_word = w.to_string();
        pig_latin_word.push_str("-hay");
    } else {
        pig_latin_word = w[1..].to_string();
        let p = format!("-{}ay", &w[..1]);
        pig_latin_word.push_str(&p);
    }
    println!("{}", pig_latin_word);
}
