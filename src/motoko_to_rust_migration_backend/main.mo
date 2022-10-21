import Cycles "mo:base/ExperimentalCycles";
import Principal "mo:base/Principal";
import Error "mo:base/Error";

import IC "./ic.types";

import DataBucket "../motoko_data/data";

actor Main {
  private let ic : IC.Self = actor "aaaaa-aa";

  private type DataBucket = DataBucket.DataBucket;

  private stable var canisterId: ?Principal = null;

  private let user: {user: Text} = {user = "David"};
  
  public shared({ caller }) func init(): async (Principal) {
    Cycles.add(1_000_000_000_000);

    let b = await DataBucket.DataBucket(user);

    canisterId := ?(Principal.fromActor(b));

    switch (canisterId) {
      case null {
        throw Error.reject("Bucket init weird error");
      };
      case (?canisterId) {
        let self: Principal = Principal.fromActor(Main);

        let controllers: ?[Principal] = ?[canisterId, caller, self];

        await ic.update_settings(({canister_id = canisterId; settings = {
            controllers = controllers;
            freezing_threshold = null;
            memory_allocation = null;
            compute_allocation = null;
        }}));

        return canisterId;
      };
    };
  };

  public shared ({ caller }) func installCode(wasmModule : Blob) : async (Principal) {
    switch (canisterId) {
      case null {
        throw Error.reject("No bucket canisterId to install");
      };
      case (?cId) {
        await ic.install_code({
          arg = to_candid(user);
          wasm_module = wasmModule;
          mode = #upgrade;
          canister_id = cId;
        });

        return cId;
      };
    };
  };
};
