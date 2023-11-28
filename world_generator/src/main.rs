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
