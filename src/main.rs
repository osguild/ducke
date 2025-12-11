use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};
mod utils;

fn main() {
    // Check for git BEFORE initializing the terminal
    if !utils::is_git_installed() {
        eprintln!("Error: Git is not installed or not in PATH");
        eprintln!("Please install Git to use this application. For details see: https://git-scm.com/install/");
        std::process::exit(1);
    }

    println!("✓ Git is installed and ready!");
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello, World!");
    frame.render_widget(text, frame.area());
}