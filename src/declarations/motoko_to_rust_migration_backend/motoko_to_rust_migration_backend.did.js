export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'init' : IDL.Func([], [IDL.Principal], []) });
};
export const init = ({ IDL }) => { return []; };
