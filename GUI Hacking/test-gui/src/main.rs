
#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
#[macro_use] extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
//extern crate rand; // for making a random color.

mod support;

use glium::Surface;
use std::string::{ToString};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum CurrentTurn {
    Order,
    Chaos
}

struct BoardGUI {
    title_pad: f64,
    turn: CurrentTurn,
    current_piece: bool,
    piece_label: String,
//    piece_matrix: [[String; 6]; 6],
}

impl BoardGUI {
    fn new() -> Self {
        BoardGUI {
            title_pad: 475.0,
            turn: CurrentTurn::Order,
            current_piece: false,
            piece_label: "X".to_string(),
//            piece_matrix:
        }
    }

    fn reset(&mut self) {

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
widget_ids! {
    struct Ids {
        canvas,
        button_matrix,
        title,
        button,
        curr_piece,
        toggle,
        border_width,
        reset_button,
        curr_turn,

    }
}

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


    // We can use this `Canvas` as a parent Widget upon which we can place other widgets.
    widget::Canvas::new()
        .border(0.)
        .pad(30.0)
        .color(conrod_core::color::WHITE)
        .scroll_kids()
        .set(ids.canvas, ui);


    // Text example.
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
        let board_button = widget::Button::new().color(color::WHITE);
//        if board_button.was_clicked() {
//            board_button.label(&app.piece_label).label_font_size(32);
//        }
        for _click in elem.set(board_button, ui) {
            println!("Hey! {:?}", (r, c));
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
        if app.current_piece {
            app.piece_label = "X".to_string();
        } else {
            app.piece_label = "O".to_string();
        }
        app.current_piece = !app.current_piece;
    }

    let mut current_turn = "".to_string();
    if app.turn == CurrentTurn::Order {
        current_turn = "Order's Turn   ".to_string();
    } else {
        current_turn = "Chaos' Turn   ".to_string();
    }

    widget::Text::new(&current_turn)
        .mid_right()
        .up_from(ids.curr_piece, 20.)
        .font_size(16)
        .set(ids.curr_turn, ui);



    let new_game = widget::Button::new()
        .w_h(100.0, 25.0)
        .bottom_right()
//        .down_from(ids.button, 100.)
//        .right_from(ids.button, -100.0)
        .color(color::WHITE)
        .border(1.)
        .label("New game")
        .label_font_size(16)
        .set(ids.reset_button, ui);

    if new_game.was_clicked() {
        app.reset();
    }

    // A demonstration using a DropDownList to select its own color.
//    for selected_idx in widget::DropDownList::new(&app.ddl_colors, app.selected_idx)
//        .w_h(150.0, 40.0)
//        .right_from(ids.slider_height, 30.0) // Position right from widget 6 by 50 pixels.
//        .max_visible_items(3)
//        .color(app.ddl_color)
//        .border(app.border_width)
//        .border_color(app.ddl_color.plain_contrast())
//        .label("Colors")
//        .label_color(app.ddl_color.plain_contrast())
//        .scrollbar_on_top()
//        .set(ids.color_select, ui)
//        {
//            app.selected_idx = Some(selected_idx);
//            app.ddl_color = match &app.ddl_colors[selected_idx][..] {
//                "Black" => color::BLACK,
//                "White" => color::WHITE,
//                "Red"   => color::RED,
//                "Green" => color::GREEN,
//                "Blue"  => color::BLUE,
//                _       => color::PURPLE,
//            }
//        }




//

//    while let Some(elem) = elements.next(ui) {
//        let (col, row) = (elem.col, elem.row);
//
//        // Color effect for fun.
//        let (r, g, b, a) = (
//            0.5 + (elem.col as f32 / cols as f32) / 2.0,
//            0.75,
//            1.0 - (elem.row as f32 / rows as f32) / 2.0,
//            1.0
//        );
//
//        // We can use `Element`s to instantiate any kind of widget we like.
//        // The `Element` does all of the positioning and sizing work for us.
//        // Here, we use the `Element` to `set` a `Toggle` widget for us.
//        let toggle = widget::Toggle::new(app.bool_matrix[col][row])
//            .rgba(r, g, b, a)
//            .border(2.);
//        if let Some(new_value) = elem.set(toggle, ui).last() {
//            app.bool_matrix[col][row] = new_value;
//        }
//    }


    // Toggle widget example.
//    if let Some(value) = widget::Toggle::new(app.current_piece)
//        .w_h(75.0, 75.0)
//        .down(50.)
//        .right(375.)
////        .color(color::WHITE)
//        .rgba(0.5, 0.5, 0.5, 0.99)
//        .border(1.)
//        .label(&app.piece_label)
////        .label_color(color::WHITE.)
//        .label_font_size(32)
//        .set(ids.toggle, ui)
//        .last()
//    {
//        app.current_piece = !app.current_piece;
//        app.piece_label = match value {
//            true => "X".to_string(),
//            false => "O".to_string()
//        }
//    }
}
//
//=
//    // Number Dialer widget example. (value, min, max, precision)
//    for new_height in widget::NumberDialer::new(app.v_slider_height, 25.0, 250.0, 1)
//        .w_h(260.0, 60.0)
//        .right_from(shown_widget, 30.0)
//        .color(app.bg_color.invert())
//        .border(app.border_width)
//        .label("Height (px)")
//        .label_color(app.bg_color.invert().plain_contrast())
//        .set(ids.slider_height, ui)
//        {
//            app.v_slider_height = new_height;
//        }
//
//    // Number Dialer widget example. (value, min, max, precision)
//    for new_width in widget::NumberDialer::new(app.border_width, 0.0, 15.0, 2)
//        .w_h(260.0, 60.0)
//        .down(20.0)
//        .color(app.bg_color.plain_contrast().invert())
//        .border(app.border_width)
//        .border_color(app.bg_color.plain_contrast())
//        .label("Border Width (px)")
//        .label_color(app.bg_color.plain_contrast())
//        .set(ids.border_width, ui)
//        {
//            app.border_width = new_width;
//        }
//
//    // A demonstration using widget_matrix to easily draw a matrix of any kind of widget.
//    let (cols, rows) = (8, 8);
//    let mut elements = widget::Matrix::new(cols, rows)
//        .down(20.0)
//        .w_h(260.0, 260.0)
//        .set(ids.toggle_matrix, ui);
//
//    // The `Matrix` widget returns an `Elements`, which can be used similar to an `Iterator`.

//
//    // A demonstration using a DropDownList to select its own color.
//    for selected_idx in widget::DropDownList::new(&app.ddl_colors, app.selected_idx)
//        .w_h(150.0, 40.0)
//        .right_from(ids.slider_height, 30.0) // Position right from widget 6 by 50 pixels.
//        .max_visible_items(3)
//        .color(app.ddl_color)
//        .border(app.border_width)
//        .border_color(app.ddl_color.plain_contrast())
//        .label("Colors")
//        .label_color(app.ddl_color.plain_contrast())
//        .scrollbar_on_top()
//        .set(ids.color_select, ui)
//        {
//            app.selected_idx = Some(selected_idx);
//            app.ddl_color = match &app.ddl_colors[selected_idx][..] {
//                "Black" => color::BLACK,
//                "White" => color::WHITE,
//                "Red"   => color::RED,
//                "Green" => color::GREEN,
//                "Blue"  => color::BLUE,
//                _       => color::PURPLE,
//            }
//        }
//
//    // Draw an xy_pad.
//    for (x, y) in widget::XYPad::new(app.circle_pos[0], -75.0, 75.0, // x range.
//                                     app.circle_pos[1], 95.0, 245.0) // y range.
//        .w_h(150.0, 150.0)
//        .right_from(ids.toggle_matrix, 30.0)
//        .align_bottom_of(ids.toggle_matrix) // Align to the bottom of the last toggle_matrix element.
//        .color(app.ddl_color)
//        .border(app.border_width)
//        .border_color(color::WHITE)
//        .label("Circle Position")
//        .label_color(app.ddl_color.plain_contrast().alpha(0.5))
//        .line_thickness(2.0)
//        .set(ids.circle_position, ui)
//        {
//            app.circle_pos[0] = x;
//            app.circle_pos[1] = y;
//        }
//
//    // Draw a circle at the app's circle_pos.
//    widget::Circle::fill(15.0)
//        .xy_relative_to(ids.circle_position, app.circle_pos)
//        .color(app.ddl_color)
//        .set(ids.circle, ui);
//
//    // Draw two TextBox and EnvelopeEditor pairs to the right of the DropDownList flowing downward.
//    for i in 0..2 {
//        let &mut (ref mut env, ref mut text) = &mut app.envelopes[i];
//        let (text_box, env_editor, env_y_max, env_skew_y) = match i {
//            0 => (ids.text_box_a, ids.envelope_editor_a, 20_000.0, 3.0),
//            1 => (ids.text_box_b, ids.envelope_editor_b, 1.0, 1.0),
//            _ => unreachable!(),
//        };
//
//        // A text box in which we can mutate a single line of text, and trigger reactions via the
//        // `Enter`/`Return` key.
//        for event in widget::TextBox::new(text)
//            .and_if(i == 0, |text| text.right_from(ids.color_select, 30.0))
//            .font_size(20)
//            .w_h(320.0, 40.0)
//            .border(app.border_width)
//            .border_color(app.bg_color.invert().plain_contrast())
//            .color(app.bg_color.invert())
//            .set(text_box, ui)
//            {
//                match event {
//                    widget::text_box::Event::Enter => println!("TextBox {}: {:?}", i, text),
//                    widget::text_box::Event::Update(string) => *text = string,
//                }
//            }
//
//        // Draw an EnvelopeEditor. (&[Point], x_min, x_max, y_min, y_max).
//        for event in widget::EnvelopeEditor::new(env, 0.0, 1.0, 0.0, env_y_max)
//            .down(10.0)
//            .w_h(320.0, 150.0)
//            .skew_y(env_skew_y)
//            .color(app.bg_color.invert())
//            .border(app.border_width)
//            .border_color(app.bg_color.invert().plain_contrast())
//            .label(&text)
//            .label_color(app.bg_color.invert().plain_contrast().alpha(0.5))
//            .point_radius(6.0)
//            .line_thickness(2.0)
//            .set(env_editor, ui)
//            {
//                event.update(env);
//            }
//    }
//}
//////! A simple demonstration of how to construct and use Canvasses by splitting up the window.
////
////#[macro_use] extern crate conrod_core;
////extern crate conrod_glium;
////#[macro_use] extern crate conrod_winit;
////extern crate find_folder;
////extern crate glium;
////extern crate image;
////
////use glium::Surface;
////
////mod support;
////
////fn main() {
////    const WIDTH: u32 = 1000;
////    const HEIGHT: u32 = 1000;
////
////    // Build the window.
////    let mut events_loop = glium::glutin::EventsLoop::new();
////    let window = glium::glutin::WindowBuilder::new()
////        .with_title("Canvas")
////        .with_dimensions((WIDTH, HEIGHT).into());
////    let context = glium::glutin::ContextBuilder::new()
////        .with_vsync(true)
////        .with_multisampling(4);
////    let display = glium::Display::new(window, context, &events_loop).unwrap();
////    let display = support::GliumDisplayWinitWrapper(display);
////
////    // construct our `Ui`.
////    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
////
////    // Add a `Font` to the `Ui`'s `font::Map` from file.
////    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
////    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
////    ui.fonts.insert_from_file(font_path).unwrap();
////
////    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
////    // for drawing to the glium `Surface`.
////    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();
////
////    // The image map describing each of our widget->image mappings (in our case, none).
////    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();
////
////    // Instantiate the generated list of widget identifiers.
////    let ids = &mut Ids::new(ui.widget_id_generator());
////
////    // Poll events from the window.
////    let mut event_loop = support::EventLoop::new();
////    'main: loop {
////
////        // Handle all events.
////        for event in event_loop.next(&mut events_loop) {
////
////            // Use the `winit` backend feature to convert the winit event to a conrod one.
////            if let Some(event) = support::convert_event(event.clone(), &display) {
////                ui.handle_event(event);
////                event_loop.needs_update();
////            }
////
////            match event {
////                glium::glutin::Event::WindowEvent { event, .. } => match event {
////                    // Break from the loop upon `Escape`.
////                    glium::glutin::WindowEvent::CloseRequested |
////                    glium::glutin::WindowEvent::KeyboardInput {
////                        input: glium::glutin::KeyboardInput {
////                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
////                            ..
////                        },
////                        ..
////                    } => break 'main,
////                    _ => (),
////                },
////                _ => (),
////            }
////        }
////
////        // Instantiate all widgets in the GUI.
////        set_widgets(ui.set_widgets(), ids);
////
////        // Render the `Ui` and then display it on the screen.
////        if let Some(primitives) = ui.draw_if_changed() {
////            renderer.fill(&display.0, primitives, &image_map);
////            let mut target = display.0.draw();
////            target.clear_color(0.0, 0.0, 0.0, 1.0);
////            renderer.draw(&display.0, &mut target, &image_map).unwrap();
////            target.finish().unwrap();
////        }
////    }
////}
////
////// Draw the Ui.
////fn set_widgets(ref mut ui: conrod_core::UiCell, ids: &mut Ids) {
////    use conrod_core::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
////
////    // Construct our main `Canvas` tree.
////    widget::Canvas::new().flow_down(&[
////        (ids.body, widget::Canvas::new().length(1000.0).flow_right(&[
////            (ids.middle_column, widget::Canvas::new().color(color::WHITE)),
////        ])),
////    ]).set(ids.master, ui);
////
////    let footer_wh = ui.wh_of(ids.middle_column).unwrap();
////    let mut elements = widget::Matrix::new(COLS, ROWS)
////        .w_h(footer_wh[0], footer_wh[1])
////        .mid_top_of(ids.middle_column)
////        .set(ids.button_matrix, ui);
////    while let Some(elem) = elements.next(ui) {
////        let (r, c) = (elem.row, elem.col);
////        let button = widget::Button::new().color(color::WHITE);
////        for _click in elem.set(button, ui) {
////            println!("Hey! {:?}", (r, c));
////        }
////    }
////
////
////}
////
////
////// Button matrix dimensions.

////
////// Generate a unique `WidgetId` for each widget.
////widget_ids! {
////    struct Ids {
////        master,
////        header,
////        body,
////        left_column,
////        middle_column,
////        right_column,
////        footer,
////        footer_scrollbar,
////        floating_a,
////        floating_b,
////        tabs,
////        tab_foo,
////        tab_bar,
////        tab_baz,
////
////        title,
////        subtitle,
////        top_left,
////        bottom_right,
////        foo_label,
////        bar_label,
////        baz_label,
////        button_matrix,
////        bing,
////        bong,
////    }
////}
////
////
////
