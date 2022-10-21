export const idlFactory = ({ IDL }) => {
  const Asset = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Vec(IDL.Nat8) });
  const DataBucket = IDL.Service({
    'get' : IDL.Func(
        [],
        [
          IDL.Record({
            'assets' : IDL.Vec(IDL.Tuple(IDL.Text, Asset)),
            'user' : IDL.Text,
          }),
        ],
        ['query'],
      ),
    'put' : IDL.Func([IDL.Text, IDL.Text], [], []),
  });
  return DataBucket;
};
export const init = ({ IDL }) => {
  return [IDL.Record({ 'user' : IDL.Text })];
};
