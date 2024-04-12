# Yew Horoscope

A joke app to test the absolute basics of the [yew](https://yew.rs) framework.

## Production

You can test the app by visiting the GitHub Pages.

```
# TODO actually deploy to GH pages via Action
```

## Development

### Requirements

Yew requires `wasm32-unknown-unknown` and `trunk`.

```shell
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

*Or see the [trunk](https://trunkrs.dev/) installation instructions.*

### Running a dev version

To run the app with the default `trunk` config, which is serving at `127.0.0.1:8080`, use:

```shell
trunk serve
```

### Building for production

This builds the compiled result to the dist `dist/`.

```shell
trunk build --release
```

Release builds can be tested with:

```shell
trunk serve --release
```

