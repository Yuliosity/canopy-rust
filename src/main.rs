extern crate csv;
extern crate rustc_serialize;

use std::env;

#[derive(RustcDecodable)]
struct Profile {
    id: String,
    data: Vec<f32>,
}

// impl Profile {
// }

// fn create_canopy(origin: &Profile) {
//     mut vec<&Profile> neighbours;
//
// }
//
// fn canopy_walk() {
//
// }

fn main() {
    //TODO: Parse command line options

    //RParse point descriptions file
    let mut profiles: Vec<Profile> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let mut reader = csv::Reader::from_file(&args[1]).unwrap().has_headers(false).delimiter(b' ');
    for row in reader.decode() {
        profiles.push(row.unwrap());
    }

    //TODO: Validate points

    //TODO: Precompute pearseon correlations

    //TODO: Canopy clustering

    //TODO: Filtering canopies

    //Writing results
    for profile in profiles {
        println!("{} {}", profile.id, &profile.data[0]);
    }
}
