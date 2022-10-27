// Non snake case for backwards compatibility
#![allow(non_snake_case)]

pub mod storage {
    use std::collections::HashMap;
    use candid::{Principal, CandidType};
    use serde::{Deserialize, Serialize};
    use std::clone::Clone;
    use crate::types::http::HeaderField;

    // Internal types

    pub type Batches = HashMap<u128, Batch>;
    pub type Chunks = HashMap<u128, Chunk>;
    pub type Assets = HashMap<String, Asset>;

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct State {
        pub stable: StableState,
        pub runtime: RuntimeState,
    }

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct StableState {
        pub user: Option<Principal>,
        pub assets: Assets,
    }

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct RuntimeState {
        pub chunks: Chunks,
        pub batches: Batches,
    }

    // Exposed types

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Chunk {
        pub batchId: u128,
        pub content: Vec<u8>,
    }

    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct AssetEncoding {
        pub modified: u64,
        pub contentChunks: Vec<Vec<u8>>,
        pub totalLength: u128,
    }

    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct AssetKey {
        // myimage.jpg
        pub name: String,
        // images
        pub folder: String,
        // /images/myimage.jpg
        pub fullPath: String,
        // ?token=1223-3345-5564-3333
        pub token: Option<String>,
        // The sha256 representation of the content
        pub sha256: Option<Vec<u8>>,
    }

    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct Asset {
        pub key: AssetKey,
        pub headers: Vec<HeaderField>,
        pub encoding: AssetEncoding,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Batch {
        pub key: AssetKey,
        pub expiresAt: u64,
    }
}

pub mod interface {
    use candid::{CandidType};
    use serde::{Serialize, Deserialize};
    use crate::types::http::HeaderField;

    #[derive(CandidType)]
    pub struct InitUpload {
        pub batchId: u128,
    }

    #[derive(CandidType)]
    pub struct UploadChunk {
        pub chunkId: u128,
    }

    #[derive(CandidType, Deserialize)]
    pub struct CommitBatch {
        pub batchId: u128,
        pub headers: Vec<HeaderField>,
        pub chunkIds: Vec<u128>,
    }

    #[derive(CandidType, Deserialize)]
    pub struct Del {
        pub fullPath: String,
        pub token: Option<String>,
    }
}

pub mod http {
    use candid::{CandidType, Func};
    use serde::{Serialize, Deserialize};

    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct HeaderField(String, String);

    #[derive(CandidType, Deserialize, Clone)]
    pub struct HttpRequest {
        pub url: String,
        pub method: String,
        pub headers: Vec<HeaderField>,
        pub body: Vec<u8>,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct HttpResponse {
        pub body: Vec<u8>,
        pub headers: Vec<HeaderField>,
        pub status_code: u16,
        pub streaming_strategy: Option<StreamingStrategy>,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub enum StreamingStrategy {
        Callback {
            token: StreamingCallbackToken,
            callback: Func,
        },
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct StreamingCallbackToken {
        pub fullPath: String,
        pub token: Option<String>,
        pub headers: Vec<HeaderField>,
        pub sha256: Option<Vec<u8>>,
        pub index: usize,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct StreamingCallbackHttpResponse {
        pub body: Vec<u8>,
        pub token: Option<StreamingCallbackToken>,
    }
}

// TODO: delete after migration from Motoko to Rust

pub mod migration {
    use candid::{CandidType, Principal, Int};
    use serde::{Serialize, Deserialize};
    use crate::types::http::HeaderField;
    use crate::types::storage::{Asset, AssetEncoding, AssetKey};

    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct Asset2 {
        pub key: String,
        pub value: Vec<u8>,
    }

    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct Asset3 {
        pub key: AssetKey,
        pub value: Vec<u8>,
    }

    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct Asset4 {
        pub key: AssetKey,
        pub headers: Vec<HeaderField>,
        pub value: Vec<u8>,
    }


    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct AssetEncodingShort {
        pub contentChunks: Vec<Vec<u8>>,
    }

    #[derive(CandidType, Deserialize, Clone, Serialize)]
    pub struct Asset5 {
        pub key: AssetKey,
        pub headers: Vec<HeaderField>,
        pub modified: Int,
        pub contentChunks: Vec<Vec<u8>>,
    }

    #[derive(CandidType, Deserialize, Serialize)]
    pub struct UpgradeState {
        pub test: Option<u128>,
        pub user: Option<Principal>,
        // pub entries: Option<Vec<(String, Asset)>>,
        // pub entriesTest: Option<Vec<(String, Asset)>>,
        pub yolo: Option<Asset2>,
        pub test45678: Option<Vec<Asset2>>,
        pub test456789: Option<Vec<(String, Asset2)>>,
        pub test4567890: Option<Vec<(String, Asset3)>>,
        pub test45678901: Option<Vec<(String, Asset4)>>,
        pub test45678901234567: Option<Vec<(String, Asset5)>>,
    }
}