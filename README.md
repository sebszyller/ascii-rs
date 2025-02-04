# ascii-rs

`ascii-rs` allows you to generate high resolution ascii art based on character weight, and resulting semantics.

It aims to speed up the manual and low resolution process.
Without relying purely on colour and negative space information like many shader-based solutions.

## 🛠️ Usage

Just `git clone` and `cargo build` however you want.

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
## 💡 Tips

🚨**Does your ascii art have weird dimensions?**

Not all typefaces and line settings work well for ascii art.

All examples in this repo are generated with:
- font size `< 8`;
- alacritty line offset `offset = { x = 0, y = -11 }`;
- Courier New typeface.

These are **not** my usual terminal settings.

🚨**Thresholding is fiddly.**

It's quite difficult to find edge and luma thresholds that work across various images.
For best results, you need to experiment with different values.
Pass `--output-edges` to inspect the Canny algorithm output.

🚨**It's slow**

While ascii conversion is fast, Canny can take quite long.
Your typical terminal is going to be `< 500` characters wide.
There's no point processing images larger than that.

See recipes.

## 📜 Recipes

`cargo run -- test_inputs/godfather.jpg  --width=200 --luma-threshold-mid 100 --luma-threshold-high 175 --no-colour`

`cargo run -- test_inputs/godfather.jpg  --width=200 --luma-threshold-mid 100 --luma-threshold-high 175`

| placeholder.jpg | placeholder.jpg |
|:---:|:---:|

`cargo run -- test_inputs/nyc.jpg --width 250 --luma-threshold-mid 50 --luma-threshold-high 170 --no-colour`

`cargo run -- test_inputs/nyc.jpg --width 250 --luma-threshold-mid 50 --luma-threshold-high 170`

| placeholder.jpg | placeholder.jpg |
|:---:|:---:|

`cargo run -- test_inputs/tom.jpg  --width=150 --luma-threshold-mid 100 --luma-threshold-high 175`

`cargo run -- test_inputs/tom.jpg  --width=150 --luma-threshold-mid 100 --luma-threshold-high 175 --no-colour`

| placeholder.jpg | placeholder.jpg |
|:---:|:---:|

## 🚀 Feature milestones

- [9873e7408a2b81259cd905901fc43d0b5d69256d](https://github.com/sebszyller/ascii-rs/commit/9873e7408a2b81259cd905901fc43d0b5d69256d) -- mid-tone gradation
- [13ea38f9eec250bcc9c678d516e6dc288c4dc469](https://github.com/sebszyller/ascii-rs/commit/13ea38f9eec250bcc9c678d516e6dc288c4dc469) -- choose char using luma
- [a6ba5eef0beb1195804553d2d8717401f28ffc20](https://github.com/sebszyller/ascii-rs/commit/a6ba5eef0beb1195804553d2d8717401f28ffc20) -- edge detection
- [83d2e2601059a6d3f431218505cffe362d851ad5](https://github.com/sebszyller/ascii-rs/commit/83d2e2601059a6d3f431218505cffe362d851ad5) -- print coloured ascii
