/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let lowercase_sentence = sentence.to_lowercase();
    let mut count = 0;

    for letter in alphabet.chars() {
        if lowercase_sentence.contains(letter) {
            count += 1
        }
    }

    count == alphabet.len()
}
