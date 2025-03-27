# ascii-rs

`ascii-rs` allows you to generate high resolution ascii art based on character weight, and resulting semantics.

It aims to speed up the manual and low resolution process.
Without relying purely on colour and negative space information like many shader-based solutions.

<img width="675" alt="tom" src="https://github.com/user-attachments/assets/1c7df11d-5154-4a25-9fb0-665fb9f35cae" />

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

| <img width="896" alt="godfather_bw" src="https://github.com/user-attachments/assets/8f82e3f1-661e-4b91-8668-052f9bfd618d" /> | <img width="898" alt="godfather" src="https://github.com/user-attachments/assets/28189862-a9c9-4a40-93fa-ead98260d49b" /> |
|:---:|:---:|

`cargo run -- test_inputs/nyc.jpg --width 250 --luma-threshold-mid 50 --luma-threshold-high 170 --no-colour`

`cargo run -- test_inputs/nyc.jpg --width 250 --luma-threshold-mid 50 --luma-threshold-high 170`

| <img width="1123" alt="nyc_bw" src="https://github.com/user-attachments/assets/67714cc1-c726-4162-b0ff-50cf809a8b1c" /> | <img width="1122" alt="nyc" src="https://github.com/user-attachments/assets/b721a568-4b51-40a3-a3b3-9ea90a5ae8aa" /> |
|:---:|:---:|

`cargo run -- test_inputs/tom.jpg  --width=150 --luma-threshold-mid 100 --luma-threshold-high 175`

`cargo run -- test_inputs/tom.jpg  --width=150 --luma-threshold-mid 100 --luma-threshold-high 175 --no-colour`

| <img width="675" alt="tom_bw" src="https://github.com/user-attachments/assets/1d8302a8-952e-479b-a5bb-f17dcd02433f" /> | <img width="675" alt="tom" src="https://github.com/user-attachments/assets/8287ccce-5307-455d-98de-0442ba1ebd2b" /> |
|:---:|:---:|

## 🚀 Feature milestones

- [9873e7408a2b81259cd905901fc43d0b5d69256d](https://github.com/sebszyller/ascii-rs/commit/9873e7408a2b81259cd905901fc43d0b5d69256d) -- mid-tone gradation
- [13ea38f9eec250bcc9c678d516e6dc288c4dc469](https://github.com/sebszyller/ascii-rs/commit/13ea38f9eec250bcc9c678d516e6dc288c4dc469) -- choose char using luma
- [a6ba5eef0beb1195804553d2d8717401f28ffc20](https://github.com/sebszyller/ascii-rs/commit/a6ba5eef0beb1195804553d2d8717401f28ffc20) -- edge detection
- [83d2e2601059a6d3f431218505cffe362d851ad5](https://github.com/sebszyller/ascii-rs/commit/83d2e2601059a6d3f431218505cffe362d851ad5) -- print coloured ascii
