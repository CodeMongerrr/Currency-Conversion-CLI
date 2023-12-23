use crate::get_user_input;

pub struct Console;
pub struct Conversion_ {
    pub base_currency: String,
    pub currency: String,
}
impl Console {
    pub fn clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn conversion(&self)-> Conversion_ {
        self.clear();
        println!("------------------------------------");
        println!("---Welcome to Currency Conversion---");
        println!("------------------------------------");
        println!("-----Select your Base Currency------");
        let base_currency = get_user_input().trim().to_string();
        println!("--Select your Conversion Currency---");
        let currency = get_user_input().trim().to_string();
        println!("-------Conversion in Process--------");
        let base = base_currency.clone();
        let curr = currency.clone();
        return Conversion_ {base_currency: base, currency:curr}
    }
}
