/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // A string of every lowercase letter
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    // Get a lowercase version of the given sentence
    let lowercase_sentence = sentence.to_lowercase();

    // Start the count at zero
    let mut count = 0;

    // Iterate over all letters in the alphabet
    for letter in alphabet.chars() {
        // If the current letter is in the sentence...
        if lowercase_sentence.contains(letter) {
            // Add 1 to the count
            count += 1
        }
    }

    // If the count is 26 (the length of the alphabet), that means the sentence contains all letters
    // and is a pangram.
    count == alphabet.len()
}
