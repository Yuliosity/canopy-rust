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

fn canopy_walk() {
    //TODO: implement
}
