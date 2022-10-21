import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Asset { 'key' : string, 'value' : Array<number> }
export type Assets = Array<[string, Asset]>;
export interface State { 'owner' : string, 'assets' : Assets }
export interface _SERVICE { 'get' : ActorMethod<[], State> }
