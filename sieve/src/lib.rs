pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // Create a vector of booleans up the the upper bound + 1. All values are initially true
    let mut is_prime = vec![true; (upper_bound + 1) as usize];

    // Mark 0 as not prime
    is_prime[0] = false;

    // If the upper bound is at least 1, mark 1 not prime
    if upper_bound >= 1 {
        is_prime[1] = false;
    }

    // Initialize a new `current` number, starting at the first prime number, 2
    let mut current = 2;

    // Start a loop that continues as long as the square of the current number is less than or equal
    // to the upper bound
    while current * current <= upper_bound {
        // If the current number is prime...
        if is_prime[current as usize] {
            // Start marking its multiples as not prime

            // Initialize the first multiple to check. We start with the current number squared
            // because we already know all previous multiples have already been removed in previous
            // iterations of the loop
            let mut multiple = current * current;

            // While the multiple in in bounds...
            while multiple <= upper_bound {
                // Set the value that represents it in the vector to false, marking it not prime
                is_prime[multiple as usize] = false;

                // Go to the next multiple
                multiple += current;
            }
        }

        // Go to the next number in the vector
        current += 1;
    }

    // We are left with a vector full of booleans where true = prime and false = not prime
    // We need to turn this into a vector of numbers

    // Initialize a new vector
    let mut primes = Vec::new();

    // Iterate over all numbers from 2 to the upper bound
    for n in 2..=upper_bound {
        // If the boolean representation of this number in our is_primes vector is true...
        if is_prime[n as usize] {
            // Push the number into the newly created primes vector
            primes.push(n);
        }
    }

    // Return the vector of primes
    primes
}
