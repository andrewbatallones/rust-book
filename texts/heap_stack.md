# Heap and Stack

## Stack

Think of it as a deck of cards. You can place cards on top, but can only take the top card. This is the same with primary variables like integers, bytes and characters. From there, they can also take the form of scopes, the scope will be on top of the stack, (Along with all primitive variable types) and when it goes out of scope, it removes the chunk of data (or cards).

## Heap

The heap is a little bit more chaotic, but that mess helps code dump data that it is unsure of. What helps me is imagine a hat with words written on sheets of paper in it that are tied to a string sticking out of the hat. You have access to the string and can pull out the sheet of paper that's tied to the string. From there, you can alter that paper (erase and write a new word, color it in, etc.), retie the paper and drop it back into the hat.

- Hat: Heap
- Paper: Data objects
- String: Pointers to the object.

### Questions?

# Rust

## Other languages

So, there are many languages that deal with how to manage data from the heap/stack. Lower level languages (C, C++) will allow the user to manually allowcate and clean the data. For high level languages (C#, Python, Ruby, etc), it is automatically cleaned via the garbage collector.

## How Rust solves this issue

The book says it's entirely new with ownership, however I still believe it's a different type of a "Hybrid" garbage collection. Because the way the Rust was built, it does a pretty good job of tracking down what data needs to be cleaned and so forth with the rules that Rust has set.

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.