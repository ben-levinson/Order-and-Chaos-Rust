[![Build Status](https://travis-ci.com/blevinson19/Order-and-Chaos-Rust.svg?branch=master)](https://travis-ci.com/blevinson19/Order-and-Chaos-Rust)

## Rules of the game
Order and Chaos is an asymmetric tic-tac-toe variant. The objective of the order player is to get either
5 X's or 5 O's in a row. Either player may place an X or an O in any open space when it is their turn.
As in typical tic-tac-toe this can be achieved horizontally, vertically, or on a diagonal. 
The objective of the chaos player is to fill the board such that Order cannot achieve this goal. 6 pieces in a row do not
count as a win for Order. The Order player makes the first move.

## Playing the game with the GUI
### Playing against another human
Playing against a human is the default mode for the game. There is a toggle button
that allows the player to select whether an X or an O should be placed. Then the player selects where to place the piece. 
The information about where the player clicked is transmitted to the board library which checks the board for a win condition. 
When a game is won, a message displaying the winner is shown, and no more pieces may be placed. 

### Starting a new game
On the left-hand side of the window, there is a button to allow the game to be reset. 

### Playing against the AI
Beneath the new game button another toggle button displays whether the game 
will be played against a human or the AI. If the AI is chosen, another toggle
appears to select whether the opponent is Order or Chaos. Once the opponent is selected,
click new game. 