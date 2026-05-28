fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("s = {s}");

    let s1 = String::from(" i am s1");
    let s2 = s1.clone(); //expensive
    println!("x is {s1} and value of y is {s2}");

    let m = String::from("alpha");
    gives_ownership(m);

    let s = takes_ownership();
    println!("{s}");

    let length = calculate_len(&s); //used reference than ownership
    println!("the length is {length}");

    println!("{s}"); //now if we print s again, it will work

}

fn takes_ownership() -> String {
    let s = String::from("this is a string from a takes ownership function");
    s
}

fn gives_ownership(s: String) {
    println!("hello there, its {s}.");
}

fn calculate_len(s: &String) -> usize {
    // -> &String takes reference , not the ownership
    let result = s.len();
    result
}

