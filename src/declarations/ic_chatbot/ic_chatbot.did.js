export const idlFactory = ({ IDL }) => {
  const SessionId = IDL.Text;
  const UserInput = IDL.Text;
  const JsonText = IDL.Text;
  return IDL.Service({
    'get_next_block' : IDL.Func([SessionId, UserInput], [JsonText], []),
    'init_session' : IDL.Func([], [JsonText], []),
  });
};
export const init = ({ IDL }) => { return []; };
