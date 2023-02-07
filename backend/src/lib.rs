mod aliases;
use crate::aliases::{AliasGenerator, Randomness};
use ic_cdk::export::candid::CandidType;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum WhoamiResponse {
    #[serde(rename = "known_user")]
    KnownUser(User),
    #[serde(rename = "unknown_user")]
    UnknownUser,
}

/// File metadata.
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct FileMetadata {
    pub file_id: u64,
    pub file_name: String,
}

// A file is composed of its metadata and its content, which is a blob.
pub struct File {
    #[allow(dead_code)]
    pub metadata: FileMetadata,
    pub contents: Option<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum FileData {
    #[serde(rename = "not_found_file")]
    NotFoundFile,
    #[serde(rename = "not_uploaded_file")]
    NotUploadedFile,
    #[serde(rename = "found_file")]
    FoundFile(Vec<u8>),
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum UploadFileResponse {
    #[serde(rename = "not_requested_file")]
    NotRequestedFile,
    #[serde(rename = "already_uploaded_file")]
    AlreadyUploadedFile,
    #[serde(rename = "upload_ok")]
    UploadOk,
}

pub struct State {
    /// Keeps track of how many files have been requested so far
    /// and is used to assign IDs to newly requested files.
    pub file_count: u64,
    /// Keeps track of usernames vs. their principals.
    pub users: BTreeMap<ic_cdk::export::Principal, User>,
    /// Mapping between file IDs and file information.
    pub file_data: BTreeMap<u64, File>,
    /// Mapping between file aliases (randomly generated links) and file metadata.
    pub file_alias_index: BTreeMap<String, FileMetadata>,

    // Generates aliases for file requests.
    alias_generator: AliasGenerator,
}

impl Default for State {
    fn default() -> Self {
        Self {
            file_count: 0,
            users: BTreeMap::new(),
            file_data: BTreeMap::new(),
            file_alias_index: BTreeMap::new(),
            alias_generator: AliasGenerator::new(
                Randomness::try_from(vec![0; 32].as_slice()).unwrap(),
            ),
        }
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