use serde::{Deserialize, Serialize};
use serde_json::value::Index;
use std::{alloc::handle_alloc_error, fs, io::{self, Write}};

#[derive(Debug, Clone, Serialize, Deserialize)]struct Habit{
    name: String,
    streak: u32,
}

impl Habit{
    fn new(name: String)-> Self{
        Habit{name,streak:0}
    }
    fn complete(&mut self){
        self.streak+=1;
    }
    fn reset(&mut self){
        self.streak =0;
    }
}

fn save_habits(habits: &[Habit]) -> Result<(), io::Error> {
    let json_payload = serde_json::to_string_pretty(habits).map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
    fs::write("habits.json", json_payload)?;
    Ok(())
}

fn load_habits() -> Result<Vec<Habit>, io::Error> {
    let data = fs::read_to_string("habits.json")?;
    serde_json::from_str(&data).map_err(|e|io::Error::new(io::ErrorKind::Other, e))
}
fn is_valid_habit_name(name: &str) -> bool{
    if name.is_empty() {
        return false;
    }
    
    if name.contains(' ') {
        return false;
    }
    name.chars().all(|c| c.is_lowercase() || c.is_numeric() || c == '-')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

fn find_habit_by_name(name: &str, habits: &[Habit]) -> Option<usize>{
    habits.iter().position(|e|e.name == name)
}

fn main() {
    println!("🦀 Habit Tracker CLI\n");
    let mut habits: Vec<Habit> = match load_habits() {
        Ok(h) => h,
        Err(_) => vec![],
    };
    loop{
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = *parts.get(0).unwrap_or(&"");
        let args = if parts.len()>1{&parts[1..]} else {&[]};
        match command {
            "quit" | "q" => {
                match save_habits(&habits){
                    Ok(_) => {println!("auto saving progress, 👋 Goodbye!"); break},
                    Err(_) => {println!("👋 Goodbye!"); break}
                }
            }
            
            "help" | "h" => {
                println!("\n📋 Available Commands:");
                println!("  add <name>      - Add a new habit");
                println!("  list            - Show all habits");
                println!("  save            - Saves progress");
                println!("  view <name>     - Show specific habit");
                println!("  complete <name> - Increment habit streak");
                println!("  reset <name>    - Reset habit to 0");
                println!("  delete <name>   - Remove habit");
                println!("  stats           - Show statistics");
                println!("  save            - Save to file");
                println!("  quit            - Exit\n");
            }
            
            "" => continue,
            "list" | "l" => {
                if habits.is_empty(){println!("No habits yet! Use 'add <name>' to create one.")}
                else{
                    println!("\n Your habits:");
                    for (i, habit) in habits.iter().enumerate(){
                        let fire = if habit.streak > 0 { "🔥" } else { "" };
                        println!("  {}. {} - {} days {}", i + 1, habit.name, habit.streak, fire);

                    }
                    println!();
                }
            }
            "add" | "a" => {
                
                if args.is_empty(){println!("To use add, try: add <habit-name>")}
                else if args.len()>1{
                    println!("❌ Habit name cannot contain spaces");
                    println!("   Did you mean: {}?", args.join("-"));
                }
                else{
                    let habit_name = args[0]; 

                    if !is_valid_habit_name(&habit_name){println!("Habits should be kebab-case")}
                    
                    else if find_habit_by_name(habit_name, &habits).is_some() {
                        println!("❌ Habit '{}' already exists!", habit_name);
                    }
                    else{
                        habits.push(Habit::new(habit_name.to_string()));
                        println!("Habit {} successfully added", habit_name)
                    }
                }
            }
            "save" | "s" =>{
                match save_habits(&habits){
                    Ok(_) => {println!("Saved progress")},
                    Err(e) => {println!("Error saving to file: {}",e)}
                }
            }
            "complete" | "c" =>{
                if args.is_empty(){println!("❌ Usage: complete <habit-name>")}
                else if args.len()>1{
                    println!("❌ Habit name cannot contain spaces");
                    println!("   Did you mean: {}?", args.join("-"));
                }
                else{
                    let habit_name = args[0];
                    match find_habit_by_name(habit_name, &habits){
                        Some(index) =>{
                            habits[index].complete();
                            let new_streak = habits[index].streak;
                            println!("Great job! You upped your streak from {} to {}", new_streak-1, new_streak);
                        }
                        None => println!("Habit name {} not found", habit_name)
                    }
                }
            }
            "view" | "v" =>{
                if args.is_empty(){println!("❌ Usage: view <habit-name>")}
                else if args.len()>1{
                    println!("❌ Habit name cannot contain spaces");
                    println!("   Did you mean: {}?", args.join("-"));
                }
                else{
                    let habit_name = args[0];
                    match find_habit_by_name(habit_name, &habits){
                        Some(index) =>{
                            println!("Current {} streak is {}🔥", habits[index].name,habits[index].streak )
                        }
                        None => println!("Habit name {} not found", habit_name)
                    }
                } 
            }
            "reset" | "r"=>{
                if args.is_empty(){println!("❌ Usage: reset <habit-name>")}
                else if args.len()>1{
                    println!("❌ Habit name cannot contain spaces");
                    println!("   Did you mean: {}?", args.join("-"));
                }
                else{
                    let habit_name = args[0];
                    match find_habit_by_name(habit_name, &habits){
                        Some(index) => {habits[index].reset();println!("Reset exercise {}", habits[index].name);}
                        None => println!("Habit not {} found", habit_name)
                    }
                }
            }
            "delete" | "d" => {
                if args.is_empty() {
                    println!("❌ Usage: delete <habit-name>");
                } 
                else if args.len()>1{
                    println!("❌ Habit name cannot contain spaces");
                    println!("   Did you mean: {}?", args.join("-"));
                }
                else {
                    let habit_name = args[0];
                    
                    match find_habit_by_name(habit_name, &habits) {
                        Some(index) => {
                            habits.remove(index);  // Vec method, not Habit method!
                            println!("🗑️  Deleted: {}", habit_name);
                        }
                        None => {
                            println!("❌ Habit '{}' not found", habit_name);
                        }
                    }
                }
            }
            "stats" => {
                if habits.is_empty() {
                    println!("📊 No habits to show stats for!");
                } else {
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
                    
                    println!("\n📊 Habit Statistics");
                    println!("━━━━━━━━━━━━━━━━━━━━");
                    println!("Total habits: {}", total);
                    println!("Active (streak > 0): {}", active);
                    println!("Longest streak: {} days", longest);
                    println!("Average streak: {:.1} days\n", average);
                }
            }
            _ => {
                println!("❌ Unknown command: '{}'", command);
                println!("💡 Type 'help' to see available commands");
            }
        }
    }
}
