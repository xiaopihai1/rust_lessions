use xuyingjie_hello_macro::HelloMacro;

#[derive(xuyingjie_hello_macro_derive::HelloMacro)]
struct MyStruct;
fn main() {
    MyStruct::hello_macro();
}
