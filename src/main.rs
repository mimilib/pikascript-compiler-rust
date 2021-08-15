mod class_info;
mod import_info;
mod method_info;
mod my_string;
mod object_info;
mod compiler;
use class_info::*;
use import_info::*;
use method_info::*;
use object_info::*;
use compiler::*;

fn main() {
    MethodInfo::new(
        &String::from("Test"),
        String::from("def test(test:str)->str:"),
    )
    .unwrap()
    .name;
}
