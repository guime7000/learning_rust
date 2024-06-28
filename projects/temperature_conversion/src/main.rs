use std::io;

fn main() {
        
    loop{

        println!("Input °F temperature: ");

        let mut f_temp = String::new();
        
        io::stdin()
        .read_line(&mut f_temp)
        .expect("Failed to read line");

        match f_temp.trim().parse::<f64>() {
            Ok(num) => println!("{}°F is {:.5}°C.\n",num, (5./9.)*(num-32.)),
            Err(_) => {println!("Temperature must be a float value ! Please try again.\n");
                    },
        };
    }
}
