use std::vec;
use rand::Rng;

fn main()
{
    loop
    {
        /* Generate4DigitRandomNumber */
        let randomNumbers: Vec<String> = Generate4DigitRandomNumber();
        /* Generate4DigitRandomNumber */

        /* Play again? */
        println!("Play again? y or n");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input);
        if input.trim().to_lowercase() != "y"
        {
            break;
        }
    }
    /* Play again? */
}

fn Generate4DigitRandomNumber() -> Vec<String>
{
    let mut result: Vec<String> = Vec::new();

    let mut numbers: Vec<String> = Vec::new();
    let mut counter = 0;
    while counter < 10
    {
        numbers.push(format!("{counter}"));
        counter += 1;
    }

    counter = 0;
    while counter < 4
    {
        let from = if counter == 0 {1} else {0};
        let index = rand::thread_rng().gen_range(from..numbers.len());
        result.push(numbers[index].clone());
        numbers.remove(index);
        counter += 1;
    }

    result
}
