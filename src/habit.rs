use chrono::{NaiveDate, NaiveTime, NaiveWeek, Timelike, format};
use serde::{Deserialize,Serialize};

use crate::validation::find_habit_by_name;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreakHabit{
    pub name: String,
    pub streak: u32,
    pub last_completed: Option<NaiveDate>, 
}

impl StreakHabit{
    pub fn new(name: String)-> Self{
        StreakHabit { name, streak: 0, last_completed: None }
    }
    pub fn mark_complete(&mut self, today: NaiveDate){
        // today = chrono::Local::now().date_naive();
        match self.last_completed{
            Some(last_date)=>{
                if last_date == today {
                    // Already completed today - do nothing
                    return;
                }
                // Check if yesterday (consecutive)
                let yesterday = today.pred_opt().unwrap();
                if last_date == yesterday {
                    self.streak += 1;
                } else {
                    self.streak = 1;
                }
                
                self.last_completed = Some(today);
            }
            None => {
                self.streak+=1;
                self.last_completed = Some(today);
            },
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DailyEntry{
    pub date: NaiveDate,
    pub value: u32,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct HourlyEntry{
    pub hour: u8,
    pub value: u32,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct QuantityHabit{
    pub name: String,
    pub unit: String,
    pub unit_size: u32,
    pub today_total: u32,
    pub today_date: NaiveDate,
    pub today_history: Vec<HourlyEntry>,
    pub history: Vec<DailyEntry>,
}

impl QuantityHabit{
    pub fn new(name: String, unit: String, unit_size: u32) -> Self{
        // let today_date = chrono::Local::now().date_naive();
        // let today_history: Vec<HourlyEntry> = Vec::new();
        // let history: Vec<DailyEntry> = Vec::new(); 
        QuantityHabit { name, unit, unit_size, today_total: 0, 
                        today_date: chrono::Local::now().date_naive(), 
                        today_history: Vec::new(),
                        history: Vec::new()}
    }
    pub fn log(&mut self, amount: u32){
        let now = chrono::Local::now();
        let today = now.date_naive();
        let hour = now.hour() as u8;
        // check if today's date as last saved entry
        // if it is not the same push the last date's history
        // and reset current tracking hour vector to be empty
        if today!=self.today_date{
            // push end of date value
            let entry = DailyEntry{date: self.today_date, value: self.today_total};
            self.history.push(entry);
            // change habit date to current date and set today_total to '0' and reset today_history
            self.today_date = today;
            self.today_total = 0;
            self.today_history.clear();
        }
        // if today's date is same as habit's date
        else{
            let log_amount = amount*self.unit_size;
            self.today_total+=log_amount;
            self.today_history.push(HourlyEntry { hour, value: log_amount });
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Habit{
    Streak(StreakHabit),
    Quantity(QuantityHabit),
}


impl Habit{
    pub fn new_streak(name: String) -> Self{
        Habit::Streak(StreakHabit::new(name))
    }
    pub fn new_quantity(name: String, unit: String, unit_size: u32)->Self{
        Habit::Quantity(QuantityHabit::new(name, unit, unit_size))
    }
    pub fn complete(&mut self) -> Result<String, String>{
        let today = chrono::Local::now().date_naive();
        match self{
            Habit::Streak(streak_habit) => {
                streak_habit.mark_complete(today);
                Ok(format!("Streak: {} days", streak_habit.streak))
            },
            Habit::Quantity(_) => {
                Err("Cannot complete quantity habit. Use 'log <amount>' instead".to_string())            },
        }
    }
    // complete for quantity
    pub fn log_amount(&mut self, amount: u32) -> Result<String, String> {
        match self {
            Habit::Quantity(q) => {
                q.log(amount);
                Ok(format!("Logged {}{}", q.today_total, q.unit))
            }
            Habit::Streak(_) => {
                Err("Cannot log quantity for streak habit. Use 'complete' instead".to_string())
            }
        }
    }
}

// Shared behaviours
pub trait HabitFunctions {
    fn name(&self)->&str;
    fn reset(&mut self);
    fn display_line(&self) -> String;
}

impl HabitFunctions for Habit{
    fn name(&self)->&str {
        match self{
            Habit::Streak(h) => &h.name,
            Habit::Quantity(q) => &q.name,
        }
    }

    fn reset(&mut self) {
        match self {
            Habit::Streak(streak_habit) => streak_habit.streak=0,
            Habit::Quantity(quantity_habit) => quantity_habit.today_total=0,
        }
    }
    
    fn display_line(&self) -> String {
        match self{
            Habit::Streak(s) => {
                format!("[S] {}: streak {}",s.name,s.streak)
            },
            Habit::Quantity(q) => {
                format!("[Q] {}: {}{} today",q.name, q.today_total, q.unit )
            },
        }
    }
}