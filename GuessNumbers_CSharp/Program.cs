namespace GerssNumbers
{
    class Program
    {
        public static string invalid4DigitNumber = "Invalid 4 digit number";

        public static void Main(string[] args)
        {
            do
            {
                List<string> randomNumbers = Generate4DigitRandomNumber();
                Get4DigitNumberFromUserAndCheckItIn10Steps(randomNumbers);

                Console.ForegroundColor = ConsoleColor.Blue;
                Console.WriteLine("Play again? y or n");
                Console.ResetColor();
            } while (Console.ReadLine()?.ToLower() == "y");
        }

        private static void Get4DigitNumberFromUserAndCheckItIn10Steps(List<string> randomNumbers)
        {
            var guessSteps = 0;
            for (guessSteps = 0; guessSteps < 10; guessSteps++)
            {
                var isUserWin = Get4DigitNumberFromUserAndCheckIt(randomNumbers);
                if (isUserWin)
                {
                    break;
                }
            }

            if (guessSteps == 10)
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine("Game over");
            }
            else
            {
                Console.ForegroundColor = ConsoleColor.Green;
                Console.WriteLine("You win");
            }
        }

        private static bool Get4DigitNumberFromUserAndCheckIt(List<string> randomNumbers)
        {
            Console.ForegroundColor = ConsoleColor.Blue;
            Console.WriteLine("Input your guess (4 digit number)");
            Console.ResetColor();
            var input = Console.ReadLine()?.ToCharArray();
            if ((input?.Length ?? 0) != 4)
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine(invalid4DigitNumber);
                return false;
            }
            return Check4DigitNumber(randomNumbers, input);
        }

        private static bool Check4DigitNumber(List<string> randomNumbers, char[] input)
        {
            var result = true;
            var index = 0;
            input.ToList().ForEach(i =>
            {
                var isCorrentNumber = i.ToString() == randomNumbers[index];
                if (!isCorrentNumber)
                {
                    result = false;
                }
                if (!randomNumbers.Contains(i.ToString()))
                {
                    Console.ForegroundColor = ConsoleColor.Red;
                }
                else if (isCorrentNumber)
                {
                    Console.ForegroundColor = ConsoleColor.Green;
                }
                else
                {
                    Console.ForegroundColor = ConsoleColor.Yellow;
                }
                Console.Write(i);
                ++index;
            });
            Console.WriteLine();
            return result;
        }


        private static List<string> Generate4DigitRandomNumber()
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
    }
}
