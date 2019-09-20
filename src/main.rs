#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)] mod tests;

#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use std::path::{PathBuf, Path};
use rocket::response::NamedFile;




#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

type ID = usize;
type MessageMap = Mutex<HashMap<ID,String>>;

#[derive(Serialize, Deserialize)]
struct Message{
    id:ID,
    comments:String,
}


#[post("/<id>", format = "json", data = "<message>")]
fn new(id:ID, message: Json<Message>, map:State<'_, MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        json!({
          "status":"error",
          "reason":"ID already exists"
        })
    }else{
        hashmap.insert(id,message.0.comments);
        json!({"status":"ok"})
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn hello(name:&RawStr) -> String {
    format!("Hello {}",name.as_str())
}

#[get("/<file..>")]
fn get_file(file:PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("test/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index])
                    .mount("/hello",routes![hello])
                    .mount("/getfile",routes![get_file])
                    .launch();
}