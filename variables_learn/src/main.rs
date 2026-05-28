fn main() {
    let age = 23; //immurable
    println!("my age is{age}");

    let mut  name = "alpha"; //mutable
    println!("my name is {name}");

    const PI:f64= 3.14;  // for const it is mandatory to declare the type of the variable
    println!("the value of pi is {PI}");


    //Data Types

    let a = random_number().wrapping_add(56);
    println!("value of a is {a}");

    fn random_number() -> u8{
        200
    }

    //array
    let a = [2,54,5434,23,1];
    let b = [10;5]; // it means [10,10,10,10,10]

    let first_of_a = a[1];
    
}
