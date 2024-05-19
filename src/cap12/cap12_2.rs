#[derive(Debug)]
struct City {
    name: String,
    years: Vec<u32>,
    populations: Vec<u32>,
}
impl City {
    fn change_city_data<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
    {
        f(&mut self.years, &mut self.populations)
    }
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        years: vec![1372, 1834, 1897, 1925, 1959, 1989, 2000, 2010, 2020],
        populations: vec![
            3_250, 15_300, 58_800, 119_800, 283_071, 478_974, 400_378, 406_703, 437_619,
        ],
    };

    tallinn.change_city_data(|x, y| {
        x.push(2030);
        y.push(500_000);
    });
    tallinn.change_city_data(|years, populations| {
        let new_vec = years
            .iter_mut()
            .zip(populations.iter_mut())
            .take(3)
            .collect::<Vec<(_, _)>>();
        println!("{new_vec:?}");
    });
    tallinn.change_city_data(|x, y| {
        let position_option = x.iter().position(|x| *x == 1834);
        if let Some(position) = position_option {}
    });
    println!(
        "Going to delete {} at position {:?} now.",
        x[position], position
    );
    x.remove(position);
    y.remove(position);
    println!(
        "Years left are {:?}\nPopulations left are {:?}",
        tallinn.years, tallinn.populations
    );
}
