
import Iter "mo:base/Iter";
import HashMap "mo:base/HashMap";
import Text "mo:base/Text";
import Blob "mo:base/Blob";

actor class DataBucket(owner: {user: Text}) = this {

    private stable let user : {user: Text} = owner;

    private type Asset = {
        key : Text;
        value: [Nat8];
    };

    private stable var entries : [(Text, Asset)] = [];

    private var assets : HashMap.HashMap<Text, Asset> = HashMap.HashMap<Text, Asset>(
      10,
      Text.equal,
      Text.hash
    );

    public func put(key: Text, value : Text) : async () {
        assets.put(key, {
            key = value;
            value = Blob.toArray(Text.encodeUtf8(value));
        });
    };

    public query func get() : async ({user: Text; assets: [(Text, Asset)]}) {
        let values = Iter.toArray(assets.entries());
        let u = user.user;

        {
            user = u;
            assets = values;
        }
    };

    system func preupgrade() {
        entries := Iter.toArray(assets.entries());
    };

    system func postupgrade() {
        assets := HashMap.fromIter<Text, Asset>(entries.vals(), 10, Text.equal, Text.hash);
        entries := [];
    };
     
}