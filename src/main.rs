use project::board::{Game, GameStatus, Move, MoveType};
use project::strategy::Player;

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
#[macro_use]
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;

mod support;

use glium::Surface;

const ROWS: usize = 6;
const COLS: usize = 6;

///Each conrod widget needs a unique identifier to track state for each update. The widgets_ids! macro
/// defines these ids
widget_ids! {
    struct Ids {
        canvas,
        button_matrix,
        title,
        button,
        curr_piece,
        piece_toggle,
        ai_toggle,
        border_width,
        reset_button,
        curr_turn,
        opponent,
        against,
        winner,
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum BoardState {
    Empty,
    X,
    O,
}

impl<'a> BoardState {
    pub fn display(&self) -> &'a str {
        match self {
            BoardState::Empty => "",
            BoardState::X => "X",
            BoardState::O => "O",
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Opponent {
    Human,
    AI,
}

impl<'a> Opponent {
    pub fn display(&self) -> &'a str {
        match self {
            Opponent::AI => "AI",
            Opponent::Human => "Human",
        }
    }
}

struct BoardGUI<'a> {
    turn: Player,
    current_piece: BoardState,
    piece_label: &'a str,
    piece_matrix: [&'a str; ROWS * COLS],
    opponent: Opponent,
    ai_opponent: Player,
}

impl<'a> BoardGUI<'a> {
    fn new() -> Self {
        BoardGUI {
            turn: Player::Order,
            current_piece: BoardState::X,
            piece_label: BoardState::X.display(),
            piece_matrix: [BoardState::Empty.display(); ROWS * COLS],
            opponent: Opponent::Human,
            ai_opponent: Player::Chaos,
        }
    }

    ///Get the current player
    pub fn turn(&self) -> Player {
        self.turn
    }

    ///Change the current player
    pub fn set_turn(&mut self, turn: Player) {
        self.turn = turn
    }

    ///Get the current piece
    pub fn current_piece(&self) -> BoardState {
        self.current_piece
    }

    ///Change the current piece
    pub fn set_current_piece(&mut self, piece: BoardState) {
        self.current_piece = piece
    }

    ///Get the label to display for the piece
    pub fn piece_label(&self) -> &'a str {
        self.piece_label
    }

    ///Set the piece label
    pub fn set_piece_label(&mut self, label: &'a str) {
        self.piece_label = label
    }

    ///The matrix representing the board
    pub fn piece_matrix(&self) -> [&'a str; ROWS * COLS] {
        self.piece_matrix
    }

    ///Set a piece in a specific location in the board
    pub fn set_piece(&mut self, row: usize, col: usize, piece: &'a str, game: &Game) -> Game {
        let cell_type = if piece == "X" {
            MoveType::X
        } else {
            MoveType::O
        };
        match game.make_move(Move::new(cell_type, row, col)) {
            Some(new_game) => {
                self.piece_matrix[row * COLS + col] = piece;
                return new_game;
            }
            None => game.clone(),
        }
    }

    ///Get the opponent
    pub fn opponent(&self) -> Opponent {
        self.opponent
    }

    ///Set the opponent
    pub fn set_opponent(&mut self, opponent: Opponent) {
        self.opponent = opponent
    }

    ///When the opponent is the AI, get the type of player for the AI
    pub fn ai_opponent(&self) -> Player {
        self.ai_opponent
    }

    ///Set the type of AI opponent
    pub fn set_ai_opponent(&mut self, ai_opponent: Player) {
        self.ai_opponent = ai_opponent
    }

    ///Create a new GUI game. Resets the internal state of the library.
    pub fn reset(&mut self, mut game: Game) -> Game {
        self.piece_matrix = [BoardState::Empty.display(); ROWS * COLS];
        self.turn = Player::Order;
        game.reset()
    }
}

///With modifications to adjust to the purposes of our game, the main function is the
///same as given in conrod example programs.
fn main() {
    let mut game = Game::new();
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 1000;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Widget Demonstration")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Identifiers used for instantiating our widgets.
    let mut ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // Our demonstration app that we'll control with our GUI.
    let mut app = BoardGUI::new();

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // We'll set all our widgets in a single function called `set_widgets`.
        {
            let mut ui = ui.set_widgets();
            game = set_widgets(&mut ui, &mut app, &mut ids, game);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

///Helper function to compute index into linearized 2d array
fn flat_index(row: usize, col: usize) -> usize {
    row * COLS + col
}

/// Set all `Widget`s within the User Interface.
///
/// The first time this gets called, each `Widget`'s `State` will be initialised and cached within
/// the `Ui` at their given indices. Every other time this get called, the `Widget`s will avoid any
/// allocations by updating the pre-existing cached state. A new graphical `Element` is only
/// retrieved from a `Widget` in the case that it's `State` has changed in some way.
fn set_widgets(
    ui: &mut conrod_core::UiCell,
    app: &mut BoardGUI,
    ids: &mut Ids,
    game: Game,
) -> Game {
    use conrod_core::{
        color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
    };
    let mut new_game_board = game.clone();

    widget::Canvas::new()
        .border(0.)
        .pad(30.0)
        .color(conrod_core::color::WHITE)
        .scroll_kids()
        .set(ids.canvas, ui);

    widget::Text::new("Order and Chaos")
        .top_left_with_margins_on(ids.canvas, 0.0, 475.)
        .font_size(32)
        .set(ids.title, ui);

    let mut elements = widget::Matrix::new(COLS, ROWS)
        .w_h(800., 800.)
        .align_middle_x()
        .set(ids.button_matrix, ui);
    while let Some(elem) = elements.next(ui) {
        let (r, c) = (elem.row, elem.col);
        let button = widget::Button::new()
            .color(color::WHITE)
            .label(&app.piece_matrix()[flat_index(r, c)])
            .label_font_size(50);

        for _click in elem.set(button, ui) {
            if app.piece_matrix()[flat_index(r, c)] == BoardState::Empty.display() {
                new_game_board = app.set_piece(r, c, app.current_piece().display(), &game);
            }

            if app.turn() == Player::Order {
                app.set_turn(Player::Chaos);
            } else {
                app.set_turn(Player::Order);
            }
        }
    }

    widget::Text::new("Current piece is:")
        .mid_right()
        .font_size(16)
        .set(ids.curr_piece, ui);

    let button = widget::Button::new()
        .w_h(75.0, 75.0)
        .down_from(ids.curr_piece, 20.)
        .right_from(ids.curr_piece, -100.0)
        .color(color::WHITE)
        .border(1.)
        .label(&app.piece_label())
        .label_font_size(50)
        .set(ids.button, ui);

    if button.was_clicked() {
        if app.current_piece() == BoardState::X {
            app.set_current_piece(BoardState::O);
        } else {
            app.set_current_piece(BoardState::X);
        }
        app.set_piece_label(app.current_piece().display());
    }

    let current_turn;
    if app.turn() == Player::Order {
        current_turn = "Order's Turn   ";
    } else {
        current_turn = "Chaos' Turn   ";
    }

    widget::Text::new(current_turn)
        .mid_right()
        .up_from(ids.curr_piece, 20.)
        .font_size(16)
        .set(ids.curr_turn, ui);

    let new_game = widget::Button::new()
        .w_h(100.0, 25.0)
        .mid_left()
        .color(color::WHITE)
        .border(1.)
        .label("New game")
        .label_font_size(16)
        .set(ids.reset_button, ui);

    if new_game.was_clicked() {
        new_game_board = app.reset(new_game_board);
    }

    widget::Text::new("     Against:")
        .down_from(ids.reset_button, 10.)
        .font_size(16)
        .set(ids.against, ui);

    let opponent_toggle = widget::Button::new()
        .w_h(100.0, 25.0)
        .down_from(ids.reset_button, 40.)
        .color(color::WHITE)
        .border(1.)
        .label(&app.opponent().display())
        .label_font_size(16)
        .set(ids.piece_toggle, ui);

    if opponent_toggle.was_clicked() {
        if app.opponent() == Opponent::Human {
            app.set_opponent(Opponent::AI);
        } else {
            app.set_opponent(Opponent::Human);
        }
    }

    if app.opponent() == Opponent::AI {
        let opponent_ai_toggle = widget::Button::new()
            .w_h(100.0, 25.0)
            .down_from(ids.reset_button, 70.)
            .color(color::WHITE)
            .border(1.)
            .label(&app.ai_opponent().display())
            .label_font_size(16)
            .set(ids.ai_toggle, ui);

        if opponent_ai_toggle.was_clicked() {
            if app.ai_opponent() == Player::Order {
                app.set_ai_opponent(Player::Chaos);
            } else {
                app.set_ai_opponent(Player::Order);
            }
        }
    }

    if game.get_status() == GameStatus::ChaosWins {
        widget::Text::new("Chaos Wins!")
            .align_bottom_of(ids.canvas)
            .right(350.)
            .font_size(50)
            .set(ids.winner, ui);
    } else if game.get_status() == GameStatus::OrderWins {
        widget::Text::new("Order Wins!")
            .align_bottom_of(ids.canvas)
            .right(350.)
            .font_size(50)
            .set(ids.winner, ui);
    }

    new_game_board
}
