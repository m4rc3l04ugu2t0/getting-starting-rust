pub mod module_s {
    use std::io;

    pub fn s() {
        println!("{:-^40}", "Caulculadora");
        let mut s = String::new();
        let banner = "Digite uma sequencia de números\n\
        separados por vírgula,\n\
        exemplo: 1, 2, 4\n\
        ";
        println!("{banner}");
        println!("Digite algo.");
        io::stdin().read_line(&mut s).expect("Error read console.");

        // fn clean(c: &str) -> &str {
        //     c.trim()
        // }

        let nums: Vec<i32> = s.split(",").map(|c| c.trim().parse().expect("Error parse")).collect();

        let result: i32 = nums.iter().sum();
        println!("A soma de todos os números é: {:?}", result);
        println!("Quantidades de letras {}", s.trim().chars().count());
        println!("{}", "-".repeat(40));
    }
}
