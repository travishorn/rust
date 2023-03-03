/// Get the multiples of a given number up to a given upper bound
fn get_multiples(n: u64, upper_bound: u64) -> Vec<u64> {
    // Initialize a new vector
    let mut result = Vec::new();

    // Start with the given number multiplied by 2.
    // We dont start with the number itself because we don't want to consider
    // the number itself a multiple for the purposes of this exercise
    let mut current = n * 2;

    // Start a loop that continues as long as the current number is less than or equal to the given
    // upper bound
    while current <= upper_bound {
        // Push the current number into the vector
        result.push(current);

        // Add the given number to the current number to get the next multiple
        current += n;
    }

    // Return the resulting vector which contains all multiples
    result
}

/// Get the primes up to a given upper bound
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // Store a vector of all numbers from 2 up to the upper bound
    let mut ns: Vec<u64> = (2..=upper_bound).collect();

    // Initialize an index variable to keep track of where we are in the vector
    let mut i = 0;

    // Keep looping as long as we haven't reached the end of the vector
    // We cannot use `for n in ns` here because we're modifying ns
    // We cannot use `for i in 0..ns.len()` because we're taking elements out of ns, making it
    // shorter. Eventually i will be an index out-of-bounds of the shortened ns vector
    while i < ns.len() {
        // Store the value at the index
        let n = ns[i];

        // Get the multiples of that number (except itself)
        let multiples = get_multiples(n, upper_bound);

        // Modify ns, keeping only the values that do not exist in the multiples vector
        // Effectively, removing all multiples
        ns.retain(|x| !multiples.contains(x));

        // Increment the index for the next iteration
        i += 1;
    }

    // Return the resulting vector, which has all numbers that are multiples of other numbers
    // removed. Meaning it only contains prime numbers.
    ns
}
