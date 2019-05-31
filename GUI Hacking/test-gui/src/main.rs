
#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
#[macro_use] extern crate conrod_winit;
extern crate find_folder;
extern crate glium;

mod support;

use glium::Surface;
use std::string::{ToString};
use std::borrow::{ToOwned};

widget_ids! {
    struct Ids {
        canvas,
        button_matrix,
        title,
        button,
        curr_piece,
        toggle,
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
enum CurrentTurn {
    Order,
    Chaos,
}

impl<'a> CurrentTurn {
    fn display(&self) -> &'a str {
        match self {
            CurrentTurn::Order => "Order",//.to_owned(),
            CurrentTurn::Chaos => "Chaos",//.to_owned(),
        }
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
    title_pad: f64,
    turn: CurrentTurn,
    current_piece: BoardState,
    piece_label: &'a str,
    piece_matrix: [&'a str; ROWS*COLS],
    opponent: Opponent,
    ai_opponent: CurrentTurn,
}

impl<'a> BoardGUI<'a> {
    fn new() -> Self {
        BoardGUI {
            title_pad: 475.0,
            turn: CurrentTurn::Order,
            current_piece: BoardState::X,
            piece_label: BoardState::X.display(),
            piece_matrix: [BoardState::Empty.display(); ROWS*COLS],
            opponent: Opponent::Human,
            ai_opponent: CurrentTurn::Chaos,
        }
    }

    fn reset(&mut self) {
        self.piece_matrix = [BoardState::Empty.display(); ROWS*COLS];
        self.turn = CurrentTurn::Order;
    }
}

fn main() {
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
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
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
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
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
            set_widgets(&mut ui, &mut app, &mut ids);
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


// In conrod, each widget must have its own unique identifier so that the `Ui` can keep track of
// its state between updates.
//
// To make this easier, conrod provides the `widget_ids` macro. This macro generates a new type
// with a unique `widget::Id` field for each identifier given in the list. See the `widget_ids!`
// documentation for more details.


const ROWS: usize = 6;
const COLS: usize = 6;

/// Set all `Widget`s within the User Interface.
///
/// The first time this gets called, each `Widget`'s `State` will be initialised and cached within
/// the `Ui` at their given indices. Every other time this get called, the `Widget`s will avoid any
/// allocations by updating the pre-existing cached state. A new graphical `Element` is only
/// retrieved from a `Widget` in the case that it's `State` has changed in some way.
fn set_widgets(ui: &mut conrod_core::UiCell, app: &mut BoardGUI, ids: &mut Ids) {
    use conrod_core::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .border(0.)
        .pad(30.0)
        .color(conrod_core::color::WHITE)
        .scroll_kids()
        .set(ids.canvas, ui);


    widget::Text::new("Order and Chaos")
        .top_left_with_margins_on(ids.canvas, 0.0, app.title_pad)
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
            .label(&app.piece_matrix[r * COLS + c])
            .label_font_size(50);

        for _click in elem.set(button, ui) {
            if app.piece_matrix[r * COLS + c] == BoardState::Empty.display() {
                app.piece_matrix[r * COLS + c] = app.current_piece.display();
            }

            if app.turn == CurrentTurn::Order {
                app.turn = CurrentTurn::Chaos;
            } else {
                app.turn = CurrentTurn::Order;
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
        .label(&app.piece_label)
        .label_font_size(50)
        .set(ids.button, ui);

    if button.was_clicked() {
        if app.current_piece == BoardState::X {
            app.current_piece = BoardState::O;
        } else {
            app.current_piece = BoardState::X;
        }
        app.piece_label = app.current_piece.display();
    }

    let mut current_turn = "";//.to_string();
    if app.turn == CurrentTurn::Order {
        current_turn = "Order's Turn   ";//.to_string();
    } else {
        current_turn = "Chaos' Turn   ";//.to_string();
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
        app.reset();
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
        .label(&app.opponent.display())
        .label_font_size(16)
        .set(ids.toggle, ui);

    if opponent_toggle.was_clicked() {
        if app.opponent == Opponent::Human {
            app.opponent = Opponent::AI;
        } else {
            app.opponent = Opponent::Human;
        }
    }

    if app.opponent == Opponent::AI {
        let opponent_ai_toggle = widget::Button::new()
            .w_h(100.0, 25.0)
            .down_from(ids.reset_button, 70.)
            .color(color::WHITE)
            .border(1.)
            .label(&app.ai_opponent.display())
            .label_font_size(16)
            .set(ids.ai_toggle, ui);

        if opponent_ai_toggle.was_clicked() {
            if app.ai_opponent == CurrentTurn::Order {
                app.ai_opponent = CurrentTurn::Chaos;
            } else {
                app.ai_opponent = CurrentTurn::Order;
            }
        }
    }

    widget::Text::new("Order Wins!")
        .align_bottom_of(ids.canvas)
        .right(350.)
        .font_size(50)
        .set(ids.winner, ui);
}