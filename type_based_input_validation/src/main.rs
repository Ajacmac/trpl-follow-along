pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {

    let guess = fetch_guess();

    loop {

        if guess.value == 80 {
            println!("The secret number will be bewteen 1 and 100.");
            continue;
        } else {
            let guess = fetch_guess();
        }

    }
}

fn fetch_guess() -> Guess {
    let guess = Guess { value: 50 };


    guess
}