fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("{v:?}");
    println!("{v:?}");
    println!("{v:?}");

    let hello = "Здравствуйте";

    let s = &hello[0..1];
    println!("{s}");

    
}
