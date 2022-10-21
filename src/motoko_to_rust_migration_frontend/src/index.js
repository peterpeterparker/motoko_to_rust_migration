import {motoko_to_rust_migration_backend} from "../../declarations/motoko_to_rust_migration_backend";
import {createActor} from "../../declarations/motoko_data";

let canisterId;

const initCanister = async () => {
  try {
    const principal = await motoko_to_rust_migration_backend.init();
    console.log('Initialized', principal.toText());

    canisterId = principal;
  } catch (err) {
    console.error(err);
  }
}

const put = async () => {
  try {
    const actor = createActor(canisterId);
    await actor.put("1", "hello");
    await actor.put("2", "world");
    console.log("Done put.");
  } catch (err) {
    console.error(err);
  }
}

const get = async () => {
  try {
    const actor = createActor(canisterId);
    const results = await actor.get();
    console.log("Get:", results);
  } catch (err) {
    console.error(err);
  }
}

const init = () => {
  const btnInit = document.querySelector("button#init");
  btnInit.addEventListener("click", initCanister);

  const btnPut = document.querySelector("button#put");
  btnPut.addEventListener("click", put);

  const btnGet = document.querySelector("button#get");
  btnGet.addEventListener("click", get);
};

document.addEventListener("DOMContentLoaded", init);
