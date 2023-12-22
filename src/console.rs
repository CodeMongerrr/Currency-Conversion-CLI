use crate::get_user_input;


pub struct Console;

impl Console {
    pub fn clear(&self){
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn conversion(&self){
        println!("------------------------------------");
        println!("---Welcome to Currency Conversion---");
        println!("------------------------------------");
        println!("-----Select your Base Currency------");
        let base_currency = get_user_input().trim().to_string();
        println!("--Fetching Selected Base Currency---");
        println!("------------------------------------");
        println!("------------------------------------");
        println!("------------------------------------");
        println!("--Select your Conversion Currency---");
        let currency = get_user_input().trim().to_string();
        println!("-------Conversion in Process--------");
        println!("1 {} = {} {}", currency,,base_currency)
    }
}