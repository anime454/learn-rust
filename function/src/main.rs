
use rand::Rng; // 0.8.5

fn main() {
    let name = String::from("world");
    hello_world(name);

    let name = "new world";
    let greeting = random_greeting(name);
    println!("{}", greeting);
}

fn hello_world(name: String) {
    println!("Hello, {}!", name);
}

fn random_greeting(name: &str) -> String {
    let greetings = vec!["Hello", "Hi", "Hey", "Yo"];
    let num = rand::thread_rng().gen_range(0..4);
    format!("{}, {}!", greetings[num], name)
}