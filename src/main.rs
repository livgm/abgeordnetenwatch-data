#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use std::error::Error;


#[derive(Serialize, Deserialize)]
struct Parliament {
    name: String,
    periods: HashMap<u32, Period>,
}

#[derive(Serialize, Deserialize)]
struct Period {
    name: String,
    polls: Vec<Poll>,
}

#[derive(Serialize, Deserialize)]
struct Poll {
    id: u32,
    title: String,
    date: String,
    description: String,
    sources: Vec<String>,
    votes: serde_json::Value,
}


fn parse_polls() -> Result<HashMap<u32, Parliament>>{
    let contents = fs::read_to_string("src/data.json").expect("Unable to read file");

    let p: HashMap<u32, Parliament> = serde_json::from_str(&contents)?;

    Ok(p)
}



#[get("/list?<period>&<count>")]
fn index(period: u32,count: u32) -> String {
    let x: HashMap<u32,Parliament> = match parse_polls() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    format!("The ID is {}",x.len())

}



#[launch]
fn rocket() -> _ {

    rocket::build()
    .mount("/", routes![index])
}
