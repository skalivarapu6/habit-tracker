use serde::{Deserialize, Serialize};
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
}

fn save_habits(habits: &[Habit]) -> Result<(), io::Error> {
    // TODO: 
    // 1. Call serde_json::to_string_pretty(habits)
    // 2. Use .map_err() to convert error
    // 3. Use ? to propagate error
    // 4. Write to file with fs::write()
    // 5. Use ? again
    // 6. Return Ok(())
    let json_payload = serde_json::to_string_pretty(habits).map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
    fs::write("habits.json", json_payload)?;
    Ok(())
}

fn load_habits() -> Result<Vec<Habit>, io::Error> {
    // TODO:
    // 1. Read file with fs::read_to_string("habits.json")
    // 2. Use ? to propagate error
    // 3. Parse with serde_json::from_str(&data)
    // 4. Use .map_err() and ? 
    // 5. Return the result (it's already Result<Vec<Habit>, ...>)
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
    // TODO: Find the index of the habit with matching name
    //
    // Hints:
    // - Use .iter() to iterate over habits
    // - Use .position() which returns Option<usize>
    // - Compare habit.name with name parameter
    //
    // Remember: position() returns the INDEX, not the habit itself!
    habits.iter().position(|e|e.name == name)
}

fn main() {
    println!("🦀 Habit Tracker CLI\n");

    // TODO: Load habits using match on load_habits()
    // If Ok(h) → store in habits, print success
    // If Err(_) → create empty Vec, print "starting fresh"
    // YOUR CODE HERE
    let mut habits: Vec<Habit> = match load_habits() {
        Ok(h) => h,
        Err(_) => vec![],
    };

    // TODO: Create infinite loop
    // YOUR CODE HERE {
    loop{
        // TODO: Print prompt "> " (no newline!)
        print!(">");
        // TODO: Flush stdout so prompt appears
        io::stdout().flush().unwrap();
        // TODO: Create empty String for input
        let mut input = String::new();
        // TODO: Read line from stdin into input
        io::stdin().read_line(&mut input).unwrap();
        
        // TODO: Trim the input
        let input = input.trim();
        // TODO: Skip if input is empty (use continue)
        
        // TODO: Split input into parts by whitespace
        let parts: Vec<&str> = input.split_whitespace().collect();
        // TODO: Get first part as command (handle empty input!)
        // TODO: Get rest as arguments
        let command = *parts.get(0).unwrap_or(&"");
        let args = if parts.len()>1{&parts[1..]} else {&[]};
        
        // TODO: Match command:
        // "quit" or "q" → break
        // "help" or "h" → print command list
        // _ → print "Unknown command: <command>"
        match command {
            "quit" | "q" => {
                println!("👋 Goodbye!");
                break;
            }
            
            "help" | "h" => {
                println!("\n📋 Available Commands:");
                println!("  add <name>      - Add a new habit");
                println!("  list            - Show all habits");
                println!("  view <name>     - Show specific habit");
                println!("  complete <name> - Increment habit streak");
                println!("  reset <name>    - Reset habit to 0");
                println!("  delete <name>   - Remove habit");
                println!("  stats           - Show statistics");
                println!("  save            - Save to file");
                println!("  quit            - Exit\n");
            }
            
            "" => continue,
            
            _ => {
                println!("❌ Unknown command: '{}'", command);
                println!("💡 Type 'help' to see available commands");
            }

            "list" | "l" => {
                if habits.is_empty(){println!("No habits yet! Use 'add <name>' to create one.")}
                else{
                    println!("\n Your habits:")
                    for (i, habit) in habits.iter().enumerate(){
                        println!("{}. {}. Current streak is {} days",i,habit.name, habit.streak)
                    }
                    println!();
                }
            }       
        }
    }
        // TODO: Print goodbye message
}
