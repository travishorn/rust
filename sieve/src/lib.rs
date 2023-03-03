pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // Create a vector of booleans up the the upper bound + 1. All values are initially true,
    // meaning that we initiallize assume all numbers ARE prime unless marked otherwise
    let mut is_prime = vec![true; (upper_bound + 1) as usize];

    // Mark 0 and 1 as not prime
    is_prime[0] = false;
    is_prime[1] = false;

    // Initialize a new `current` number, starting at the first prime number, 2
    let mut current = 2;

    // Start a loop that continues as long as the square of the current number is less than or equal
    // to the upper bound
    while current * current <= upper_bound {
        // If the current number has not yet been marked as "not prime"...
        if is_prime[current as usize] {
            // Start marking its multiples as not prime. Any number that is a multiple of another
            // number is not prime

            // Initialize the first multiple to check. We start with the current number squared
            // because we already know all previous multiples would have already been removed in
            // previous iterations of the loop
            let mut multiple = current * current;

            // While the multiple is in bounds...
            while multiple <= upper_bound {
                // Set the value that represents it in the vector to false, marking it "not prime"
                is_prime[multiple as usize] = false;

                // Go to the next multiple and continue checking
                multiple += current;
            }
        }

        // Go to the next number in the vector to mark multiples of *it* as "not prime"
        current += 1;
    }

    // We are left with a vector full of booleans where true = prime and false = not prime
    // We can create a vector of prime numbers using this information

    // Initialize a new vector
    let mut primes = Vec::new();

    // Iterate over all numbers from 2 to the upper bound
    for n in 2..=upper_bound {
        // If the boolean representation of this number in our is_primes vector is true...
        if is_prime[n as usize] {
            // The number is prime. Push it into our `primes` vector
            primes.push(n);
        }
    }

    // Return the vector of primes
    primes
}
