use crate::game_core::GameCore;
use fltk::{app::*, button::*, frame::*, group::Pack, group::PackType, input::IntInput, window::*};

pub fn run() {
    let mut score = 0;
    let mut steep = 0;
    let mut round = 1;
    let mut core = GameCore::new();
    let app = App::default();
    let mut win = Window::new(100, 100, 400, 300, "Guess num");
    let mut btn = Button::new(160, 200, 80, 40, "Ok");
    let input = IntInput::new(160, 140, 80, 30, "");
    let mut pack = Pack::new(0, 0, 380, 100, "");
    let mut top = Pack::new(0, 0, 380, 100, "");
    top.set_type(PackType::Horizontal);
    let mut rounds = Frame::default().with_size(250, 100).with_label("Round: 1");
    let mut score_view = Frame::default().with_size(90, 100).with_label("Score: 0");
    top.end();
    let mut frame = Frame::default()
        .with_size(260, 40)
        .with_label("Enter 3-digits number");
    pack.set_type(PackType::Vertical);
    pack.end();

    core.generate();

    btn.set_callback(move || {
        if input.value().len() < 3 {
            frame.set_label_color(Color::Red);
            frame.set_label("The number must be a 3-digits number!");
        } else {
            frame.set_label_color(Color::Black);
            steep += 1;
            let val = input.value();
            core.set_user_answer(&val);
            if core.check_win() {
                score = score + 1000 / steep;
                round += 1;
                core.generate();
                frame.set_label("Eter 3-digits number");
                score_view.set_label(&format!("Score: {}", score));
                rounds.set_label(&format!("Round: {}", round));
            } else {
                frame.set_label(&format!(
                    "Matching digits: {}. Matching positions:{}",
                    core.eq_nums_count(),
                    core.eq_pos_count()
                ));
            }
            input.set_value("");
        }
    });

    win.end();
    win.show();
    app.run().unwrap();
}
