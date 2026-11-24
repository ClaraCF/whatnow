use chrono::NaiveDate;

pub type TaskId = u32;

pub struct Task {
    id: TaskId,
    description: String,
    priority: Option<i32>,
    deadline: Option<NaiveDate>
}
