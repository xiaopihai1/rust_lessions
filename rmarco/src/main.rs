use xuyingjie_hello_macro::HelloMacro;

use xuyingjie_hello_macro::timer_duration;

#[derive(HelloMacro)]
struct MyStruct;

fn main() {
    MyStruct::hello_macro();
    test_duration();
    let result = test_return();
    println!("Result : {result}")
}

#[timer_duration]
fn test_duration() {
    let number = vec![0;999999];
    for _ in number.iter() {
       
    }
}


#[timer_duration]
fn test_return() -> i32 {
    let number = vec![1;999999];
    number.iter().sum()

}