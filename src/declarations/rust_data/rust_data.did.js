export const idlFactory = ({ IDL }) => {
  const Assets = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Vec(IDL.Nat8) });
  const State = IDL.Record({ 'owner' : IDL.Text, 'assets' : Assets });
  return IDL.Service({ 'get' : IDL.Func([], [State], ['query']) });
};
export const init = ({ IDL }) => { return []; };
