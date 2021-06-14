#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs;


#[derive(Serialize, Deserialize)]
struct Parliament {
    id: u32,
    name: String,
    periods: Vec<Period>,
}

#[derive(Serialize, Deserialize)]
struct Period {
    id: u32,
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


fn parse_polls() -> Result<Vec<Parliament>, std::error::Error>{
    let contents = fs::read_to_string("src/tinytestdata.json")?;

    let p: Vec<Parliament> = serde_json::from_str(&contents)?;

    Ok(p)
}



#[get("/list?<period>&<count>")]
fn index(period: u32,count: u32) -> String {
    let x: Vec<Parliament> = match parse_polls() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }

    format!("The ID is {}",x.len())

}



#[launch]
fn rocket() -> _ {

    rocket::build()
    .mount("/", routes![index])
}
