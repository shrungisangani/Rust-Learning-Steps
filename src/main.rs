// use core::slice;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
// use std::fs::File;
// use std::num::ParseIntError;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    //declare integer
    let x: u32 = 300;
    println!("Integer = {}", x);

    // mut variable can re-assign
    //f32 meanse float
    let mut p: f32 = 3.14;
    println!("Float: {}", p);

    // change value of PI
    p = 535.23;
    println!("Update Value of PI: {}", p);

    //declare decimal
    let decimal: f32 = 64.31;
    println!("decimal = {}", decimal);

    // boolean value true
    let boolean: bool = true;
    println!("boolean = {}", boolean);

    // char type
    let character: char = 'z';
    println!("character = {}", character);

    //Rust Type Casting
    // assign a floating point f64 value to decimal variable
    let decimal: f32 = 64.31;

    // convert decimal variable to u16 integer type using as keyword
    let integer = decimal as u16;

    println!("type cast decimal = {}", decimal);
    println!("type cast integer = {}", integer);

    //Type Conversion: Character to Integer in Rust and Integer to character
    let characterval: char = 'A';
    // convert char type to u8 integer type
    let integerval = characterval as u8;
    println!("type conversion character = {}", characterval);
    println!("type conversion integer = {}", integerval);

    //OPERATORS
    // add two variables using + operator
    let a: i32 = 20;
    let b = 2;

    let x = a + b;
    println!("{} + {} = {}", a, b, x);

    // subtract two variables using - operator
    let y = a - b;
    println!("{} - {} = {}", a, b, y);

    // multiply two variables using * operator
    let z = a * b;
    println!("{} * {} = {}", a, b, z);

    //Example: Comparison Operators
    let a = 7;
    let b = 3;

    // use of comparison operators
    let c = a > b;
    let d = a < b;
    let e = a == b;

    println!("{} >= {} is {}", a, b, c);
    println!("{} <= {} is {}", a, b, d);
    println!("{} == {} is {}", a, b, e);

    //Logical Operators
    let a = true;
    let b = false;

    // logical AND operation
    let c = a && b;

    // logical OR operation
    let d = a || b;

    // logical NOT operation
    let e = !a;

    println!("{} && {} = {}", a, b, c);
    println!("{} || {} = {}", a, b, d);
    println!("!{} = {}", a, e);

    let number: i8 = 10;

    // condition to check if number is greater than zero
    if number > 0 {
        println!("{} is greater than 0", number);
    }

    //if else if
    if number > 0 {
        println!("{} is positive", number);
    } else if number < 0 {
        println!("{} is negative", number);
    } else {
        println!("{} is equal to 0", number);
    }

    //loop
    // loop {
    //     println!("Loop forever!");
    // }

    // usage of while loop
    // let mut counter:i8 = 1;
    // while counter < 6 {
    //     println!("{}", counter);
    //     counter += 1;
    // }

    //for loop in rust
    for i in 1..6 {
        println!("{}", i);
    }

    // array of natural numbers
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..3];
    println!("Array of numbers = {:?}", slice);

    //Rust Slice
    // an array of numbers
    let numbers = [1, 2, 3, 4, 5];

    // create a slice of 2nd and 3rd element
    let slice = &numbers[1..3];

    println!("Rust Slice array = {:?}", numbers);
    println!("Rust Slice slice = {:?}", slice);

    // omit the start index
    let slice = &numbers[..3];
    println!("slice = {:?}", slice);

    //Rust Tuple
    let tuple = ("Hello", 3.14, 5);
    println!("Tuple Without data type contents = {:?}", tuple);

    let tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple With data type contents = {:?}", tuple);

    //how to get tuple index values
    let random_tuple = ("Hello", 200, 3.14);
    // accessing tuple element at index 0
    println!("Tuple Value at Index 0 = {}", random_tuple.0);

    //Rust Struct
    // define a Person struct
    struct Person {
        name: String,
        age: u8,
        height: u8,
    }

    // instantiate Person struct
    let person = Person {
        name: String::from("John Doe"),
        age: 18,
        height: 178,
    };

    // access value of name field in Person struct
    println!("Struct Person name = {}", person.name);
    println!("Struct Person age = {}", person.age);
    println!("Struct Person height = {}", person.height);

    //--------------Destructure struct----------
    // define a Person struct
    struct PersonDes {
        name: String,
        age: u8,
        height: u8,
    }

    // instantiate Person struct
    let person = PersonDes {
        name: String::from("John Doe"),
        age: 18,
        height: 178,
    };

    // destructure Person struct into name, age and height variables
    let PersonDes { name, age, height } = person;

    println!("PersonDes name = {}", name);
    println!("PersonDes age = {}", age);
    println!("PersonDes height = {}", height);

    //-----------------------------------------------------

    //Rust Functions
    // define a function using `fn`
    fn greet() {
        println!("Rust first function calling!");
    }
    greet();

    //---- function with param
    // function with parameters
    fn add(a: i32, b: i32) {
        let sum = a + b;

        println!("Sum of a and b = {}", sum);
    }

    add(2, 11);

    //-----------------------------------------------------
    //Rust Variable Scope
    let random = "first";

    // start of the inner block
    {
        println!(
            "random variable before shadowing in inner block = {}",
            random
        );

        // this declaration shadows the outer random variable
        let random = "abc";

        println!("random after shadowing in inner block = {}", random);
    }
    // end of the inner block

    println!("random variable in outer block = {}", random);

    //---------------------------------------
    //Rust Closure

    // closure that prints a text
    let print_text = || println!("Closure run!!!!!");

    print_text();

    //multi line closure
    // define a multi-line closure
    let squared_sum = |x: i32, y: i32| {
        // find the sum of two parameters
        let sum: i32 = x + y;

        // find the squared value of the sum
        let result: i32 = sum * sum;

        return result;
    };

    // call the closure
    let result = squared_sum(5, 3);

    println!("Result = {}", result);

    //-------------------------------

    let word: &str = "Hello";

    // immutable closure
    let print_str = || {
        // word variable is moved to a new variable
        let new_word = word;
        println!("new moved word using closure = {}", new_word);
    };

    print_str();

    //---------- The heap
    let x = Box::new(100);
    let y = 222;

    println!("x = {}, y = {}", x, y);

    //---------------------------
    //Rust Vector
    //Example: Creating a Vector in Rust
    // vector creation with vec! macro
    let v = vec![1, 2, 3];

    println!("v2= {:?}", v);

    //----------------------

    let colors = vec!["blue", "red", "green"];
    // method 1: access vector elements using vector index
    println!("first color = {}", colors[0]);
    println!("second color = {}", colors[1]);
    println!("third color = {}", colors[2]);

    //-- Adding Values to a Vector in Rust
    // mutable vector
    let mut even_numbers = vec![2, 4, 6, 8, 10];

    println!("original vector = {:?}", even_numbers);

    // push values at the end of the vector
    even_numbers.push(12);
    even_numbers.push(14);

    println!("changed vector = {:?}", even_numbers);

    //----Removing Values from a Vector in Rust
    let mut even_numbers = vec![2, 4, 6, 8, 10];

    println!("original vector = {:?}", even_numbers);

    // remove value from the vector in its second index
    even_numbers.remove(2);

    println!("removed changed vector = {:?}", even_numbers);

    //---Looping Through a Vector in Rust
    let colors = vec!["blue", "red", "green"];

    // loop through a vector to print its index and value
    for index in 0..3 {
        println!(
            "Looping Through a Vector in Rust Index: {} -- Value: {}",
            index, colors[index]
        );
    }

    //--- Mutable String in Rust

    let mut word = String::from("cat");

    println!("Mutable original string = {}", word);

    // push a new string at the end of the initial string
    word.push_str(" dog");

    println!("Mutable changed string = {}", word);

    //-------Iterating over Strings
    let str = String::from("Hello");

    // Loop through each character in a string using chars() method
    for char in str.chars() {
        println!("{}", char);
    }

    //-----HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // insert elements to hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    println!("HashMap = {:?}", fruits);

    //--- Access Values in a HashMap in Rust
    let mut fruits: HashMap<i32, String> = HashMap::new();

    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));

    let first_fruit = fruits.get(&1);
    println!("Access Values in a HashMap in Rust = {:?}", first_fruit);

    //---- Remove HashMap in rust
    let mut fruits: HashMap<i32, String> = HashMap::new();

    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));

    fruits.remove(&1);
    println!("Remove Values in a HashMap in Rust = {:?}", fruits);

    //--- Change HashMap in rust
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // insert values in the hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));

    // update the value of the element with key 1
    fruits.insert(1, String::from("Mango"));
    println!("Change Values in a HashMap in Rust = {:?}", fruits);

    //------- HashSet In Rust

    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements to hashset
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    println!("HashSet insert in Rust = {:?}", colors);
    //remove values from the hash set
    colors.remove("Yellow");
    println!("HashSet remove in Rust = {:?}", colors);

    // Create HashSet with default set of values using from() method
    let numbers = HashSet::from([2, 7, 8, 10]);

    println!("HashSet with default values numbers = {:?}", numbers);

    //-----Set Operations
    //1. Union of two Sets
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);

    // Union of hashsets
    let result: HashSet<_> = hashset1.union(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("union = {:?}", result);
    //2. Intersection of two Sets

    // Intersection of hashsets
    let result: HashSet<_> = hashset1.intersection(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("intersection = {:?}", result);

    //3. Difference between two Sets
    let hashset1 = HashSet::from([1, 2, 3, 4]);
    let hashset2 = HashSet::from([4, 3, 2]);

    // Difference between hashsets
    let result: HashSet<_> = hashset1.difference(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("difference = {:?}", result);

    //4. Symmetric Difference between two Sets
    let hashset1 = HashSet::from([2, 7, 8, 10, 12]);
    let hashset2 = HashSet::from([1, 2, 7, 9]);

    // Symmetric difference of hashsets
    let result: HashSet<_> = hashset1.symmetric_difference(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("symmetric difference = {:?}", result);

    //----Example: Iterator in Rust
    let numbers = vec![2, 1, 17, 99, 34, 56];

    // iterator
    let numbers_iterator = numbers.iter();

    for number in numbers_iterator {
        println!("{}", number);
    }

    //---- Rust Error Handling

    // let data_result = File::open("data.txt");

    // // using match for Result type
    // let data_file = match data_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the data file: {:?}", error),
    // };

    // println!("Rust Error Handling Data file{:?}", data_file);

    //---- Example: Using unwrap()
    // function to find a user by their username which return an Option enum
    // fn get_user(username: &str) -> Option<&str> {
    //     if username.is_empty() {
    //         return None;
    //     }

    //     return Some(username);
    // }

    // // use of unwrap method to get the result of Option enum from get_user function
    // let result = get_user("Hari").unwrap();

    // // print the result
    // println!("user = {:?}", result);

    //--- The expect() Method
    // Function to parse an integer
    // fn parse_int() -> Result<i32, ParseIntError> {
    //     // Example of ? where value is unwrapped
    //     let x: i32 = "12".parse()?; // x = 12

    //     // Example of ? where error is returned
    //     let y: i32 = "12a".parse()?; // returns an Err() immediately

    //     Ok(x + y) // Doesn't reach this line
    // }
    // let res = parse_int();

    // println!("{:?}", res);

    //-------------------Data Move in Rust
    // owner of the String value
    // rule no. 1
    let fruit1 = String::from("Banana");

    // ownership moves to another variable
    // only one owner at a time
    // rule no. 2
    let fruit2 = fruit1;

    // cannot print variable fruit1 because ownership has moved
    // error, out of scope, value is dropped
    // rule no. 3
    // println!("fruit1 = {}", fruit1);

    // print value of fruit2 on the screen
    // println!("fruit2 = {}", fruit1);
    println!("move fruit2 = {}", fruit2);

    //-------------------------------------
    let x = "fruit1";

    // copies data from x to y
    // ownership rules are not applied here
    let y = x;

    println!("copy x = {}, y = {}", x, y);

    //----------- Rust Reference and mutable reference
    let mut str = String::from("hello");
    println!("string = {}", str);
    change_string(&mut str);
    println!("New string = {}", str);

    fn change_string(s: &mut String) {
        s.push_str(", Rust Reference with mut");
    }

    //-----------------Rust Module named config
    // Example: Using Module in Rust
    mod player {
        // private function
        fn focus() {
            println!("called player::focus");
        }

        // public function
        pub fn shift() {
            println!("called player::shift");
        }

        // public function
        pub fn jump() {
            // call private function focus and shift inside the module
            focus();
            shift();
            println!("called player::jump");
        }
    }
    // call public function jump from player module
    player::jump();

    //--- Nested Modules
    // nested module
    pub mod nested_player {
        pub mod sprite {
            pub fn create() {
                println!("NESTED___called player::sprite::create");
            }
        }
    }

    // call public function create from sprite module which is inside player module
    nested_player::sprite::create();

    //---------END------------------------

    //--- Matching a Variable in Rust
    let x = 2;

    // use of match expression to pattern match against variable x
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        _ => println!("x is something else"),
    }

    //------- Matching an Enum In Rust
    // enum Color {
    //     Red,
    //     Green,
    //     Blue,
    // }

    // let my_color = Color::Green;

    // // use of match expression to match against an enum variant
    // match my_color {
    //     Color::Red => println!("The color is red"),
    //     Color::Green => println!("The color is green"),
    //     Color::Blue => println!("The color is blue"),
    // }

    // Matching Option and Result Type in Rust
    //if let Expression in Rust
    let my_option: Option<i32> = Some(222);

    // use of if let expression on the Option type
    if let Some(value) = my_option {
        println!("IF let The option has a value of {}", value);
    } else {
        println!("IF let The option has no value");
    }

    //Rust Generics
    //Example: Using Generics in Rust
    // Create a HashMap with types i32 and &str
    let mut numbers: HashMap<i32, &str> = HashMap::new();

    // Insert values to numbers HashMap
    numbers.insert(1, "One");
    numbers.insert(2, "Two");

    println!("Numbers: {:?}", numbers);

    // Create a HashMap with types &str and &str
    let mut language_codes: HashMap<&str, &str> = HashMap::new();

    // Insert values to language_codes HashMap
    language_codes.insert("EN", "English");
    language_codes.insert("NE", "Nepali");

    println!("Language Codes: {:?}", language_codes);

    //----- Trait in rust
    //Define trait Printable
    trait Printable {
        fn print(&self);
    }

    //Define struct for trait
    struct PersonDetails {
        name: String,
        age: u32,
    }

    // Implement trait Printable on struct PersonDetails
    impl Printable for PersonDetails {
        fn print(&self) {
            println!("PersonDetails {{ name : {}, age:{}}}", self.name, self.age);
        }
    }

    // Define another struct to implement a trait
    struct Car {
        make: String,
        model: String,
    }

    // Define trait Printable on struct Car
    impl Printable for Car {
        fn print(&self) {
            println!("Car {{ make: {}, model: {} }}", self.make, self.model);
        }
    }

    // Utility function to print any object that implements the Printable trait
    fn print_thing<T: Printable>(thing: &T) {
        thing.print();
    }

    // Instantiate PersonDetails and Car
    let person_details = PersonDetails {
        name: "Hari".to_string(),
        age: 31,
    };
    let car = Car {
        make: "Tesla".to_string(),
        model: "Model X".to_string(),
    };

    // Call print_thing with reference of PersonDetails and Car
    print_thing(&person_details);
    print_thing(&car);

    //----------------------
    #[derive(Copy, Clone)]
    struct MyStruct {
        value: i32,
    }

    let x = MyStruct { value: 10 };
    let y = x;

    println!("x: {:?}", x.value);
    println!("y: {:?}", y.value);

    //--- File handling

    // Read a file in the local file system
    let mut data_file = File::open("src/data.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);

    //------ Write content on file

    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("src/data.txt")
        .expect("cannot open file");

    // Write to a file
    data_file
        .write("I am learning Rust!".as_bytes())
        .expect("write failed");

    println!("Appended content to a file");

    //--- Rust Macro
    // A macro which uses repetitions
    macro_rules! repeat_print {
        // match rule which matches multiple expressions in an argument
        ($($x:expr),*) => {
            $(
                println!("{}", $x);
            )*
        };
    }
    // Call the macro with multiple arguments
    repeat_print!(1, 2, 3);

    //-- React Thread
    // create a thread
    thread::spawn(|| {
        // everything in here runs in a separate thread
        for i in 0..10 {
            println!("{} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // main thread
    for i in 0..5 {
        println!("{} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //---- - Sending Messages between Threads in Rust
    // main thread starts here
    // create a new channel
    let (sender, receiver) = mpsc::channel();

    // spawn a new thread
    let handle = thread::spawn(move || {
        // receive message from channel
        let message = receiver.recv().unwrap();

        println!("Main thread Received message: {}", message);
    });

    let message = String::from("Hello, World!");
    // send message to channel
    sender.send(message).unwrap();

    // wait for spawned thread to finish
    handle.join().unwrap();
}
