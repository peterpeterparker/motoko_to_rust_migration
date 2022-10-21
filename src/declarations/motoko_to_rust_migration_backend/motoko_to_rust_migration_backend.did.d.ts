import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'init' : ActorMethod<[], Principal>,
  'installCode' : ActorMethod<[Array<number>], Principal>,
}
