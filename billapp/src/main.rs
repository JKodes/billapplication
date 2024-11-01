use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill){
        self.inner.insert(bill.name.to_string(),bill);
    }
    fn get_all(&self) -> Vec<&Bill>{
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool{
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true 
            }
            None => false,
        }
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

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input(){
            Some(input)=> input,
            None => return None,
        };
        if &input == ""{
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
                Ok(amount) => return Some(amount),
                Err(_) => println!("Please enter a number"),
        }
    }
}


mod menu {
    use crate::{get_input, get_bill_amount, Bill, Bills};

     pub fn add_bill(bills: &mut Bills){
        println!("Bill name:");
        let name = match get_input(){
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount(){
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill {name, amount};
        bills.add(bill);
        println!("Bill added")
    }

    pub fn remove_bill(bills: &mut Bills){
        for bill in bills.get_all(){
            println!("{:?}", bill);
        }
        println!("Enter the name of the bill so it can get removed:");

        let name = match get_input(){
            Some(name) => name,
            None => return,
        };
        if bills.remove(&name){
            println!("bill has been removed")
        } else {
            println!("bill cannot be found so nothing was deleted")
        }
    }

    pub fn view_bills(bills: &Bills){
        for bill in bills.get_all(){
            println!("{:?}", bill);
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu>{
        match input{
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            _ => None,
        }
    }
    fn show(){
        println!("");
        println!(" == Bill Manager == ");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("3. Remove Bill");
        println!("");
        println!("Enter selection: ");
    }
}

fn main(){
    let mut bills = Bills::new();
    loop{
        MainMenu::show();
        let input = get_input().expect("no data entered");
        match MainMenu::from_str(input.as_str()){
            Some(MainMenu::AddBill)=> menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            None => return,
        }
    }
}