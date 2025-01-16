use anyhow::Result;
use std::collections::HashMap;

use aoc_utils::*;
use day_12::*;

#[inline]
fn is_corner(
    plant: Option<char>,
    first_adjacent: Option<char>,
    second_adjacent: Option<char>,
    diagonal: Option<char>,
) -> bool {
    (first_adjacent != plant && second_adjacent != plant)
        || (first_adjacent == plant && second_adjacent == plant && diagonal != plant)
}

#[derive(Debug)]
struct Garden {
    pub plots: HashMap<Point, char>,
    pub regions: Vec<Region>,
}

impl Garden {
    fn cost(&self) -> usize {
        self.regions
            .iter()
            .map(|region| {
                // The number of sides is equivelent to the number of corners
                let sides = region
                    .region
                    .iter()
                    .map(|plot| self.corners(plot))
                    .sum::<usize>();
                region.area() * sides
            })
            .sum::<usize>()
    }

    fn get(&self, plot: &Point) -> Option<char> {
        self.plots.get(plot).cloned()
    }

    // Find the number of corners that this plot contributes to the shape of the
    // region.
    fn corners(&self, plot: &Point) -> usize {
        let mut count = 0;
        let this = self.get(plot);

        let n = self.get(&plot.north());
        let ne = self.get(&plot.north_east());
        let e = self.get(&plot.east());
        let se = self.get(&plot.south_east());
        let s = self.get(&plot.south());
        let sw = self.get(&plot.south_west());
        let w = self.get(&plot.west());
        let nw = self.get(&plot.north_west());
        count += is_corner(this, n, e, ne) as usize;
        count += is_corner(this, s, e, se) as usize;
        count += is_corner(this, s, w, sw) as usize;
        count += is_corner(this, n, w, nw) as usize;

        count
    }
}

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        let (plots, regions) = read_data(value);
        Self { plots, regions }
    }
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();

    let garden = Garden::from(INPUT);
    let result = garden.cost();

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
