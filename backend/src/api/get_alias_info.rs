use crate::{AliasInfo, FileMetadata, GetAliasInfoError, State};

pub fn get_alias_info(state: &State, alias: String) -> Result<AliasInfo, GetAliasInfoError> {
    state
        .file_alias_index
        .get(&alias)
        .ok_or(GetAliasInfoError::NotFound)
        .map(|file_id| {
            let file_metadata = get_file_metadata(state, *file_id);
            AliasInfo {
                file_id: *file_id,
                file_name: file_metadata.file_name.clone(),
                user: state
                    .users
                    .get(&file_metadata.requester_principal)
                    .unwrap()
                    .clone(),
            }
        })
}

fn get_file_metadata(state: &State, file_id: u64) -> &FileMetadata {
    &state.file_data.get(&file_id).unwrap().metadata
}
