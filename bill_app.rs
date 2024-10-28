fn get_input()->Option<String>{
    let mut buffer = String::new()
    while io::stdin().read_line(&mut buffer).is_err(){
        print!("Please enter your data again")
    }
    let input = buffer.trim().to_owned();
    if &input =""{
        None
    } else {
        Some(input)
    }
}


fn main(){}