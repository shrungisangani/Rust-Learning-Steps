# Why Rust:

- Rust resolve memory save problem compare to c language.

# Rust Variables and Mutability using `mut` key word

    let mut x = 1;

# Rust Data Types:

- There are four primary data types in Rust also known as scalar types:

1. Integer
1. Floating-Point
1. Boolean
1. Character

# Rust Type Casting

- `as` keyword to perform type casting
- Type Conversion: Character to Integer in Rust
- Type Conversion: Integer to Character in Rust
- Type Casting: Boolean to Integer in Rust

# Rust Operators

1. Arithmetic Operators
1. Compound Assignment Operators
1. Logical Operators
1. Comparison Operators

   # Arithmetic Operators

   Operator Example

   ```
   + (Addition)	a + b
   - (Subtraction)	a - b
   * (Multiplication)	a * b
   / (Division)	a / b
   % (Remainder)	a % b
   ```

   # Compound Assignment Operators

   Operator Example

   ```
   += (addition assignment)
   -= (subtraction assignment)
   *= (multiplication assignment)
   /= (division assignment)
   %= (remainder assignment)
   ```

   # Logical Operators

   Operator Example

   ```
   && (Logical AND)
   || (Logical OR)
   ! (Logical NOT)
   ```

   # Comparison Operators

   Operator Example

   ```
   > (Greater than)
   < (Less than)
   >= (Greater than or equal to)
   <= (Less than or equal to)
   == (Equal to)
   != (Not equal to)
   ```

# Rust if...else

- Boolean Expression
- Rust if Expression
- Rust if..else Expressions
- Rust if..else if Expressions
- Example: if..else if..else Conditional
- Nested if..else

# Rust loop

- loop
- while
- for

# Rust Slice

- Omit Indexes of a Rust Slice
- Mutable Slice in Rust

# Rust Tuple

-In Rust, we can create a tuple in two different ways:

1. Tuple with data type
1. Tuple without data type

# Rust Struct

- Defining a Struct in Rust
  1.  Structure Struct
  1.  Destructure Struct

# Rust Functions

```
fn - keyword used to create a function in Rust
greet() - name of the function
// code - function body
{ } - start and end of the function body
```

# Rust Variable Scope

- Global scope
- function scope
- Reassign to function

# Rust Closure

```
print_text - variable to store the closure
|| - start of a closure
println!("Defining Closure") - body of the closure

```

# Rust Stack and Heap

- Adding data is called pushing onto the stack
- Removing data is called popping off the stack
- This phenomenon is called Last In, First Out (LIFO) in programming.

# Differences between Stack and Heap

| Stack                                                                         | Heap                                                                                               |
| ----------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Accessing data in the stack is faster.                                        | Accessing data in a heap is slower.                                                                |
| Managing memory in the stack is predictable and trivial.                      | Managing memory for the heap (arbitrary size) is non-trivial.                                      |
| Rust stack allocates by default.                                              | Box is used to allocate to the heap.                                                               |
| Primitive types and local variables of a function are allocated on the stack. | Data types that are dynamic in size, such as String, Vector, Box, etc., are allocated on the heap. |

# Rust Vector

Create a Vector in Rust
In Rust, we can create a vector using the vec! macro. For example,

let v = vec![1, 2, 3];
Here, we are creating a vector using the vec! macro with some initial values.

let v - the name of the variable
vec![1, 2, 3] - initialize a vector with integer values 1, 2, 3

- This concept is used in instead of array for memories allocation.

# Rust String

```
We can create a string with a default value using the String::from() method. For example,

// create a string with a default value
let word = String::from("Hello, World!");

```

# Rust HashMap

```
- Creating a HashMap in Rust
	1. HashMap is part of the Rust standard collections library, so we must include the HashMap module in our program to use it.

	use std::collections::HashMap;

```

# Rust HashSet

```
- HashSet implements the set data structure in Rust. Just like a set, it allows us to store values without duplicates.
	use std::collections::HashSet;

	Set Operations:
		1. Union of two Sets
		2. Intersection of two Sets
		3. Difference between two Sets
		4. Symmetric Difference between two Sets
```

# Rust Iterators
- An iterator in Rust is responsible for creating a sequence of values and allows us to iterate over each item of the sequence. It is primarily used for looping and we can only loop over iterators in Rust.

- Note: Collections like Array, Vector, HashMap and HashSet are not iterable by default. We can use the iter() method to tell Rust that it can be used to loop over the values.

```
Ways to Create Iterator in Rust
We can create an iterator by converting a collection into an iterator. There are three ways to create an iterator.

Using iter() method
Using into_iter() method
Using iter_mut() method
```

# Rust Error Handling

- In Rust, errors are of two categories:
1. Unrecoverable Errors
2. Recoverable Errors

# Rust unwrap() and expect()
- The unwrap() are expect() utility methods that work with Option and Result types in Rust.


# Rust Ownership
```
Rust includes an ownership mechanism to manage the memory of our program. Ownership is a set of rules that ensure memory safety in Rust programs.

The ownership feature in Rust allows our program to run without memory leaks and slowness.
```
- Data Move in Rust :
1. Sometimes, we might not want a variable to be dropped at the end of the scope. Instead, we want to transfer ownership of an item from one binding (variable) to another.

- Data Copy in Rust
1. Primitive types like Integers, Floats and Booleans don't follow the ownership rules. These types have a known size at compile time and are stored entirely on the stack, so copies of the actual values are quick to make.

# Rust References and Borrowing
```
References in Rust allow us to point to a resource (value) without owning it. This means that the original owner of the resource remains the same.

References are helpful when passing values to a function that we do not want to change the ownership of. Creating a reference is known as borrowing in Rust.

Rules of References::
  Rust primarily follows these rules of references at any given time:

  At any given time, you can have either one mutable reference or any number of immutable references.
  References must always be valid.
```
# Rust Module
```
Modules in Rust help in splitting a program into logical units for better readability and organization.

Once a program gets larger, it is important to split it into multiple files or namespaces. Modules help in structuring our program.

A module is a collection of items: functions, structs and even other modules.

Defining a Module in Rust::
The `mod` keyword is used to define a module
```
# Rust Crate and Package
```
A crate can contain one or more Rust modules, which in turn can contain code, such as functions, types, and constants.

A crate is of two types:

Binary crate
Library crate


- Creating a Package in Rust
  To create a binary package, we can use the `cargo` command in the terminal.

  $ cargo new hello_world --bin

-Creating a Library Package in Rust
  $ cargo new hello_world_lib --lib
```
# Rust Cargo
- Cargo is the Rust package manager. It comes pre-installed with Rust and can be used to package dependencies, manage them as well as build and distribute our own packages/libraries.

```
- Dependency Management with Cargo in Rust:
  1. Cargo.toml file in the root of our project directory hello_world is used to manage the dependencies.

  2. If we want to use the "rand" crate, we add the following line to the [dependencies] section of the Cargo.toml.

  [package]
  name = "hello_world"
  version = "0.1.0"
  edition = "2021"

  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

  [dependencies]
  rand = "0.8.5"

  3. Note: We can also add dependency to our project using the command cargo add rand.
```
# Useful Cargo Commands in Rust
Cargo can do a bunch of repetitive tasks for us. Here are some of the commonly used cargo commands.

|Command|	Description|
|--|--|
|cargo new	|Create a new Rust project with basic directory structure|
|cargo build|	Build (compile) the current project and generate a binary executable|
|cargo run	|Build and run your current project (cargo build + run)|
|cargo check	|Build the current project without generating a binary executable|
|cargo add	|Add a new dependency and include it in Cargo.toml file|
|cargo update	|Update all dependencies of current project to latest version|

# Rust Pattern Matching
- Pattern matching is a way to match the structure of a value and bind variables to its parts. It is a powerful way to handle data and control flow of a Rust program.

- We generally use the match expressions when it comes to pattern matching.

```
The most common case for pattern matching is with Option and Result enum types. Both the Option and Result type have two variants.

Option type has:

None → to indicate failure with no value
Some(T) → a value with type T

Result type has:

Ok(T) → operation succeeded with value T
Err(E) → operation failed with an error E
```

```
if let Expression in Rust
- The `if let` expression in Rust is a shorthand for a match expression with only one pattern/arm to match.
```

# Rust Generics
- Generics allows us to write code that is flexible and can be reused with different types of data, without having to write separate implementations for each type. It helps us write code that can handle values of any type in a type-safe and efficient way.

- With the help of generics, we can define placeholder types for our methods, functions, structs, enums and traits.

# Rust Trait

```
Implementing a Trait in Rust
To implement a trait, we use the impl keyword. 
Trait is use in multiple time of the function
```

# Rust File Handling
- In computer programming, file handling is a way to deal with data in a file. File handling enables us to open, read, write, create and update files on the local system.

- File handling is commonly performed by many applications including databases, web servers. It is an important example of how I/O (Input/Output) operations work.

- File handling is also generally known as File I/O.
```
File Struct in Rust:
Let's look at the basics of file I/O in Rust with these operations:

Opening a file
Reading from a file
Writing to a file
Removing a file
Appending to a file
```

# Rust Macro
A macro in Rust is a piece of code that generates another piece of code.

Macros generate code based on input, simplify repetitive patterns, and make code more concise.

Rust macro simply allows us to write code that writes more code which is also known as meta programming. Macros are used extensively in Rust.

Some of the popular Rust macros are println!, vec! and panic!.
  
  - Here, () => {} is the entry for a macro rule. We can have many rules to match for in a single macro.

  ```
  Note: There are many designators that we can use inside a macro rule body:

  stmt: a statement
  pat: a pattern
  expr: an expression
  ty: a type
  ident: an identifier
  ```

# Rust Thread
- A thread is the smallest executable unit of a process.

- Threads allow us to split the computation in our program into multiple threads. Running multiple tasks at the same time can improve performance of the code. However, it can add complexity.

```
Creating a New Thread in Rust
In Rust, we can create a native operating system thread using the thread::spawn() function from the std module. The spawn method takes a closure as an argument.

# Join Handles in Rust
  A spawned thread always returns a join handle. If we want the spawned thread to complete execution, we can save the return value of thread::spawn in a variable and then call the join() method on it.

  The join() method on JoinHandle (return type of thread::spawn) waits for the spawned thread to finish.


```