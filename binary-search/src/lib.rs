/// A binary search implementation
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // Since this implementation uses recursion and it passes along a slice of the original array
    // each iteration, we need to keep track of where the slice is in relation to the original
    // array. We use the `start` argument, starting at 0.
    find_helper(array, key, 0)
}

fn find_helper(array: &[i32], key: i32, start: usize) -> Option<usize> {
    // If the array is empty, there's nothing left to search, so no matching value was found and we
    // return None
    if array.len() == 0 {
        return None
    }

    // Initialize a reference to the middle index of the array. If the array length is even it will
    // be the index exactly to the left of the middle
    let middle = array.len() / 2;
    
    // If the element at the middle index matches the search key, the key has been found. Return the
    // index of the key, offset by the start position. If this is the first iteration in the
    // recursion, start will be 0. But the key was higher than the middle value of the last
    // iteration, we're searching the right half of the array, so we need to offset the index by
    // wherever we started
    if array[middle] == key {
        return Some(start + middle);
    }

    // If the key is higher than the middle value...
    if array[middle] < key {
        // Recursively call the function again, but this time only with the right half of the array.
        // To keep track of where this next iteration started in relation to the original array,
        // pass the current start position + the middle of the array + 1
        return find_helper(&array[(middle + 1)..], key, start + middle + 1);
    }

    // If the key is lower than the middle value...
    if array[middle] > key {
        // Recursively call the function again, but this time only with the left half of the array.
        return find_helper(&array[..middle], key, start);
    }

    // If none of those conditions match, there is something else wrong with the way `find` was
    // called. No match could be found. Return None.
    None
}
