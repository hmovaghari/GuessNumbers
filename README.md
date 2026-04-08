# GuessNumbers

GuessNumbers is a small console game inspired by Mastermind. In each round, the program generates a secret 4-digit number with non-repeating digits, and you have 10 chances to guess it.

## Game Rules

- The secret number is always 4 digits.
- Digits do not repeat.
- The first digit is never zero.
- You have 10 attempts to guess the full number.
- After each guess, the color of each digit shows the result:
  - Green: the digit is correct and in the correct position.
  - Yellow: the digit exists in the secret number, but its position is wrong.
  - Red: the digit does not exist in the secret number.

## Project Structure

This repository currently includes two console implementations of the game:

- `GuessNumbers_CSharp`: C# / .NET 8 version
- `GuessNumbers_Rust`: Rust version

## Run the C# Version

Requirements:

- .NET 8 SDK

Run:

```bash
dotnet run --project GuessNumbers_CSharp/GuessNumbers_CSharp.csproj
```

## Run the Rust Version

Requirements:

- Rust toolchain
- Cargo

Run:

```bash
cargo run --manifest-path GuessNumbers_Rust/Cargo.toml
```

## Sample Flow

1. Start the game.
2. Enter a 4-digit number as your guess.
3. Review the color feedback for each digit.
4. Continue until you guess correctly or use all 10 attempts.
5. Choose whether to play another round.

## License

This project is available under the terms of the `LICENSE` file in this repository.
