# ascii-rs

## Usage

Invoke help with `ascii-rs -h`:

```shell
Usage: ascii-rs [OPTIONS] --image <IMAGE_PATH>

Options:
      --image <IMAGE_PATH>
      --width <MAX_WIDTH>
      --height <MAX_HEIGHT>
      --light-chars <LIGHT_CHARS>                    [default: ?%#@]
      --heavy-chars <HEAVY_CHARS>                    [default: .,o]
      --edge-char <edge_char>                        [default: +/\]
      --luma-threshold <LUMINANCE_THRESHOLD>         [default: 50]
      --canny-low-threshold <CANNY_LOW_THRESHOLD>    [default: 10]
      --canny-high-threshold <CANNY_HIGH_THRESHOLD>  [default: 50]
      --seed <SEED>                                  [default: 1234567890]
  -h, --help                                         Print help
```

## Feature milestones

- [13ea38f9eec250bcc9c678d516e6dc288c4dc469](https://github.com/sebszyller/ascii-rs/commit/13ea38f9eec250bcc9c678d516e6dc288c4dc469) -- choose char using luma
- [a6ba5eef0beb1195804553d2d8717401f28ffc20](https://github.com/sebszyller/ascii-rs/commit/a6ba5eef0beb1195804553d2d8717401f28ffc20) -- edge detection
- [83d2e2601059a6d3f431218505cffe362d851ad5](https://github.com/sebszyller/ascii-rs/commit/83d2e2601059a6d3f431218505cffe362d851ad5) -- print coloured ascii
