
use diffurch::*;

fn main() {

    let t = Time;
    let [x, y] = Coord::vec();

    let eq : [Box<dyn Symbol>; 2] = [Box::new(y), Box::new(-x)];
}
