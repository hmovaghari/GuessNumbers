fn main()
{
    loop
    {
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
