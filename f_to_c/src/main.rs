use std::io;

fn main() {

    let mut f_temp: f32 = 0.0;
    
    loop {

        let mut f_input = String::new();

        println!("Input a Fahrenheit temperature");

        io::stdin()
            .read_line(&mut f_input)
            .expect("Invalid input");
        
        match f_input.trim().parse() {
            Ok(num) => { 
                f_temp = num; 
                break; 
            },
            Err(_) => continue,
        };
    }

    println!("{}", f_temp);
    let c_temp = (f_temp - 32.0) * (5.0/9.0);

    println!("That temperature in Celsius is: {}", c_temp);
}
