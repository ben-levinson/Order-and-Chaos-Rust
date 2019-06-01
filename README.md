[![Build Status](https://travis-ci.com/blevinson19/Order-and-Chaos-Rust.svg?branch=master)](https://travis-ci.com/blevinson19/Order-and-Chaos-Rust)

## Current State of the project
Order and Chaos is playable via on the GUI between two human players. A minmax algorithm for both the order player and the chaos player is implemented, but still needs to be integrated into the game. 
We've thought about a couple of approaches with rayon, but need to get the single-threaded version integrated first.
The GUI is feature complete, but could probably use some refactoring to reduce the amount of information shared between the board library and the GUI client as well as cleaning it up a little bit.

## Playing the game with the GUI
Right now, the computer player still needs to be integrated into the game. Two humans can play against each other in the GUI. Order goes first. There is a toggle button
that allows the player to select whether an X or an O should be placed. Then the player selects where to place the piece. The information about where the 
player clicked is transmitted to the board library which checks the board for a win condition. When a game is won, a message displaying the winner is shown, but allows for further pieces to be placed. 
On the left-hand side of the window, there is a functioning button to allow the game to be reset. Beneath this button there is another button to toggle 
between playing against or a human or against the AI. When the AI option is shown another button appears asking whether the AI should play as Order or Chaos. 


## Things to do 
* Integrate the AI with the GUI 
* Parallelize the search for a move
* Some refactoring/cleanup
* Improve documentation

## Difficulties encountered with Conrod
On at least MacOS the game must be tested and run in release mode as the backend of conrod seems to crash in debug mode. 
Additionally clippy must be disabled for Travis CI as conrod or its dependencies have issues that clippy considers to be error conditions. 
Running the compiled binary directly instead of using cargo build --release each time gives more consistent behavior with the GUI. 

One major design decision that arose out of how conrod works is how widget labels (what is shown on screen) work. 
This means that once created it seems a widget's label function cannot be called again. To dynamically change the content of a widget, 
the label of the widget needs to be given a &str. To change what is shown the value of this &str is changed. This is implemented with a 
separate struct to provide the backing memory and a setting function is used to update the value of the field, then the next time the 
window is drawn the new value of the label will be used. 

## Goals for Presentation on Tuesday
* Integrate the AI into the GUI 
* At least some parallel search