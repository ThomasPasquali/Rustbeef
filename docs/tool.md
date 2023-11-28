# Never lost again compass

## Inputs:
- TypeType or Content: final destination


## Methods:
- get_move() -> Option<Direction>
while let Some(direction) = get_move() {
    interface::go.(direction)
}
- set_destination_tiletype(destination: TileType)
- set_destination_content(destination: Content)
- set_destination_coordinate(destination: (usize, usize))