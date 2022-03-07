mod config;
mod info;
mod read;
mod write;

use info::Info;

fn main() {
    let info = Info::read();
    info.write();

    println!("DONE!");
}
