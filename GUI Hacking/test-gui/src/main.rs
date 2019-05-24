//! A simple demonstration of how to construct and use Canvasses by splitting up the window.

#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
#[macro_use] extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

use glium::Surface;
//use conrod_core::Color;

mod support;

fn main() {
    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 1000;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Canvas")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

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

        // Instantiate all widgets in the GUI.
        set_widgets(ui.set_widgets(), ids);

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

// Draw the Ui.
fn set_widgets(ref mut ui: conrod_core::UiCell, ids: &mut Ids) {
    use conrod_core::{color, widget, Colorable, Positionable, Sizeable, Widget};

    // Construct our main `Canvas` tree.
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new().color(color::WHITE).pad_bottom(20.0)),
        (ids.body, widget::Canvas::new().length(1000.0).flow_right(&[
            (ids.middle_column, widget::Canvas::new().color(color::WHITE)),//.color(color::ORANGE)),
        ])),
    ]).set(ids.master, ui);


//    fn text(text: widget::Text) -> widget::Text { text.color(color::BLACK).font_size(36) }

    let footer_wh = ui.wh_of(ids.middle_column).unwrap();
    let mut elements = widget::Matrix::new(COLS, ROWS)
        .w_h(footer_wh[0], footer_wh[1])
        .mid_top_of(ids.middle_column)
        .set(ids.button_matrix, ui);

    while let Some(elem) = elements.next(ui) {
        let (r, c) = (elem.row, elem.col);
//        let n = c + r * c;
//        let luminance = n as f32 / (COLS * ROWS) as f32;
        let button = widget::Button::new().color(color::WHITE);

        for _click in elem.set(button, ui) {
            println!("Hey! {:?}", (r, c));
        }
    }

}


// Button matrix dimensions.
const ROWS: usize = 6;
const COLS: usize = 6;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        header,
        body,
        left_column,
        middle_column,
        right_column,
        footer,
        footer_scrollbar,
        floating_a,
        floating_b,
        tabs,
        tab_foo,
        tab_bar,
        tab_baz,

        title,
        subtitle,
        top_left,
        bottom_right,
        foo_label,
        bar_label,
        baz_label,
        button_matrix,
        bing,
    }
}
