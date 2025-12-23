mod habit;
mod storage;
mod validation;
mod cli;
mod tui;
pub mod stats;

use std::env;

fn main()  {
    // let args: Vec<String> = env::args().collect();
    
    // if args.len() > 1 && args[1] == "--tui" {
    //     tui::run()?;
    // } else {
        cli::run();
    // }
    
    // Ok(())
}


// fn main() {
//     println!("🧪 Testing Habit Logic\n");
    
//     // Import what you need
//     use crate::habit::{StreakHabit, QuantityHabit};
//     use chrono::NaiveDate;
    
//     // Test StreakHabit
//     println!("=== Testing StreakHabit ===\n");
    
//     let mut exercise = StreakHabit::new("exercise".into());
//     println!("New habit: streak={}", exercise.streak);
    
//     // Simulate Day 1
//     let day1 = NaiveDate::from_ymd_opt(2024, 12, 16).unwrap();
//     exercise.last_completed = None;  // Reset for testing
//     exercise.streak = 0;
    
//     // Complete on day 1
//     // (manually call the logic since mark_complete uses now())
//     exercise.streak = 1;
//     exercise.last_completed = Some(day1);
//     println!("Day 1 complete: streak={}", exercise.streak);
    
//     // Simulate Day 2 (consecutive)
//     let day2 = NaiveDate::from_ymd_opt(2024, 12, 17).unwrap();
//     // Manually run logic:
//     if let Some(last) = exercise.last_completed {
//         let yesterday = day2.pred_opt().unwrap();
//         if last == yesterday {
//             exercise.streak += 1;
//         }
//     }
//     println!("Day 2 complete (consecutive): streak={}", exercise.streak);
    
//     // Simulate Day 5 (skipped days)
//     let day5 = NaiveDate::from_ymd_opt(2024, 12, 20).unwrap();
//     let yesterday = day5.pred_opt().unwrap();
//     if let Some(last) = exercise.last_completed {
//         if last != yesterday {
//             exercise.streak = 1;  // Broke streak!
//         }
//     }
//     println!("Day 5 complete (broke streak): streak={}", exercise.streak);
    
//     println!("\n=== Testing QuantityHabit ===\n");
    
//     let mut water = QuantityHabit::new("water".into(), "ml".into(), 500);
//     println!("New metric: total={}ml", water.today_total);
    
//     water.log(2);
//     println!("After log(2): total={}ml, entries={}", 
//         water.today_total, water.today_history.len());
    
//     water.log(1);
//     println!("After log(1): total={}ml, entries={}", 
//         water.today_total, water.today_history.len());
    
//     println!("\nHourly breakdown:");
//     for entry in &water.today_history {
//         println!("  Hour {}: {}ml", entry.hour, entry.value);
//     }
// }