import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Assets { 'key' : string, 'value' : Array<number> }
export interface State { 'owner' : string, 'assets' : Assets }
export interface _SERVICE { 'get' : ActorMethod<[], State> }
