/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        // Convert the given sentence to lowercase
        .to_lowercase()
        // Split the sentence into an iterator over its characters
        .chars()
        // Filter so that only lowercase ASCII letters remain
        .filter(|c| c.is_ascii_lowercase())
        // Fold the iterator into an array
        // The initial array is 26 instances of `false` (one for each letter of the alphabet)
        .fold([false; 26], |mut acc, c| {
            // Convert the letter into its ASCII value (a = 97, b = 98, etc)
            // Subtract 97 from that. (a = 0, b = 1, etc)
            // Use that value as an index for the accumulator array and set the array element to
            // true
            acc[c as usize - 97] = true;

            // Return the accumulator array
            acc
        })
        // Convert the array of true/false values into an iterator
        .iter()
        // Check if all values are true. If so, all letters are used in the sentence. Therefor, it
        // is a pangram
        .all(|&b| b)
}
