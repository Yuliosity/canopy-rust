extern crate csv;

use std::env;

impl Vec<T> {
    fn filter_inplace(&mut self, pred: |&T| -> bool {
        //TODO: implement
    }
}

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
    let mut canopies: Vec<Canopy> = Vec::new();

    //TODO: Filtering canopies
    //let canopies = canopies.
    canopies.filter_inplace(|c| c.center.nonzero_components() > 0);


    //Writing results
    for profile in profiles {
        println!("{} {}", profile.id, &profile.data[0]);
    }
}
