wit_bindgen::generate!("greetworld");

struct Greeting;

export_greetworld!(Greeting);

impl greeting::Greeting for Greeting {
    fn greet(s: String) -> String {
        let mut s1 = "Hello".to_string();
        s1.push_str(&s);
        s1
    }
}