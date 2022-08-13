## Important concepts 

The stack and the heap are parts of memory available to the code to use at runtime.

Data that can store each one:

  - Data stored on the stack must have a known, fixed size.
  - Data with an unknown size at compile time or a size that might change must be stored on the heap.

How data is stored:

- The stack stores values in the order it gets them and removes the values in the opposite order (LIFO). Adding data is called *pushing onto the stack*, and removing data is called *popping off the stack*.
- The heap is less organized, when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called *allocating on the heap* and can be abbreviated as just *allocating* (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

Speed:

- Pushing to the stack is faster than allocating on the heap, as allocating on the heap requires more work.
- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Processors are faster if they jump around less in memory, they can do their job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap).

When the stack or the heap is being used:

- When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.


## References

<https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html>


