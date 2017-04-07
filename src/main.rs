extern crate csv;
extern crate rustc_serialize;

use std::env;

#[derive(RustcDecodable)]
struct Profile {
    id: String,
    data: Vec<f32>,
}

impl Profile {
    fn new(size: u8) -> Profile {
        Profile {
            id: "!GENERATED!",
            data: vec![0; size]
        }
    }

    fn nonzero_components(&self) -> u8 {
        data.iter().filter(|x| x > 0).count()
    }

    fn single_point_proportion(&self) -> f32 {
        0 //TODO: implement
    }
}

fn distance(p1, p2: Profile) -> f32 {

}

fn centroid(cluster: &Vec<Profile>) -> Profile {
    let mut centroid = Point::new(cluster[0].data.size());
    //TODO: implement
    centroid
}

struct Canopy<'a> {
    center: &Profile<'a>,
    neighbours: Vec<&Profile<'a>>
}

impl Canopy {
    fn new(origin: &Profile<'a>) -> Canopy<'a> {
        let mut neighbours: Vec<&Profile>;
        //TODO: implement
        Canopy {
            center: &origin,
            neighbours: Vec::new()
        }
    }

    fn update_center(&mut self) {
        //TODO: implement
    }
}

impl Ord for Canopy {
    fn cmp(&self, other: &Canopy) -> Ordering {
        self.neighbours.size().cmp(other.neighbours.size())
    }
}

impl Vec<T> {
    fn filter_inplace(&mut self, pred: |&T| -> bool {
        //TODO: implement
    }
}

fn canopy_walk() {
    //TODO: implement
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
