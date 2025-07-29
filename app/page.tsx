"use client";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function Home() {
  const [input, setInput] = useState("");
  const [data, setData] = useState("");

  function handleBtn() {
    invoke<string>("app", { data: input })
      .then((res) => setData(res))
      .catch(console.error);
  }
  return (
    <>
      <div className="px-4 py-8">
        <h1>{data}</h1>
      </div>
      <input
        type="text"
        placeholder="safe text..."
        value={input}
        onChange={(e) => setInput(e.target.value)}
      ></input>
      <button onClick={handleBtn}>Send</button>
    </>
  );
}
