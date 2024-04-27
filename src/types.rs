pub mod my_module {

    pub fn types() {
        // let x = 5;
        // let y = x + 000_000_000;
        // let h = 0xff;
        // let o = 0o77;
        // let b = 0b1111_0000;
        // let by = b'A';
        // let f = 42.2;
        // let c = 'p';
        // let bo = true || false;
        // -------------------------------
        // Tupla
        let mut numbers: (i32, i32, f64) = (1, 2, 3.5);
        // let (a, b, c) = numbers;
        numbers.0 = 50;
        // X! number.0 = "Hm" !Err
        numbers = (numbers.0 * 2, 30, 40.0);
        println!(
            "num.0: {:?}, num.1: {:?}, num.2: {:?}",
            numbers.0, numbers.1, numbers.2
        );
        vec();
    }

    fn vec() {
        let numbers: [i32;5] = [1, 2, 3, 4, 5];
        // numbers[0] => 1
        // slice
        println!("{:?}", &numbers[1..4]);
    }
}
