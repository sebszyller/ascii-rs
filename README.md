# ascii-rs

## Usage

Invoke help with `ascii-rs -h`:

```shell
Usage: ascii-rs [OPTIONS] <IMAGE_PATH>

Arguments:
  <IMAGE_PATH>

Options:
      --width <MAX_WIDTH>
      --height <MAX_HEIGHT>
      --light-chars <LIGHT_CHARS>                       [default: ?%#@]
      --medium-chars <MEDIUM_CHARS>                     [default: DOS]
      --dark-chars <DARK_CHARS>                         [default: .,]
      --edge-chars <EDGE_CHARS>                         [default: /\]
      --luma-threshold-mid <LUMINANCE_THRESHOLD_MID>    [default: 50]
      --luma-threshold-high <LUMINANCE_THRESHOLD_HIGH>  [default: 95]
      --canny-threshold-low <CANNY_THRESHOLD_LOW>       [default: 10]
      --canny-threshold-high <CANNY_THRESHOLD_HIGH>     [default: 50]
      --no-colour
      --output-edges
      --seed <SEED>                                     [default: 1234567890]
  -h, --help                                            Print help
```

## Feature milestones

- [9873e7408a2b81259cd905901fc43d0b5d69256d](https://github.com/sebszyller/ascii-rs/commit/9873e7408a2b81259cd905901fc43d0b5d69256d) -- mid-tone gradation
- [13ea38f9eec250bcc9c678d516e6dc288c4dc469](https://github.com/sebszyller/ascii-rs/commit/13ea38f9eec250bcc9c678d516e6dc288c4dc469) -- choose char using luma
- [a6ba5eef0beb1195804553d2d8717401f28ffc20](https://github.com/sebszyller/ascii-rs/commit/a6ba5eef0beb1195804553d2d8717401f28ffc20) -- edge detection
- [83d2e2601059a6d3f431218505cffe362d851ad5](https://github.com/sebszyller/ascii-rs/commit/83d2e2601059a6d3f431218505cffe362d851ad5) -- print coloured ascii
