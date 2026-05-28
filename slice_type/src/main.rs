fn main() {
    let mut s=String::from("Hello world");
    // let res = find_first_word(&s);
    // s.clear();

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Hello = {}",hello);
    print!("world = {}",world);
}

// fn find_first_word(input:&String)->usize{ 
    let s = input.as_bytes();
    for(i,&item) in s.iter().enumerate(){
        if item ==b' ' { 
            return i;
        }
    }
    s.len()
}
