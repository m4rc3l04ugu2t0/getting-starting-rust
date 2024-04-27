pub mod module_fn {
    fn say_hello(name: &str, color: &str) {
        println!("Hello {name},  your color {color}")
    }

    fn add_number(x: i32, y: i32) -> i32 {
        if x < 1 {
            return y;
        };
        x + y
    }

    fn double(n: i32) -> i32 {
        n * 2
    }

    fn parse_calc() {
        let input = "55 44 22 47 88 88";

        let result: Vec<i32> = input
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .map(double)
            .collect();
        println!("{:?}", result);
    }

    pub fn function() {
        let _result = say_hello("Marcelo", "red");

        let y = {
            say_hello("Augusto", "blue");
            let _x = 5;
        };
        println!("{:?}", y);

        let res = add_number(5, 5);
        println!("{res}");
        parse_calc();
    }
}
