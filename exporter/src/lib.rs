wit_bindgen::generate!("greetworld");

struct Greeting;

export_greetworld!(Greeting);

impl greeting::Greeting for Greeting {
    fn greet(s: String) -> String {
        "Hello".to_string() + " " + &s
    }
}