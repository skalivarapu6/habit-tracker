use std::io;

use crossterm::{
    ExecutableCommand, event::{self, Event, KeyCode}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};
use ratatui::{
    Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, widgets::{List, ListItem, Paragraph, Widget}  // Add these
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
    let chunks = Layout::default()
    .direction(Direction::Vertical)  // Stack vertically
    .constraints([
        Constraint::Length(1),   // Section 1: 1 row tall
        Constraint::Min(0),      // Section 2: Take remaining space
        Constraint::Length(1),   // Section 3: 1 row tall
        Constraint::Length(1),   // Section 4: 1 row tall
    ])
    .split(f.area()); 
    // Section 1 - Header
    let text = format!("HABIT TRACKER - {} habits loaded", habits.len());
    let paragraph = Paragraph::new(text).bold().on_white().centered();
    f.render_widget(paragraph, chunks[0]);

    // // Section 2 - Habit list
    let habit_list: Vec<ListItem> = habits.iter().map(|h| {
        // Create progress bar based on streak
        let max_bar_length = 10;
        let filled = if h.streak > 0 {
            std::cmp::min(h.streak as usize, max_bar_length)
        } else {
            0
        };
        
        let bar = if h.streak > 0 {
            "▓".repeat(filled) + &"░".repeat(max_bar_length - filled)
        } else {
            "░".repeat(max_bar_length)
        };
        
        let text = format!(" {}  {}  {}", h.name, bar, h.streak);
        ListItem::new(text)
    }).collect();
    let list = List::new(habit_list);
    f.render_widget(list, chunks[1]);
    
    // Section 3 - Stats
    let total = habits.len();
    let active = habits.iter()
        .filter(|h| h.streak > 0)
        .count();
    
    let longest = habits.iter()
        .map(|h| h.streak)
        .max()
        .unwrap_or(0);
    
    let total_days: u32 = habits.iter()
        .map(|h| h.streak)
        .sum();
    
    let average = if total > 0 {
        total_days as f64 / total as f64
    } else {
        0.0
    };
    let stats  = format!("Longest {longest} • Active Streaks {active} • Total Streaks {total_days} • Average {average}" );
    let paragraph_third = Paragraph::new(stats).bold().on_white().centered();
    f.render_widget(paragraph_third, chunks[2]);
}