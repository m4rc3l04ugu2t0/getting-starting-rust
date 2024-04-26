const SECONDS_IN_MINUTE: u32 = 60;
const MINUTE_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTE_IN_HOUR;

fn main() {
    let total = 30;
    println!("trabalhou {total} horas.");
    {
        let total = total - 20;
        println!("Não trabalhou {total} horas.");
    }
    let total = 44;
    println!("trabalhou {total} horas.");
    let total_in_seconds = total * SECONDS_IN_HOUR;
    println!("trabalhou {total_in_seconds} segundos.");
}
// RAII

