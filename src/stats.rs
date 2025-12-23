
use crate::habit::{Habit, StreakHabit};

pub struct StreakStats {
    pub total: usize,
    pub active: usize,
    pub longest: u32,
    pub total_days: u32,
    pub average: f64,
}

impl StreakStats {
    pub fn calculate(habits: &[StreakHabit]) -> Self {
        // TODO: Your implementation!
        // Calculate all the stats
        // Return StreakStats { ... }
        let total = habits.len();
        let active = habits.iter()
                            .filter(|h|h.streak>0).count();
        let longest = habits.iter()
                            .map(|h| h.streak)
                            .max()
                            .unwrap_or(0);
        let total_days:u32 = habits.iter()
                            .map(|h|h.streak).sum();
        let average = if total_days>0{total_days as f64 / total as f64}
            else{
                0 as f64
            };
            StreakStats{ total, active, longest, total_days, average }

    }
    
    pub fn display_stats(&self) -> String {
        // TODO: Format for CLI display
        format!(
            "Longest {} • Active {} • Total {} • Average {:.1}",
            self.longest, self.active, self.total_days, self.average
        )
    }
}

// pub struct QuantityStats{
//     pub 
// }