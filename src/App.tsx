import React, { useRef } from "react";
import { useEffect } from "react";
import { TextRenderer } from "../wasm/pkg/wasm";
function App() {
  const canvasRef = React.createRef<HTMLCanvasElement>();
  const renderer = useRef<TextRenderer>();

  useEffect(() => {
    const context = canvasRef.current?.getContext("2d")!;

    renderer.current = new TextRenderer(context);

    const text =
      "あのイーハトーヴォのすきとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。\n" +
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

    renderer.current.drawText(text, 32, 48, 32, 32, 320);
  }, []);

  return (
    <div className="p-4">
      <canvas
        ref={canvasRef}
        width="2000"
        height="800"
        className="w-[1000px] h-[400px] border-2 border-black"
      />
    </div>
  );
}

export default App;
