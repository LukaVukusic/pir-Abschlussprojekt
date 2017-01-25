#[macro_use] extern crate enum_primitive;
extern crate rustc_serialize;
extern crate time;

use time::get_time;

mod arena;
mod db;
mod graphic;
mod player;

fn main() {
    for entry in db::natures::create_naturedex(){
        println!("{:?}", entry);
    }
}
