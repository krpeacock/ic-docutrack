mod aliases;
pub mod api;
use crate::aliases::{AliasGenerator, Randomness};
use ic_cdk::export::{candid::CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

fn get_randomness_seed() -> Vec<u8> {
    // this is an array of u8 of length 8.
    let time_seed = ic_cdk::api::time().to_be_bytes();
    // we need to extend this to an array of size 32 by adding to it an array of size 24 full of 0s.
    let zeroes_arr: [u8; 24] = [0; 24];
    [&time_seed[..], &zeroes_arr[..]].concat()
}

thread_local! {
    /// Initialize the state randomness with the current time.
    static STATE: RefCell<State> = RefCell::new(State::new(&get_randomness_seed()[..]));
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct User {
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "public_key")]
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum WhoamiResponse {
    #[serde(rename = "known_user")]
    KnownUser(User),
    #[serde(rename = "unknown_user")]
    UnknownUser,
}

/// File metadata.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileMetadata {
    pub file_name: String,
    pub user_public_key: Vec<u8>,
    pub requester_principal: Principal,
    pub requested_at: u64,
    pub uploaded_at: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FileStatus {
    #[serde(rename = "pending")]
    Pending { alias: String, requested_at: u64 },
    #[serde(rename = "uploaded")]
    Uploaded { uploaded_at: u64 },
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PublicFileMetadata {
    pub file_id: u64,
    pub file_name: String,
    pub file_status: FileStatus,
    pub shared_with: Vec<User>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GetAliasInfoError {
    #[serde(rename = "not_found")]
    NotFound,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AliasInfo {
    pub file_id: u64,
    pub file_name: String,
    pub user: User,
}

// A file is composed of its metadata and its content, which is a blob.
#[derive(Debug, PartialEq, Eq)]
pub struct File {
    pub metadata: FileMetadata,
    pub content: FileContent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileContent {
    Pending {
        alias: String,
    },
    Uploaded {
        contents: Vec<u8>,
        file_type: String,
        owner_key: Vec<u8>,
        shared_keys: BTreeMap<Principal, Vec<u8>>,
    },
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq)]
pub struct FileData {
    contents: Vec<u8>,
    file_type: String,
    owner_key: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, PartialEq, Debug)]
pub enum FileDownloadResponse {
    #[serde(rename = "not_found_file")]
    NotFoundFile,
    #[serde(rename = "not_uploaded_file")]
    NotUploadedFile,
    #[serde(rename = "permission_error")]
    PermissionError,
    #[serde(rename = "found_file")]
    FoundFile(FileData),
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum UploadFileError {
    #[serde(rename = "not_requested")]
    NotRequested,
    #[serde(rename = "already_uploaded")]
    AlreadyUploaded,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq)]
pub enum FileSharingResponse {
    #[serde(rename = "pending_error")]
    PendingError,
    #[serde(rename = "permission_error")]
    PermissionError,
    #[serde(rename = "ok")]
    Ok,
}

pub struct State {
    // Keeps track of how many files have been requested so far
    // and is used to assign IDs to newly requested files.
    file_count: u64,

    /// Keeps track of usernames vs. their principals.
    pub users: BTreeMap<Principal, User>,

    /// Mapping between file IDs and file information.
    pub file_data: BTreeMap<u64, File>,

    /// Mapping between file aliases (randomly generated links) and file ID.
    pub file_alias_index: BTreeMap<String, u64>,

    /// Mapping between a user's principal and the list of files that are owned by the user.
    pub file_owners: BTreeMap<Principal, Vec<u64>>,

    /// Mapping between a user's principal and the list of files that are shared with them.
    pub file_shares: BTreeMap<Principal, Vec<u64>>,

    // Generates aliases for file requests.
    alias_generator: AliasGenerator,
}

impl State {
    pub(crate) fn generate_file_id(&mut self) -> u64 {
        // The file ID is an auto-incrementing integer.

        let file_id = self.file_count;
        self.file_count += 1;
        file_id
    }

    fn new(rand_seed: &[u8]) -> Self {
        Self {
            file_count: 0,
            users: BTreeMap::new(),
            file_data: BTreeMap::new(),
            file_alias_index: BTreeMap::new(),
            file_owners: BTreeMap::new(),
            file_shares: BTreeMap::new(),
            alias_generator: AliasGenerator::new(Randomness::try_from(rand_seed).unwrap()),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::new(vec![0; 32].as_slice())
    }
}

/// A helper method to read the state.
///
/// Precondition: the state is already initialized.
pub fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

/// A helper method to mutate the state.
///
/// Precondition: the state is already initialized.
pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

/// Returns an unused file alias.
pub fn generate_alias() -> String {
    with_state_mut(|s| s.alias_generator.next())
}

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserData {
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "public_key")]
    pub public_key: Vec<u8>,
    #[serde(rename = "ic_principal")]
    pub ic_principal: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GetUsersResponse {
    #[serde(rename = "permission_error")]
    PermissionError,
    #[serde(rename = "users")]
    Users(Vec<UserData>),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UploadFileRequest {
    pub file_id: u64,
    pub file_content: Vec<u8>,
    pub file_type: String,
    pub owner_key: Vec<u8>,
}

#[cfg(target_arch = "wasm32")]
pub fn get_time() -> u64 {
    ic_cdk::api::time()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_time() -> u64 {
    // This is used only in tests and we need a fixed value we can test against.
    12345
}
