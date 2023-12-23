mod console;
mod curr_conv;
use curr_conv::CurrencyData; // Replace "your_module_name" with the actual module name
use console::Console;
use std::io::stdin;

fn get_user_input() -> String {
    let mut option: String = String::new();
    stdin()
        .read_line(&mut option)
        .expect("An error occurred reading user input");
    option
}
#[tokio::main]
async fn main() {
    let console: Console = Console;
    let conversion_data:console::Conversion_ = console.conversion();

    let base = conversion_data.base_currency.clone();
    let curr = conversion_data.currency.clone();
    let curr_clone = conversion_data.currency.clone();
    // let convertor = curr_conv::CurrencyData::get_conversion(base, curr);
    match CurrencyData::get_conversion(base , curr).await {
        Ok(conversion_data) => {
            println!("1 {:?} = {:?} {:?}",curr_clone,conversion_data.value, conversion_data.code);
        }
        Err(err) => {
            println!("Error getting conversion data: {:?}", err);
        }
    }
}
