extern crate rustc_serialize;

#[derive(RustcDecodable)]
struct Profile {
    id: String,
    data: Vec<f32>,
    pearson: Vec<f32>
}

impl Profile {
    fn new(size: u8) -> Profile {
        Profile {
            id: "!GENERATED!",
            data: vec![0; size],
            pearson: Vec::new()
        }
    }

    fn nonzero_components(&self) -> u8 {
        data.iter().filter(|x| x > 0).count()
    }

    fn single_point_proportion(&self) -> f32 {
        0 //TODO: implement
    }

    fn precompute_pearson(&mut self) {
        let avg: f64 = self.data.iter().sum() / self.data.size();
        let factor_sum: f64 = self.data.iter().map(|x| (x - avg).powi(2)).sum();
        let stddev: f64 = sqrt(factor_sum / self.data.size());

        //Precompute pearson data
        //if (sttdev.abs() < 2 *
        pearson = self.data.iter().map(|x| (x - avg) / (stddev * self.data.size())).collect::<Vec<f64>>();
    }

    fn corr(&self, other: &Profile) {
        self.pearson.iter().zip(other.pearson.iter()).map(mul).sum() * self.pearson.size()
    }
}

fn distance(p1, p2: &Profile) -> f32 {

}

fn centroid(cluster: &Vec<Profile>) -> Profile {
    let mut centroid = Point::new(cluster[0].data.size());
    //TODO: implement
    centroid
}
