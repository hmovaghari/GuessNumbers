use std::{io::Split, vec};
use rand::Rng;
use colored::Colorize;

static GLOBAL_Invalid4DigitNumber: &str = "Invalid 4 digit number";

fn main()
{
    loop
    {
        /* Generate4DigitRandomNumber */
        let randomNumbers: Vec<String> = Generate4DigitRandomNumber();
        /* Generate4DigitRandomNumber */

        /* Get4DigitNumberFromUserAndCheckItIn10Steps  */
        Get4DigitNumberFromUserAndCheckItIn10Steps(&randomNumbers);
        /* Get4DigitNumberFromUserAndCheckItIn10Steps  */

        println!("{}", "Play again? y or n".bright_blue());
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input);
        if input.trim().to_lowercase() != "y"
        {
            break;
        }
    }
    /* Play again? */
}

fn Get4DigitNumberFromUserAndCheckItIn10Steps(randomNumbers: &Vec<String>)
{
    let mut guessSteps = 0;
    while guessSteps < 10
    {
        let isUserWin = Get4DigitNumberFromUserAndCheckIt(&randomNumbers);
        if isUserWin
        {
            
        }
        guessSteps += 1;
    }

    if guessSteps == 10
    {
        println!("{}", "Game over".red());
    }
    else
    {
        println!("{}", "You win".green());
    }
}

fn Get4DigitNumberFromUserAndCheckIt(randomNumbers: &Vec<String>) -> bool
{
    println!("{}", "Input your guess (4 digit number)".bright_blue());
    let mut inputString: String = String::new();
    std::io::stdin().read_line(&mut inputString);
    if inputString.trim().chars().count() != 4
    {
        println!("{}", GLOBAL_Invalid4DigitNumber.red());
        return false;
    }
    return Check4DigitNumber(&randomNumbers, &inputString);
    // let mut input: Vec<String> = Vec::new();
}

fn Check4DigitNumber(random_numbers: &[String], input_string: &str) -> bool
{
    return false;
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
