use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Task {
    pub id:usize,
    pub title:String,
}