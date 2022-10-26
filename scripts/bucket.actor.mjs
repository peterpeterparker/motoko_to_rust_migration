import fetch from "node-fetch";
import { idlFactory } from "../src/declarations/motoko_data/motoko_data.did.mjs";
import { createActor } from "./actor2.mjs";

const MAINNET = false;

// Production: not deploy
// local rrkah-fqaaa-aaaaa-aaaaq-cai
export const canisterId = MAINNET
  ? "UNKNOW_CANISTER_ID"
  : "qaa6y-5yaaa-aaaaa-aaafa-cai";

export const bucketActor = createActor({
  canisterId,
  options: {
    agentOptions: {
      fetch,
      host: MAINNET ? "https://ic0.app" : "http://localhost:8000",
    },
  },
  factory: idlFactory,
});
