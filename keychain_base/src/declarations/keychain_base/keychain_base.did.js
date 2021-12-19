export const idlFactory = ({ IDL }) => {
  const KeyDataCore = IDL.Record({
    'email_address' : IDL.Text,
    'address' : IDL.Opt(IDL.Text),
    'phone_number' : IDL.Opt(IDL.Text),
  });
  const ErrorResponse = IDL.Record({
    'error_message' : IDL.Text,
    'error_code' : IDL.Nat,
  });
  const EmptySuccessResponse = IDL.Record({ 'code' : IDL.Nat });
  const GetKeyDataSuccessResponse = IDL.Record({
    'code' : IDL.Nat,
    'data' : IDL.Opt(KeyDataCore),
  });
  const ObfuscatedDataCore = IDL.Record({
    'obfuscated_phone_number' : IDL.Opt(IDL.Text),
    'website_url' : IDL.Text,
    'website_id' : IDL.Opt(IDL.Text),
    'password' : IDL.Opt(IDL.Text),
    'obfuscated_email_address' : IDL.Text,
  });
  const GetObfuscatedDataSuccessResponse = IDL.Record({
    'code' : IDL.Nat,
    'data' : IDL.Opt(IDL.Vec(ObfuscatedDataCore)),
  });
  return IDL.Service({
    'change_keydata' : IDL.Func(
        [KeyDataCore, IDL.Text],
        [
          IDL.Variant({
            'error' : ErrorResponse,
            'success' : EmptySuccessResponse,
          }),
        ],
        [],
      ),
    'get_keydata' : IDL.Func(
        [IDL.Text],
        [
          IDL.Variant({
            'error' : ErrorResponse,
            'success' : GetKeyDataSuccessResponse,
          }),
        ],
        ['query'],
      ),
    'get_obfuscated_data' : IDL.Func(
        [IDL.Text],
        [
          IDL.Variant({
            'error' : ErrorResponse,
            'success' : GetObfuscatedDataSuccessResponse,
          }),
        ],
        ['query'],
      ),
    'init_keydata' : IDL.Func(
        [KeyDataCore, IDL.Text],
        [
          IDL.Variant({
            'error' : ErrorResponse,
            'success' : EmptySuccessResponse,
          }),
        ],
        [],
      ),
    'init_obfuscated_data' : IDL.Func(
        [ObfuscatedDataCore, IDL.Text],
        [
          IDL.Variant({
            'error' : ErrorResponse,
            'success' : EmptySuccessResponse,
          }),
        ],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
