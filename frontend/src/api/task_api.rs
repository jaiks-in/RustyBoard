use crate::models::task::Task;
use reqwasm::http::Request;

pub async fn fetch_task()->Vec<Task>{
    let fetch_task=Request::get("http://localhost:8081/health").send().await
        .and_then(|resp| resp.json::<Vec<Task>>() );
    match fetch_task{
        Ok(tasks)=>{
            tasks
        },
        Err(_)=>vec![],
    }
}
