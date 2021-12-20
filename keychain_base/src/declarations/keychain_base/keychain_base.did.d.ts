import type { Principal } from '@dfinity/principal';
export interface EmptySuccessResponse { 'code' : number }
export interface ErrorResponse {
  'error_message' : string,
  'error_code' : number,
}
export interface GetKeyDataSuccessResponse {
  'code' : number,
  'data' : [] | [KeyDataCore],
}
export interface GetObfuscatedDataSuccessResponse {
  'code' : number,
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
      { 'Error' : ErrorResponse } |
        { 'Success' : EmptySuccessResponse }
    >,
  'get_keydata' : (arg_0: string) => Promise<
      { 'Error' : ErrorResponse } |
        { 'Success' : GetKeyDataSuccessResponse }
    >,
  'get_obfuscated_data' : (arg_0: string) => Promise<
      { 'Error' : ErrorResponse } |
        { 'Success' : GetObfuscatedDataSuccessResponse }
    >,
  'init_keydata' : (arg_0: KeyDataCore, arg_1: string) => Promise<
      { 'Error' : ErrorResponse } |
        { 'Success' : EmptySuccessResponse }
    >,
  'init_obfuscated_data' : (
      arg_0: ObfuscatedDataCore,
      arg_1: string,
    ) => Promise<
      { 'Error' : ErrorResponse } |
        { 'Success' : EmptySuccessResponse }
    >,
}
