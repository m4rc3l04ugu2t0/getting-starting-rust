pub mod mod_variables {
    use crate::SECONDS_IN_HOUR;
    pub fn variables() {
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
}
// RAII
