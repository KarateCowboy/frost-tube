use frost_tube::*;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Frost Tube")
        .theme(App::theme)
        .run()
}
