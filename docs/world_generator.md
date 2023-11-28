# WORLD GENERATOR
![Group](https://img.shields.io/badge/Group-RustBeef-blue)

## Description

lorem ipsum


## Parameters

- `world-size: usize`
- `min_mountains: usize`
- `passes: usize`
- `valleys: usize`


## Constants

Minimum sizes:
- `VALLEY_SIZE`: depends on mountain size
- `MOUNTAIN_SIZE`


## Pseudocode

```
param_check(min_mountains, passes, valleys)

struct Peak {
    x: usize,
    y: usize,
    elevation: usize
}

enum PeakType {
    Mountain(Peak),
    Valley(Peak, Peak)
}

generate_peaks(\todo) -> vec[PeakType]

for peak in Peaks:
    expand(PeakType)

populate()

```
