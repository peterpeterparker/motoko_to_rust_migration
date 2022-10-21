export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'init' : IDL.Func([], [IDL.Principal], []),
    'installCode' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Principal], []),
  });
};
export const init = ({ IDL }) => { return []; };
