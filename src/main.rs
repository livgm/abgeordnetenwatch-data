#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use rocket::State;
use rocket::Responder;
use rocket_contrib::json::Json;
use serde_json::ser;
use serde_json::json;

//TO-DO:
// What does responder do?



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
fn index(parliament_id: u32,count: u32, state: &State<Data>) -> String {
    let mut poll: Vec<&Poll> = Vec::new();
    let poll1 = &state.data[&parliament_id].periods[&97].polls[0];
    let poll2 = &state.data[&parliament_id].periods[&97].polls[1];
    poll.push(poll1);
    poll.push(poll2);

    serde_json::to_string(&poll).unwrap()
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
