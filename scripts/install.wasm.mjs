#!/usr/bin/env node

import { readFile } from "fs/promises";
import {managerActor} from "./actor.mjs";

const loadWasm = async (type) => {
  const buffer = await readFile(
    `${process.cwd()}/.dfx/local/canisters/${type}/${type}.wasm`
  );
  return [...new Uint8Array(buffer)];
};

const upgradeWasm = async (wasmModule) => {
  console.log(`Upgrading wasm code.`);

  const result = await managerActor.installCode(wasmModule);

  console.log(`Upgraded: ${result.toText()}`);
};

(async () => {
  const wasmModule = await loadWasm("rust_data");

  // Reinstall code
  await upgradeWasm(wasmModule);
})();
