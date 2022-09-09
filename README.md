# wasm-text-rendering

Get glyph data and render text to a canvas using Rust Wasm

## Used libraries

- [rustybuzz](https://github.com/RazrFalcon/rustybuzz) for text shaping
- [ttf-parser](https://github.com/RazrFalcon/ttf-parser) for retrieving glyph outlines
- [unicode-linebreak](https://github.com/axelf4/unicode-linebreak) for text wrapping

## Setup

- Make sure Rust and wasm-pack are installed

```bash
npm install
npm run wasm:build
npm run dev
```
