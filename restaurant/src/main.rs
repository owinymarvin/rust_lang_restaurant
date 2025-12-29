use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind; 

fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("{v:?}");
    println!("{v:?}");
    println!("{v:?}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");

    // Hash Maps
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    // handling errors.
    // panic!("crash and burn");  // this causes a manual panic. Also set the panic = 'abort' in the Cargo.toml file

    // Recoverable Errors.
    let greeting_file_result = File::open("./hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("File successfully opened");
            file
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("./hello.txt"){
                Ok(fc) => fc, 
                Err(err) => panic!("Problem creating the file: {err:?}"), 
            }, 
            other_errors => panic!("Error opening the file: {other_errors:?}")
        }
    };
    println!("{greeting_file:#?}"); 
}
