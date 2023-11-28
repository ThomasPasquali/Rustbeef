const WORLD_SIZE: Dimension = Dimension{ length: 1000, width: 1000 };
const HEIGHT: usize = WORLD_SIZE/10;
const MIN_MOUNTAIN_SIZE: Dimension = Dimension{length: 30, width: 30};
const MIN_VALLEY_SIZE: Dimension = Dimension{length: 30, width: 30};

struct Dimension {
    length: usize,
    width: usize
}

struct Peak {
    x: usize,
    y: usize,
    elevation: usize,
    expanded: bool
}

enum PeakType<'a> {
    Mountain(&'a Peak),
    Pass(&'a Peak, &'a Peak)
}

fn main() {
    //TODO
}
