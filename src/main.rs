// #[derive(Clone)]
// struct MyStruct {
//     name: String,
//     value: i32,
// }

// fn main() {
//     let original = MyStruct {
//         name: String::from("Rust"),
//         value: 100,
//     };

//     let cloned = original.clone();

//     println!("Original: {} - {}", original.name, original.value);
//     println!("Cloned: {} - {}", cloned.name, cloned.value);
// }

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() {
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("{}", concatenated_string);
}
