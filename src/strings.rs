pub mod module_strings {
   pub fn strings() {
        let  l = "Rust"; // str // 1000001 // F4241
        // string slice, string reference
        
        // Heap String
        let mut s = String::new();
        s.push_str("Hello, ");
        s.push('W');
        s.push_str("orld!");

        let ss = String::from("Salve!");

        let v = ['d', 'a', 'r', 'k'];
        let i = String::from_iter(v);


        // let c: String = "Hmmmm".into(); 

        println!("{}, {}, {}, {}", s, l, ss, i);
    }
}
