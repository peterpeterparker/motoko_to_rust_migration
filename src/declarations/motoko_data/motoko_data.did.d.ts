import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Asset { 'key' : string, 'value' : Array<number> }
export interface DataBucket {
  'get' : ActorMethod<
    [],
    { 'assets' : Array<[string, Asset]>, 'user' : string },
  >,
  'put' : ActorMethod<[string, string], undefined>,
}
export interface _SERVICE extends DataBucket {}
