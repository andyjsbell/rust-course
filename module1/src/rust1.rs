use std::mem;
use std;
fn foo_function() {
    let y = 5;
    println!("foo_function is being called with result is {}", y);
}

pub fn main1() {
    //data types
    let bool_variable = true;
    println!("My bool variable is {}", bool_variable);
    let bool_two:bool = false;
    println!("My bool_two variable is {}", bool_two);
    println!("size of bool is {}", mem::size_of_val(&bool_two));
    let c = 'c';
    println!("size of char is {}", mem::size_of_val(&c));
    let number:i32 = 42;
    println!("size of i32 is {}", mem::size_of_val(&number));
    println!("c={} and number={}", c, number);
    let double_number:f32 = 1.0;
    println!("size of f32 is {}", mem::size_of_val(&double_number));
    println!("double_nunber={}", double_number);
    let my_string  = "hello string";
    println!("my_string={}", my_string);

    // stack and heap
    let stack = 10;
    let mut stack_mut = 10;
    let heap = Box::new(10);

    println!("stack={} heap={}", stack, heap);
    println!("size of stack={} and size of heap pointer={}", mem::size_of_val(&stack), mem::size_of_val(&heap));
    println!("size of stack={} and size of heap={}", mem::size_of_val(&stack), mem::size_of_val(&*heap));

    // scope and shadowing
    foo_function();
    // Scoped area
    {
        let stack = 21; // shadowed
        stack_mut = 20;  // Mutable 
        let scoped_var = 4;
        println!("scoped_var={} and stack={}", scoped_var, stack);
    }

    // Shadow
    let scoped_var = 15;
    println!("Redeclared scoped_var={} and stack={} and stack_mut={}", scoped_var, stack, stack_mut);

    // operators

    let number = 2;
    let add_number = number + 5;
    println!("add_number={}", add_number);

    let mut another_number = 7;
    another_number -= 2;
    println!("another_number = {}", another_number);

    let cubed = i32::pow(number, 3);

    println!("number raised to the power 3={}", cubed);

    // We had to cast using 'as f64' here
    let cubed_times_pi = cubed as f64 * std::f64::consts::PI;
    println!("cubed_times_PI={}", cubed_times_pi);
    
    let bool_result = 5 < 6;
    println!("bool_result={}", bool_result);
}


