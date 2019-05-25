## Current State of the project
Order and Chaos is playable via the command line between two human players. The library for the board should be fairly stable at this point, but more optimizations for checking win conditions could be added. For instance, if Order cannot win, the game should recognize that Chaos wins before the entire board is filled with pieces. The number of Cells checked a move could also be decreased as the game goes on if the get_status function "remembers" that Order cannot win in a certain
direction.  

## Playing the game
Right now, no computer player has been implemented. Two humans can play against each other by using cargo run. Order goes first, then the player inputs the piece (x or o) followed by the coordinates, row then column, of the location they wish to play at. This input scheme is very temporary, so there is almost no error-checking at the moment

## GUI Plans
We intend to replace this interface with a GUI using the conrod crate. We spent some time looking at several crates for building GUIs in Rust. From what we looked at, there wasn't a clear winner, so we picked one that was simple to get working and had examples of similar features to what we want to implement. 

There is a separate directory called GUI Hacking with another Rust project that contains a proof of concept 6x6 clickable grid adapted from a conrod example. It may only run correctly in release mode, we're not totally sure why at the moment. It currently prints the clicked coordinates to the console, which would easily support the interface to the board module. The documentation for the crate is relatively limited, so we are still trying to figure out a good way to dynamically write the piece
to the button on the screen.

## Things to do 
* Get the GUI working
* Implement the computer player
    * Parallelize the search


## Stretch goal
In the proposal we had thought about the idea of allowing two humans to play against each other remotely. We may still have time to look into this, but we feel the computer player and GUI should be higher priorities.  
