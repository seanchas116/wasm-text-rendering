import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

import init, { start, TextRenderer } from "../wasm/pkg/wasm";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

init().then(() => {
  start();

  const canvas = document.getElementById("canvas") as HTMLCanvasElement;
  const context = canvas.getContext("2d")!;

  const renderer = new TextRenderer(context);

  const text =
    "あのイーハトーヴォのすきとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。\n" +
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

  renderer.drawText(text, 32, 48, 32, 32, 320);
});
