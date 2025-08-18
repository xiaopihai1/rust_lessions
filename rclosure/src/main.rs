fn main() {
    let handlers = vec![return_closure(), return_closure_init()];
    for handler in handlers {
        println!("{}", handler(100));
    }
}



fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


fn return_closure_init() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}