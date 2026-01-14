use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};

use crate::utils::clone_extension_template;
mod utils;

fn main() {
    // Check for git BEFORE initializing the terminal
    if !utils::is_git_installed() {
        eprintln!("Error: Git is not installed or not in PATH");
        eprintln!(
            "Please install Git to use this application. For details see: https://git-scm.com/install/"
        );
        std::process::exit(1);
    }

    println!("✓ Git is installed and ready!");

    if let Some(proj_name) = std::env::args().nth(1) {
        println!("Project name: {}", &proj_name);

        clone_extension_template(proj_name).unwrap();

        return;
    }

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
