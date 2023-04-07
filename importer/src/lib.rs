wit_bindgen::generate!("bettergreetworld");

struct BetterGreeting;

export_bettergreetworld!(BetterGreeting);

impl exports::Exports for BetterGreeting {
    fn greet(s: String) -> String {
        greeting::greet(&s)
        //greeting::greet(&s) + ", " + "what a wonderful day!"
    }
}