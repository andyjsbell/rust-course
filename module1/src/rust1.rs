use std::mem;

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
}