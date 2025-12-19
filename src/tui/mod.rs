use std::io;

use crossterm::{
    ExecutableCommand, event::{self, Event, KeyCode}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};
use ratatui::{
    Terminal, backend::CrosstermBackend
};


use crate::habit::Habit;
use crate::storage::load_habits;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Part 2: Terminal setup (your code)
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend =  CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Part 3: Load habits (your code)
    let mut habits: Vec<Habit> = match load_habits(){
        Ok(h) => h,
        Err(_) => vec![],
    };
    // Part 4: Event loop (your code)
    loop{
        terminal.draw(|f| {
            draw_ui(f, &habits);
        })?;        let event = event::read()?;
        match event {
            Event::Key(key) => {
                // Extract the KeyEvent
                match key.code {
                    KeyCode::Char('q') => break,
                    _=>()
                }
            }
            _=>()
        }
    }
    // Part 5: Cleanup (your code)
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;    
    Ok(())
}

fn draw_ui(f: &mut ratatui::Frame, habits: &[Habit]) {
    // For now, just draw a simple message
    // We'll make it fancy later
    
    use ratatui::widgets::Paragraph;
    
    let text = format!("HABIT TRACKER - {} habits loaded", habits.len());
    let paragraph = Paragraph::new(text);
    
    f.render_widget(paragraph, f.area());
}