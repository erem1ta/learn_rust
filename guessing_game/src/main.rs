pub(crate) use rand::Rng;
pub(crate) use std::cmp::Ordering;
pub(crate) use std::io;

fn main() {
    println!("guess the number!");

    // create a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_number);

    loop {
        // reading input
        println!("Input: ");
        let mut guess = String::new();
        io::stdin().
            read_line(&mut guess). // read after Enter
            expect("Read line failed"); // expect is use to handle the error -> crash & print

        // convert guess to unsigned int 32.
        let guess: u32 = match guess.
                        trim(). // trim (remove \n || \r\n)
                        parse() // parse (convert to number)
        {

            Ok(num) => num, // num is the return value if OK
            Err(_) => { // _ means catchall -> catch all Err values
                println!("invalid input");
                continue;
            },
        };

        // compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Equal => {
                println!("ok");
                break; // exit the loop
            }
            Ordering::Greater => println!("big"),
        }
    }
}
