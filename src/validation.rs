use crate::habit::Habit;

pub fn is_valid_habit_name(name: &str) -> bool{
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

pub fn find_habit_by_name(name: &str, habits: &[Habit]) -> Option<usize>{
    habits.iter().position(|e|e.name == name)
}