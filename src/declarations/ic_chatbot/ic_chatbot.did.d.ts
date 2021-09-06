import type { Principal } from '@dfinity/principal';
export type JsonText = string;
export type SessionId = string;
export type UserInput = string;
export interface _SERVICE {
  'get_next_block' : (arg_0: SessionId, arg_1: UserInput) => Promise<JsonText>,
  'init_session' : () => Promise<JsonText>,
}
