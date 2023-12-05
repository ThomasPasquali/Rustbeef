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


# Implementation

## get_move_for_content OR get_move_for_tiletype

Steps:

1) See if the destination is known: if !new & already visited & dest is outside min_r
    1) If so, call `get_move_for_coordinate`
    2) Otherwise, discover new Tiles avoiding elevation difference as much as possible


## get_move_for_coordinate