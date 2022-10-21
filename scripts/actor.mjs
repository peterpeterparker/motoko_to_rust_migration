import fetch from "node-fetch";
import { Actor, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "../src/declarations/motoko_to_rust_migration_backend/motoko_to_rust_migration_backend.did.mjs";

const createActor = (canisterId, options) => {
    const agent = new HttpAgent(options ? { ...options.agentOptions } : {});

    // Fetch root key for certificate validation during development
    if (process.env.NODE_ENV !== "production") {
        agent.fetchRootKey().catch((err) => {
            console.warn(
                "Unable to fetch root key. Check to ensure that your local replica is running"
            );
            console.error(err);
        });
    }

    // Creates an actor with using the candid interface and the HttpAgent
    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
        ...(options ? options.actorOptions : {}),
    });
};

const MAINNET = false;

// Production: not deploy
// local rrkah-fqaaa-aaaaa-aaaaq-cai
export const canisterId = MAINNET
    ? "UNKNOW_CANISTER_ID"
    : "ryjl3-tyaaa-aaaaa-aaaba-cai";

export const managerActor = createActor(canisterId, {
    agentOptions: {
        fetch,
        host: MAINNET ? "https://ic0.app" : "http://localhost:8000",
    },
});