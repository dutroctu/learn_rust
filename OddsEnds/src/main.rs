
extern crate rand;
extern crate phrase;

use phrase::greetings::french;

fn main() {
    println!("English: {}, {}", phrase::greetings::english::hello(), phrase::greetings::english::goodbye());

    println!("French: {}, {}", french::hello(), french::goodbye());
}
