
fn main() {

    println!("{greeting}, {name}!", greeting="Goodday", name="Jimmy");
    println!("Vector is {:?}", [1, 2, 3, 4]);
    println!("Pretty vector is {:#?}", ["A", "B", "C"]);
    let message = format!("{greeting}, {name}!", greeting="Good night", name="dude");
    println!("{}", message);
    let number: i32 = 42;
    println!("Int is {number}", number=number);

    // ------------------------------------- Underscores  ---------------------------------
    // The underscore _ is a special name - or rather, a "lack of name".
    // It basically means to throw away something:
    // this does *nothing* because 42 is a constant
    let _ = 42;
    fn get_number() -> (char, i32) {
        return ('a', 17);
    }
    // this calls `get_thing` but throws away its result
    let _ = get_number();

    // ------------------------------------- Shadowing  ---------------------------------
    // Separate bindings with the same name can be introduced - you can shadow a variable binding:
    let an_int = 13;
    let an_int = an_int + 3;
    // using `x` after that line only refers to the second `x`,
    // the first `x` no longer exists.
    println!("an_int is {}", an_int);

    // ------------------------------------- Tuples ---------------------------------
    // Tuples can be destructured when doing an assignment, 
    // which means they're broken down into their individual fields:
    let (some_char, some_int) = ('a', 17);
    println!("Tuples: some_char {} some_int {}", some_char, some_int);
    let some_text = "Hello there how long am I?";
    let middle = (some_text.len() / 2) - 1;
    let (left, right) = some_text.split_at(middle);
    println!("Left is {left} and right is {right}", right=right, left=left);
    // Of course, when destructuring a tuple, _ can be used to throw away part of it:
    let (_, _right) = some_text.split_at(middle);

    // ------------------------------------- Multi-line Statements ---------------------------------
    // Statements can span multiple lines
    let a_result = vec![1, 2, 3, 4]
    .iter()
    .map(|x| x * x)
    .fold(0, |x, y| x + y);
    println!(" ----- fold a_result is {:?} ", a_result);
    let another_result = vec![1, 2, 3, 4]
    .iter()
    .map(|x| x * x)
    .reduce(|x, y| x + y);
    println!(" ----- reduce another_result is {} ", another_result.unwrap());

    // ------------------------------------- Code blocks ---------------------------------
    // A pair of brackets declares a block, which has its own scope:
    let a_str = "Outside!";
    {
        let a_str = "Inside!";
        println!("Inside the code block: {a_str}", a_str=a_str);
    }
    println!("Outside the code block: {a_str}", a_str=a_str);
    // Blocks are also expressions, which mean they evaluate to.. a value.
    let _an_int = { 42 };
    // Inside a block, there can be multiple statements:
    let _another_int = {
        let y = 1; // first statement
        let z = 2; // second statement
        y + z // this is the *tail* - what the whole block will evaluate to
    };
    // And that's why "omitting the semicolon at the end of a function" is the same as returning,
    // ie. these are equivalent:
    fn _fair_dice_roll_return() -> i32 {
        return 4;
    }
    fn _fair_dice_roll_concise() -> i32 {
        4
    }

    // if conditionals and match are also expressions:
    fn _fair_dice_roll_if(feeling_lucky: bool) -> i32 {
        if feeling_lucky {
            6
        } else {
            4
        }
    }
    fn _fair_dice_roll_match(feeling_lucky: bool) -> i32 {
        match feeling_lucky {
            true => 6,
            false => 4,
        }
    }

    // ------------------------------------- Namespaces: crates and modules  ---------------------------------
    // The double-colon, ::, is similar but it operates on namespaces.
    // In this example, std is a crate (~ a library),
    // cmp is a module (~ a source file), and min is a function:
    let _least = std::cmp::min(10, 3);
    // use directives can be used to "bring in scope" names from other namespace:
    use std::cmp::min;
    let _new_least = min(1, 2);
    // Within use directives, curly brackets have another meaning: they're "globs".
    // use std::cmp::{min, max};

    // Types are namespaces too, and methods can be called as regular functions:
    // The str type, also called a 'string slice', is the most primitive string type.
    // It is usually seen in its borrowed form, &str. It is also the type of string literals, &'static str.
    let greeting: &'static str = "hello";
    let _length = greeting.len();
    // str is a primitive type (in scope by default).
    let _length2 = str::len("Another hello");
    // Many non-primitive types are also in scope by default: e.g. Vec
    let mut _empty_vector = Vec::new();  // Vec is a regular struct and equals std::vec::Vec::new();
    _empty_vector.push(1);
    // This works because Rust inserts the following at the beginning of each module
    // use std::prelude::v1::*;  Re-exports symbols, like Vec, String, Option and Result


    // ------------------------------------- Structs ---------------------------------
    // Structs are declared with the struct keyword:
    struct Person<'a> {
        name: &'a str,
        age: i32,
        balance: f64,
    }
    let _person1 = Person{name: "Luke", age: 33, balance: 200.333};
    // shortcut for initializing the rest of the fields from another struct
    let _person2 = Person{
        name: "Hannibal",
        age: 45,
        .._person1 // "struct update syntax", can only happen in last position, and cannot be followed by a comma.
    };
    // The rest of the fields can mean all the fields:
    let _person3 = Person{.._person1};
    // Structs, like tuples, can be destructured
    struct Point {
        x: f64,
        y: f64,
    }
    let point = Point { x: 3.0, y: 6.0 };
    let Point { x, y } = point;  // x is 3.0 and y is 6.0 in Point
    let p1 = Point{x, y};
    println!("---- p1.x is {x} p1.y is {y}", x=p1.x, y=p1.y);
    let Point { x, .. } = point;  // this throws away `point.y`
    // let used in if expressions
    struct Number {
        odd: bool,
        value: i32,
    }
    let one = Number { odd: true, value: 1 };
    let two = Number { odd: false, value: 2 };
    fn print_number(n: &Number) {
        if let Number { odd: true, value } = n {
            println!("Odd number: {}", value);
        } else if let Number { odd: false, value } = n {
            println!("Even number: {}", value);
        }
    }
    // ------------------------------------- Pass by value ---------------------------------
    // the function specifies that it requires ownership of the parameter because you pass it by value.
    // When a function requires a parameter by value, the compiler will check if the value can be copied
    // by verifying if it implements the trait Copy.
    // If it does, the value is copied (with a memcpy) and given to the function,
    // and you can still continue to use your original value.
    // If it doesn't, then the value is moved to the given function,
    // and the caller cannot use it afterwards
    fn print_number_match(n: &Number) {
        match n {
            Number { odd: true, value } => println!("Odd number: {}", value),
            Number { odd: false, value } => println!("Even number: {}", value),
        }
    }
    print_number(&one);
    print_number_match(&two);
    // A match has to be exhaustive: at least one arm needs to match
    // _ can be used as a "catch-all" pattern
    fn print_number_match_catch_all(n: &Number) {
        match n.value {
            1 => println!("One"),
            2 => println!("Two"),
            _ => println!("{}", n.value),
        }
    }
    print_number_match_catch_all(&two);

    // You can declare methods on your own types:
    impl Number {
        fn is_strictly_positive(self) -> bool {
            self.value > 0
        }
    }
    println!("Is 2 positive? {is_positive}", is_positive=&two.is_strictly_positive());

    // ------------------------------------- Traits ---------------------------------
    trait Signed {
        fn is_strictly_negative(self) -> bool;
    }
    // Orphan rules for traits. We can implement:
    // - one of our traits on anyone's type
    // - anyone's trait on one of our types
    // - not a foreign trait on a foreign type
    // 1) An implementation of our trait on our type:
    impl Signed for Number {
        fn is_strictly_negative(self) -> bool {
            self.value < 0
        }
    }
    let three = Number{odd: true, value: 3};
    println!("Is 2 negative? {is_negative}", is_negative=three.is_strictly_negative());
    // 2) Our trait on a foreign type (a primitive type, even)
    impl Signed for i32 {
        fn is_strictly_negative(self) -> bool {
            self < 0
        }
    }
    let minus_one: i32 = -1;
    println!("Is -1 negative? {}", minus_one.is_strictly_negative());
    // 3) A foreign trait on our type:
    // `Neg` trait is used to overload `-`, the unary minus operator.
    impl std::ops::Neg for Number {
        type Output = Self;
        // impl block is always for a type, so, inside that block, Self refers to that type
        fn neg(self) -> Self {
            Self {
                value: -self.value,
                odd: self.odd,
            }        
        }
    }
    let a_positive_number = Number { odd: true, value: 987 };
    let a_negative_number = -a_positive_number;
    println!("{}", a_negative_number.value);
    println!("Is a_negative_number negative? {}", a_negative_number.is_strictly_negative());

    // ------------------------------------- Markers (Traits) ---------------------------------
    // Some traits are markers - They don't say that a type implements some methods
    // For example, i32 implements trait Copy (in short, i32 is Copy)
    let a: i32 = 15;
    let b = a; // `a` is copied
    let c = a; // `a` is copied again
    fn print_i32(x: i32) {
        println!("x = {}", x);
    }
    print_i32(a); // `a` is copied again
    // the Number struct is not Copy so
    // move occurs because `n` has type `Number`,
    // which does not implement the `Copy` trait
    // let n = Number { odd: true, value: 51 };
    // let m = n; // `n` is moved into `m`
    // let o = n; // error: use of moved value: `n`

    // ------------------------------------- Pass by reference and borrowing ---------------------------------
    // Immutable reference
    fn print_immutable_reference_number(n: &Number) {
        println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
    }
    // Mutable reference
    fn invert(n: &mut Number) {
        n.value = -n.value;
    }
    // Declare n as mutable
    let mut mutable_n = Number { odd: true, value: 51 };
    print_immutable_reference_number(&mutable_n); // mutable_n is borrowed for the time of the call
    invert(&mut mutable_n);  // Explicit borrow of mutable reference `mutable_n`

    // ------------------------------------- Traits methods and borrowing ---------------------------------
    // Trait methods can also take self by reference or mutable reference
    impl std::clone::Clone for Number {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }
    // When invoking trait methods, the receiver is borrowed implicitly:
    let a_new_number = Number { odd: true, value: 23 };
    print_number(&a_new_number);  // a_new_number is borrowed for the time of the call
    let mut a_cloned_number = a_new_number.clone(); // by calling clone() we implicitly borrow the reference a_new_number
    a_cloned_number.value += 100;
    print_number(&a_cloned_number);
    // The clone() is equivalent to std::clone::Clone::clone(&another_new_n): implicit borrowing
    let another_new_number = a_new_number.clone();
    let _a_new_cloned_m = std::clone::Clone::clone(&another_new_number);


    // ------------------------------------- Foreign Markers Traits: Copy ---------------------------------
    // Some marker traits like Copy have no methods
    // note: `Copy` requires that `Clone` is implemented to
    impl std::marker::Copy for Number {}
    let five = Number { odd: true, value: 5 };
    let five_clone = five.clone();  // clone() still works as before
    // Number values will no longer be moved
    let five_copy = five; // `five_copy` is a copy of `five`
    let another_five = five; // same. `fice` is neither moved nor borrowed.
    // Common traits can be implemented automatically by using the derive attribute
    #[derive(Clone, Copy)]  // Expands to `impl Clone for Number` and `impl Copy for Number` blocks
    struct CloneableNumber {
        odd: bool,
        value: i32,
    }

    // ------------------------------------- Generic Functions ---------------------------------
}
