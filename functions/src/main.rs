fn main() {
    println!("Hello, world!");

    my_function(2);

    is_even(23);

    looping();
}

fn my_function(x: i32) {
    println!("hello there {x}");
}

fn is_even(x:i32) -> bool{
    if x %2 ==0{
        return true;
    }
    println!("the number is not even");
    false
}

//loop
fn looping(){
    let mut a = 0;
    loop{
        println!("this is number {a}");

        if a ==10{
            break;
        }
        a = a +1;

    }
} 