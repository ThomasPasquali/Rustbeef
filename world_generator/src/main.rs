struct Position{
    x: usize,
    y: usize
}

struct ElevationTile {
    pos: Position,
    elevation: usize,
    expanded: bool
}

// enum PeakType<'a> {
//     Mountain(&'a ElevationTile),
//     Pass(&'a ElevationTile, &'a ElevationTile)
// }

type HeightMap = Vec<Vec<ElevationTile>>;

// impl Display for HeightMap
// fn connect_peaks(peaks: (Position, Position)) -> Vec<Position>{
//     let mut new_pass_tiles = 
// }


fn main() {
    //TODO
}
