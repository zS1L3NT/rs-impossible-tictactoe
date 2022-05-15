# Impossible TicTacToe

![License](https://img.shields.io/github/license/zS1L3NT/rs-impossible-tictactoe?style=for-the-badge) ![Languages](https://img.shields.io/github/languages/count/zS1L3NT/rs-impossible-tictactoe?style=for-the-badge) ![Top Language](https://img.shields.io/github/languages/top/zS1L3NT/rs-impossible-tictactoe?style=for-the-badge) ![Commit Activity](https://img.shields.io/github/commit-activity/y/zS1L3NT/rs-impossible-tictactoe?style=for-the-badge) ![Last commit](https://img.shields.io/github/last-commit/zS1L3NT/rs-impossible-tictactoe?style=for-the-badge)

Impossible TicTacToe is a script that allows you to challenge the computer to a game of tic tac toe. However it is technically impossible to win against the computer. This script uses the minimax algorithm to calculate all possible game scenarios and make sure the game will end with either a tie or you losing.

## Motivation

I was bored and burnt out

## Features

-   TicTacToe board<br>
    ![image](https://user-images.githubusercontent.com/26828488/168480784-f1185514-c213-4f92-9168-3c8915106caf.png) ![image](https://user-images.githubusercontent.com/26828488/168480791-8682fe23-d4f3-4ba4-a058-52a68b39ca5a.png)
-   Choose which player goes first<br>
    ![Choose player](https://i.ibb.co/YWxz6YS/Untitled.png)
-   Minimax Algorithm

## Usage

```
$ cargo run src/main.rs
```

## Credits

I used [this GeeksForGeeks article](https://www.geeksforgeeks.org/minimax-algorithm-in-game-theory-set-3-tic-tac-toe-ai-finding-optimal-move/) as a reference when implementing the minimax algorithm.

## Built with

- Rust
  - [![prettytable-rs](https://img.shields.io/badge/prettytable--rs-%5E0.8-blue?style=flat-square)](https://docs.rs/prettytable-rs/0.8)
