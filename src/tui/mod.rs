use std::{fmt::format, io, os::macos::raw::stat};

use crossterm::{
    ExecutableCommand, event::{self, Event, KeyCode}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};
use ratatui::{
    Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, widgets::{List, ListItem, Paragraph, Widget}  // Add these
};


// use crate::{habit::Habit, stats::HabitStats};
use crate::{habit::{Habit, HabitFunctions}, storage::load_habits};

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

// fn draw_ui(f: &mut ratatui::Frame, habits: &[Habit]) {
//     // For now, just draw a simple message
//     // We'll make it fancy later    
//     let chunks = Layout::default()
//     .direction(Direction::Vertical)  // Stack vertically
//     .constraints([
//         Constraint::Length(1),   // Section 1: 1 row tall
//         Constraint::Min(0),      // Section 2: Take remaining space
//         Constraint::Length(1),   // Section 3: 1 row tall
//         Constraint::Length(1),   // Section 4: 1 row tall
//     ])
//     .split(f.area()); 
//     // Section 1 - Header
//     let text = format!("HABIT TRACKER - {} habits loaded", habits.len());
//     let paragraph = Paragraph::new(text).bold().on_white().centered();
//     f.render_widget(paragraph, chunks[0]);

//     // // Section 2 - Habit list
//     let habit_list: Vec<ListItem> = habits.iter().map(|h| {
//         // Create progress bar based on streak
//         let max_bar_length = 10;
//         let filled = if h.streak > 0 {
//             std::cmp::min(h.streak as usize, max_bar_length)
//         } else {
//             0
//         };
        
//         let bar = if h.streak > 0 {
//             "▓".repeat(filled) + &"░".repeat(max_bar_length - filled)
//         } else {
//             "░".repeat(max_bar_length)
//         };
        
//         let text = format!(" {}  {}  {}", h.name, bar, h.streak);
//         ListItem::new(text)
//     }).collect();
//     let list = List::new(habit_list);
//     f.render_widget(list, chunks[1]);
    
//     // Section 3 - Stats
//     let stats = HabitStats::calculate(habits);
//     let display_stats = stats.display_stats();
//     // let stats  = format!("Longest {longest} • Active Streaks {active} • Total Streaks {total_days} • Average {average}" );
//     let paragraph_third = Paragraph::new(display_stats).bold().on_white().centered();
//     f.render_widget(paragraph_third, chunks[2]);
// }

fn draw_ui(f: &mut ratatui::Frame, habits: &[Habit]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("HABIT TRACKER")
        .bold()
        .centered();
    f.render_widget(header, chunks[0]);

    // Habit list (simplified - no progress bars yet)
    let items: Vec<ListItem> = habits.iter()
        .map(|h| ListItem::new(h.display_line()))
        .collect();
    
    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    // Stats (comment out for now)
    let stats_text = Paragraph::new("Stats temporarily disabled");
    f.render_widget(stats_text, chunks[2]);

    // Commands
    let commands = Paragraph::new(" [q]uit");
    f.render_widget(commands, chunks[3]);
}