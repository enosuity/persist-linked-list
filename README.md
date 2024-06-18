# persist-linked-list

This allows the list to be modified (e.g., by prepending elements) while preserving access to previous versions of the list, which is essential for persistent data structures.

# #Key Points:
Node Struct: Each node contains a value and a reference to the previous node using Rc<RefCell<Node<T>>> to manage shared ownership and enable interior mutability.

List Struct: Represents the linked list and contains an optional reference to the head node.

Methods:
new(): Creates a new, empty list.
prepend(): Adds a new element to the front, creating a new list without modifying the original.
iter(): Returns an iterator to traverse the list.


ListIterator Struct: Implements the Iterator trait to traverse the list from the head to the tail, allowing easy iteration over the list elements.

Example Usage: Demonstrates creating a list, prepending elements, and iterating over the list, showing that the original list remains unchanged after modifications.