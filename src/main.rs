use::std::io;

fn main() {
    loop {
        println!("Please enter a number corresponding to a test function");
        let mut choose = String::new();
        io::stdin().read_line(&mut choose).expect("Failed to read line");
        let choose: u32 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choose {
            1 => and_example(),
            2 => or_example(),
            3 => if_then_example(),
            4 => xor_example(),
            5 => xnor_example(),
            _ => println!("Please type a valid number."),
        }
        break;
    }
}

fn and_example(){
    let mut temp = String::new();

    println!("Choose the value of the first bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let first_bool: bool = matches!(temp.trim(), "true");
    temp.clear();

    println!("Choose the value of the second bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let second_bool: bool = matches!(temp.trim(), "true");

    //& is used to check if both variables are true
    if first_bool && second_bool{
        println!("The result of the AND is true");
    }
    else {
        println!("The result of the AND is false");
    }
}

fn or_example(){
    let mut temp = String::new();

    println!("Choose the value of the first bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let first_bool: bool = matches!(temp.trim(), "true");
    temp.clear();

    println!("Choose the value of the second bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let second_bool: bool = matches!(temp.trim(), "true");

    //|| is used to check if at least one of the variables are true
    if first_bool || second_bool{
        println!("The result of the OR is true");
    }
    else {
        println!("The result of the OR is false");
    }
}

fn if_then_example(){
    let mut temp = String::new();

    println!("Choose the value of the first bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let first_bool: bool = matches!(temp.trim(), "true");
    temp.clear();

    println!("Choose the value of the second bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let second_bool: bool = matches!(temp.trim(), "true");

    if !first_bool || second_bool{
        println!("The result of the If Then is true");
    }
    else {
        println!("The result of the If Then is false");
    }
}

fn xor_example(){
    let mut temp = String::new();

    println!("Choose the value of the first bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let first_bool: bool = matches!(temp.trim(), "true");
    temp.clear();

    println!("Choose the value of the second bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let second_bool: bool = matches!(temp.trim(), "true");

    if first_bool ^ second_bool{
        println!("The result of the XOR is true");
    }
    else {
        println!("The result of the XOR is false");
    }
}

fn xnor_example(){
    let mut temp = String::new();

    println!("Choose the value of the first bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let first_bool: bool = matches!(temp.trim(), "true");
    temp.clear();

    println!("Choose the value of the second bool. Type true or false.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let second_bool: bool = matches!(temp.trim(), "true");

    if first_bool == second_bool{
        println!("The result of the XNOR is true");
    }
    else {
        println!("The result of the XNOR is false");
    }
}