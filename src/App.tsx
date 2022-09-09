import React, { useRef, useState } from "react";
import { useEffect } from "react";
import { TextRenderer } from "../wasm/pkg/wasm";

const defaultText =
  "あのイーハトーヴォのすきとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。\n" +
  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

function App() {
  const canvasRef = React.createRef<HTMLCanvasElement>();
  const contextRef = useRef<CanvasRenderingContext2D>();
  const rendererRef = useRef<TextRenderer>();
  const [text, setText] = useState(defaultText);

  const [fontSize, setFontSize] = useState(32);
  const [width, setWidth] = useState(320);

  useEffect(() => {
    contextRef.current = canvasRef.current?.getContext("2d")!;
    rendererRef.current = new TextRenderer(contextRef.current);
  }, []);

  useEffect(() => {
    const context = contextRef.current;
    const renderer = rendererRef.current;
    if (context && renderer) {
      context.clearRect(0, 0, context.canvas.width, context.canvas.height);
      renderer.drawText(text, fontSize, fontSize * 1.5, 32, 32, width);
    }
  }, [text, fontSize, width]);

  return (
    <div className="flex gap-4 p-4">
      <canvas
        ref={canvasRef}
        width="800"
        height="800"
        className="w-[400px] h-[400px] border-2 border-gray-900"
      />
      <div className="flex flex-col gap-2 ">
        <textarea
          className="border border-gray-300 p-1 rounded w-96 h-32"
          value={text}
          onChange={(e) => setText(e.target.value)}
        />
        <label className="flex flex-col">
          <span className="text-sm text-gray-500">Font Size</span>
          <input
            className="border border-gray-300 p-1 rounded w-40"
            type="number"
            value={fontSize}
            onChange={(e) => setFontSize(parseInt(e.target.value))}
          />
        </label>
        <label className="flex flex-col">
          <span className="text-sm text-gray-500">Width</span>
          <input
            className="border border-gray-300 p-1 rounded w-40"
            type="number"
            value={width}
            onChange={(e) => setWidth(parseInt(e.target.value))}
          />
        </label>
      </div>
    </div>
  );
}

export default App;
