#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use rocket::State;
use rocket::serde::json::Json;
use serde_json;
use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Parliament {
    name: String,
    periods: HashMap<u32, Period>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Period {
    name: String,
    polls: Vec<Poll>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Poll {
    id: u32,
    title: String,
    date: String,
    description: String,
    sources: Vec<String>,
    votes: HashMap<String,Vec<u32>>,
}

#[derive(Serialize, Deserialize)]
struct Data {
    data: HashMap<u32, Parliament>,
}

fn parse_polls() -> Result<HashMap<u32, Parliament>>{
    let contents = fs::read_to_string("src/data.json").expect("Unable to read file");

    let p: HashMap<u32, Parliament> = serde_json::from_str(&contents)?;

    Ok(p)
}



#[get("/list?<parliament_id>&<count>")]
fn index(parliament_id: u32,count: u32, state: &State<Data>) -> Json<Vec<&Poll>> {
    let mut polls: Vec<&Poll> = Vec::new();
    let poll1 = &state.data[&parliament_id].periods[&97].polls[0];
    let poll2 = &state.data[&parliament_id].periods[&97].polls[1];
    polls.push(poll1);
    polls.push(poll2);

    Json(polls)
}



#[launch]
fn rocket() -> _ {
    let x: HashMap<u32,Parliament> = match parse_polls() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    rocket::build()
    .mount("/", routes![index])
    .manage(Data{data: x })
}
