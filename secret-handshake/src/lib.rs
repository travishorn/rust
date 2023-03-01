// 00001 = wink
// 00010 = double blink
// 00100 = close your eyes
// 01000 = jump

// 10000 = Reverse the order of the operations in the secret handshake.

pub fn actions(n: u8) -> Vec<&'static str> {
    // To convert n to a string, we could use format!("{}", n)
    // To convert n's value to binary, use {:b} instead of the plain {}
    // To pad the left side of the string with zeroes until its exactly 5 characters long, use
    // {:0>5b}
    let binary_string = format!("{:0>5b}", n);

    // Create a new vector called result
    let mut result = Vec::new();

    // If the 5th character in the binary string is 1, push "wink" to the result
    if binary_string.chars().nth(4) == Some('1') {
        result.push("wink");
    }

    // If the 4th character in the binary string is 1, push "double blink" to the result
    if binary_string.chars().nth(3) == Some('1') {
        result.push("double blink");
    }

    // If the 3th character in the binary string is 1, push "close your eyes" to the result
    if binary_string.chars().nth(2) == Some('1') {
        result.push("close your eyes");
    }

    // If the 2nd character in the binary string is 1, push "jump" to the result
    if binary_string.chars().nth(1) == Some('1') {
        result.push("jump");
    }

    // If the 1st character in the binary string is 1, reverse the result
    if binary_string.chars().nth(0) == Some('1') {
        result.reverse();
    }

    // Return the resulting vector
    result
}
