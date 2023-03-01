use std::iter::FromIterator;

// This struct defines a node that will appear in linked lists. It has a value and a pointer to the
// next node in the list
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// The exercise instructions give this struct as a starting point. It uses a generic type parameter
// and has a single field `head` which points to a Node
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    // When a new list is created, the head should point to `None`
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        // Instead of iterating through the list, counting the elements, and returning true if the
        // count is 0, we can just check to see if there is a value in the head
        self.head.is_none()
    }

    // Return the length of the linked list
    // Take a shared reference to self and return the size as a `usize`
    pub fn len(&self) -> usize {
        // Initialize a mutable reference to `self.head` and a counter starting at 0
        let mut current_node = &self.head;
        let mut count = 0;

        // Iterate over each node, using a `while let` pattern matching expression. Some(node)
        // will be matched as long as the node's value is not None.
        while let Some(node) = current_node {
            // Update current_node to be the next node in the list and increment the count
            current_node = &node.next;
            count += 1;
        }

        // When the current node's value is None, we have reached the end of the list, the
        // Some(node) pattern no longer matches, and we exit the while loop. `count` now contains
        // the number of nodes in the list
        count
    }

    // Add an element to the beginning of the list
    pub fn push(&mut self, element: T) {
        // Create a new Box pointer to the new Node struct
        let new_node = Box::new(Node {
            // The value is the given element
            value: element,

            // next is the current head node. `take()` moves the Option value out of the head
            next: self.head.take(),
        });

        // Update the head to point to the new node. Must use Some() here because the new_node is a
        // Box pointer
        self.head = Some(new_node);
    }

    // Remove the head element in the list
    pub fn pop(&mut self) -> Option<T> {
        // Use take() to move the value out of the Option, then use Some() to determine if there is
        // anything in the value.
        if let Some(head) = self.head.take() {
            // If there is some value, update the head to the next value in the list, effectively
            // removing the current head from the list
            self.head = head.next;

            // Return the value that was stored in the head when we used `take()` to take it earlier
            return Some(head.value);
        }

        // If there is no value, the list is empty. Return None
        None
    }

    // Get a reference to the head element, but don't remove it
    pub fn peek(&self) -> Option<&T> {
        // as_ref gets a reference to the Box<Node<T>> inside the Option without taking ownership of
        // the value. map transforms the Option<&Box<Node<T>>> into Option<&T> by mapping the
        // reference inside Box<Node<T>> using a closure
        self.head.as_ref().map(|node| &node.value)
    }

    // Get a new reversed list without modifying the original list
    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        // Create a new linked list
        let mut new_list = SimpleLinkedList::new();

        // Create a new mutable variable initialized to the head of the original list
        let mut cur_node = self.head;

        // Start a while loop, checking to see whether there is a value in cur_node
        while let Some(node) = cur_node {
            // Push the value of the current node into the new list
            new_list.push(node.value);

            // Update the current node to be the next node before the next iteration of the loop
            cur_node = node.next;
        }

        // When the loop is finished, all values from the original list will be pushed into new_list
        // They are read from last-in and pushed so that the last value read is at the head,
        // effectively reversing the original list. Return the new (reversed) list.
        new_list
    }
}

// Create a new SimpleLinkedList from an iterator
// Define a new implementation of the FromIterator trait
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    // Take an iterator over value of type `T` and return a new SimpleLinkedList<T> containing those
    // values
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // Create a new list using the `new` method
        let mut list = SimpleLinkedList::new();

        // Loop over the values in the iterator
        for item in iter {
            // Push the value into the list
            list.push(item);
        }

        // Return the created list
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

// From<SimpleLinkedList<T>> trait for Vec<T> that allows you to create a Vec<T> from a
// <StimpleLinkedList<T>>
impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    // Use the mut keyword to take ownership over the list so we can mutate it using pop()
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        // Create a new empty Vec
        let mut vec = Vec::new();

        // Start a loop, popping values from the list and using a pattern matching expression Some()
        // to make sure there is a value there (we haven't reached the end of the list)
        while let Some(node) = linked_list.pop() {
            // Push the value that was popped from the list into the Vec
            vec.push(node);
        }

        // Reverse the Vec because we added the elements in reverse order they appeared in th e list
        vec.reverse();

        // Return the Vec
        vec
    }
}
