# ♚ Multiplayer Chess ♔
This library handles creation and move generation within a chess engine.

### Usage
#### Use the library directly
* To run the chess application on a terminal, use ```cargo run```
* Application prompts for player names.
* Application prompts for source and destination.
* Source should be valid: Should have a piece of the right color and should
  be within the dimensions of the board.
* Destination should be valid: Should not have a piece of the same color
  and should be within the dimensions of the board.

#### Use chess module independently
* Create game with player names and start the game:
  ```rust
  let mut game = Chess::new(String::from(name1), String::from(name2));
  game.start();
  ```
  
### Suggest improvements
Feel free to open a PR / Issue or suggest improvements / modifications / increments

