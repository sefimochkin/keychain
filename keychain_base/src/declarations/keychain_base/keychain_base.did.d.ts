import type { Principal } from '@dfinity/principal';
export interface EmptySuccessResponse { 'code' : bigint }
export interface ErrorResponse {
  'error_message' : string,
  'error_code' : bigint,
}
export interface GetKeyDataSuccessResponse {
  'code' : bigint,
  'data' : [] | [KeyDataCore],
}
export interface GetObfuscatedDataSuccessResponse {
  'code' : bigint,
  'data' : [] | [Array<ObfuscatedDataCore>],
}
export interface KeyDataCore {
  'email_address' : string,
  'address' : [] | [string],
  'phone_number' : [] | [string],
}
export interface ObfuscatedDataCore {
  'obfuscated_phone_number' : [] | [string],
  'website_url' : string,
  'website_id' : [] | [string],
  'password' : [] | [string],
  'obfuscated_email_address' : string,
}
export interface _SERVICE {
  'change_keydata' : (arg_0: KeyDataCore, arg_1: string) => Promise<
      { 'error' : ErrorResponse } |
        { 'success' : EmptySuccessResponse }
    >,
  'get_keydata' : (arg_0: string) => Promise<
      { 'error' : ErrorResponse } |
        { 'success' : GetKeyDataSuccessResponse }
    >,
  'get_obfuscated_data' : (arg_0: string) => Promise<
      { 'error' : ErrorResponse } |
        { 'success' : GetObfuscatedDataSuccessResponse }
    >,
  'init_keydata' : (arg_0: KeyDataCore, arg_1: string) => Promise<
      { 'error' : ErrorResponse } |
        { 'success' : EmptySuccessResponse }
    >,
  'init_obfuscated_data' : (
      arg_0: ObfuscatedDataCore,
      arg_1: string,
    ) => Promise<
      { 'error' : ErrorResponse } |
        { 'success' : EmptySuccessResponse }
    >,
}
