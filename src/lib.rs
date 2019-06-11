///A Board defines a game of Order and Chaos. A user facing client application should utilize the
/// enums and structs declared publicly in this module. A game is considered to be an immutable
/// structure. Each player's move will result in a new game with the correct state.
///
/// To use this as a library, a client application will need to create an initial game object
/// and manage any relevant input or output data. The GameStatus enum will inform the client
/// whether or not the game has been won by any player. A formatter for println! is provided
/// within the game struct; however, any other UI for the game is the responsibility of the client.
mod board;
///Strategy contains the implementation details for an Order and Chaos AI player. As such,
/// there is no publicly available API components in the strategy module.
mod strategy;

pub use board::{Game, GameStatus, Move, MoveType, Player, Strategy};
