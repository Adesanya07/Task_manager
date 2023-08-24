pub struct Task {
    pub title: String,
    pub description: String,
    pub due_date: chrono::NaiveDate,
    pub completed: bool,
}

impl Task {
   pub fn new(title:String, description:String, due_date:chrono::NaiveDate) -> Self {
        Task{ 
            title,
            description,
            due_date,
            completed: false,
         }
    }
}