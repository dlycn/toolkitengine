fn cycle()-> i32 {
    let mut i: i32 = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
    let result: i32 = loop {
        println!("in loop {i}");
        i += 1;
        if i > 10 {
            break i;
    }
    };
    println!("The result of the loop is: {}", result);
    return result
} 

fn main() { 
    let a: i16 = 12;
    let b: i16 = 34;
    let mut s: String = String::from("a is ");
    let o: i32 = cycle();
    s.push_str(&a.to_string());
    s.push_str(", b is ");
    s.push_str(&b.to_string());
    println!("{s},{o}");
    assert_eq!(s, "a is 12, b is 34");
    } 