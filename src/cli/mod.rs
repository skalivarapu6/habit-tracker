use std::io::{self, Write};
use ratatui::Viewport;
use serde::forward_to_deserialize_any;
use itertools::Itertools;
use crate::habit::{Habit, HabitFunctions};
use crate::stats::{StreakStats};
use crate::storage::{save_habits, load_habits};
use crate::validation::{is_valid_habit_name, find_habit_by_name};

pub fn run(){
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
                println!("  track <name> <unit> <unit_size>    - Add a new metric to track\n");
                println!("  list            - Show all habits");
                println!("  save            - Saves progress");
                println!("  view <name>     - Show specific habit");
                println!("  complete <name> - Increment habit streak");
                println!("  log <name> <quantity>    - Log quantity\n");
                println!("  reset <name>    - Reset habit to 0");
                println!("  delete <name>   - Remove habit");
                println!("  stats           - Show statistics");
                println!("  save            - Save to file");
                println!("  quit            - Exit\n");
            }
            
            "" => continue,
            "track"|"t"=>{
                if args.len() != 3 {
                    println!("To use: track <name> <unit> <unit_size>");
                    continue;
                }
                let name = args[0].to_string();
                let unit = args[1].to_string();
                let unit_size: u32 = match args[2].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("unit_size must be a number");
                        continue;
                    }
                };

                habits.push(Habit::new_quantity(name, unit, unit_size));
                println!("📊 Quantity habit added!");
            }
            "log" =>{
                if args.len()!= 2{
                    println!("To use: log <name> <quantity> ")
                }
                let name: String = args[0].to_string();
                let quantity: u32 = match args[1].parse(){
                    Ok(n) => n,
                    Err(_) => {
                        println!("quantity must be a number");
                        continue;
                    },
                };
                if let Some(index) = find_habit_by_name(&name, &habits) {
                    match habits[index].log_amount(quantity){
                        Ok(msg) => println!("✅ {}", msg),
                        Err(e) => println!("❌ {}", e),                   
                    }
                }                

            }
            "list" | "l" => {
                if habits.is_empty() {
                    println!("No habits yet!");
                } else {
                    println!("\nYour habits:");
                    for (i, habit) in habits.iter().enumerate() {
                        println!("  {}. {}", i + 1, habit.display_line());
                    }
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
                        habits.push(Habit::new_streak(habit_name.to_string()));
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
                    // match find_habit_by_name(habit_name, &habits){
                    //     Some(index) =>{
                    //         habits[index].complete();
                    //         let new_streak = habits[index].streak;
                    //         println!("Great job! You upped your streak from {} to {}", new_streak-1, new_streak);
                    //     }
                    //     None => println!("Habit name {} not found", habit_name)
                    // }
                    if let Some(index) = find_habit_by_name(habit_name, &habits) {
                        match habits[index].complete() {
                            Ok(msg) => println!("✅ {}", msg),
                            Err(e) => println!("❌ {}", e),
                        }
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
                    if let Some(index) = find_habit_by_name(habit_name, &habits) {
                        println!("{}", habits[index].display_line());

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
                        Some(index) => {habits[index].reset();println!("Reset exercise {}", habits[index].name());}
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
            // "stats" => {
            //     if habits.is_empty() {
            //         println!("📊 No habits to show stats for!");
            //     } else {
            //         let stats = StreakStats::calculate(&habits);
            //         println!("\n📊 Habit Statistics");
            //         println!("━━━━━━━━━━━━━━━━━━━━");
            //         println!("Total habits: {}", stats.total);
            //         println!("Active (streak > 0): {}", stats.active);
            //         println!("Longest streak: {} days", stats.longest);
            //         println!("Average streak: {:.1} days\n", stats.average);
            //     }
            // }
            _ => {
                println!("❌ Unknown command: '{}'", command);
                println!("💡 Type 'help' to see available commands");
            }
        }
    }
}
