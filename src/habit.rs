use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit{
    pub name: String,
    pub streak: u32,
}

impl Habit{
    pub fn new(name: String)-> Self{
        Habit{name,streak:0}
    }
    pub fn complete(&mut self){
        self.streak+=1;
    }
    pub fn reset(&mut self){
        self.streak =0;
    }
}