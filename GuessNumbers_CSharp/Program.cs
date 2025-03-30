
do
{
    /* Generate4DigitRandomNumber */
    List<string> randomNumbers = Generate4DigitRandomNumber();
    /* Generate4DigitRandomNumber */

    /* Play again? */
    Console.WriteLine("Play again? y or n");
} while (Console.ReadLine()?.ToLower() == "y");
/* Play again? */

List<string> Generate4DigitRandomNumber()
{
    var result = new List<string>();
    
    var numbers = new List<string>();
    for (int i = 0; i < 10; i++)
    {
        numbers.Add(i.ToString());
    }

    for (int i = 0; i < 4; i++)
    {
        Random random = new Random();
        var from = i == 0 ? 1 : 0;
        var index = random.Next(from, numbers.Count);
        result.Add(numbers[index]);
        numbers.RemoveAt(index);
    }

    return result;
}
