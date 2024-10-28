use std::io;


#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: vec![]
        }
    }

    fn add(&mut self, bill: Bill){
        self.inner.push(bill);
    }
    fn get_all(&self) -> Vec<&Bill>{
        self.inner.iter().collect()
    }
}

fn get_input()->Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        print!("Please enter your data again")
    }
    let input = buffer.trim().to_owned();
    if &input ==""{
        None
    } else {
        Some(input)
    }
}

fn get_bill_amounts() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input(){
            Some(input)=> input,
            None => return None,
        };
        if &input == ""{
            return None;
        }
    }
}


mod menu {
    use crate::{get_input, Bill, Bills};

     pub fn add_bill(bills: &mut Bills){
        println!("Bill name:");
        let name = match get_input(){
            Some(input) => input,
            None => return,
        };
        let amount = match get_input(){
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill {name, amount};
        bills.add(bill)
        println!("Bill added")
    }
}

enum MainMenu {
    AddBill,
    ViewBill
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu>{
        match input{
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            _ => None,
        }
    }
    fn show(){
        println!("");
        println!(" == Bill Manager == ");
        println!("1. AddBill");
        println!("2. ViewBill");
        println!("");
        println!("Enter selection: ");
    }
}

fn main(){

    loop{
        MainMenu::show();
        let input = get_input().expect("no data entered");
        match MainMenu::from_str(input.as_str()){
            Some(MainMenu::AddBill)=>(),
            Some(MainMenu::ViewBill) =>(),
            None => return,
        }
    }
}