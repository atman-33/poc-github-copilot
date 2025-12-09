use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
