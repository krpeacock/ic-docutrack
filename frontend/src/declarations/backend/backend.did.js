export const idlFactory = ({ IDL }) => {
  const file_id = IDL.Nat64;
  const found_file = IDL.Record({
    'contents' : IDL.Vec(IDL.Nat8),
    'owner_key' : IDL.Vec(IDL.Nat8),
    'file_type' : IDL.Text,
  });
  const download_file_response = IDL.Variant({
    'found_file' : found_file,
    'permission_error' : IDL.Null,
    'not_uploaded_file' : IDL.Null,
    'not_found_file' : IDL.Null,
  });
  const user = IDL.Record({
    'public_key' : IDL.Vec(IDL.Nat8),
    'first_name' : IDL.Text,
    'last_name' : IDL.Text,
  });
  const get_alias_info_response = IDL.Variant({
    'Ok' : IDL.Record({
      'user' : user,
      'file_name' : IDL.Text,
      'file_id' : file_id,
    }),
    'Err' : IDL.Variant({ 'not_found' : IDL.Null }),
  });
  const file_status = IDL.Variant({
    'pending' : IDL.Record({ 'alias' : IDL.Text, 'requested_at' : IDL.Nat64 }),
    'uploaded' : IDL.Record({ 'uploaded_at' : IDL.Nat64 }),
  });
  const file_metadata = IDL.Record({
    'file_status' : file_status,
    'file_name' : IDL.Text,
    'shared_with' : IDL.Vec(user),
    'file_id' : file_id,
  });
  const user_data = IDL.Record({
    'public_key' : IDL.Vec(IDL.Nat8),
    'ic_principal' : IDL.Principal,
    'first_name' : IDL.Text,
    'last_name' : IDL.Text,
  });
  const get_users_response = IDL.Variant({
    'permission_error' : IDL.Null,
    'users' : IDL.Vec(user_data),
  });
  const share_file_response = IDL.Variant({
    'ok' : IDL.Null,
    'permission_error' : IDL.Null,
  });
  const upload_file_request = IDL.Record({
    'owner_key' : IDL.Vec(IDL.Nat8),
    'file_type' : IDL.Text,
    'file_content' : IDL.Vec(IDL.Nat8),
    'file_id' : file_id,
  });
  const upload_file_error = IDL.Variant({
    'not_requested' : IDL.Null,
    'already_uploaded' : IDL.Null,
  });
  const upload_file_response = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : upload_file_error,
  });
  const upload_file_atomic_request = IDL.Record({
    'content' : IDL.Vec(IDL.Nat8),
    'owner_key' : IDL.Vec(IDL.Nat8),
    'name' : IDL.Text,
  });
  const who_am_i_response = IDL.Variant({
    'known_user' : IDL.Record({
      'first_name' : IDL.Text,
      'last_name' : IDL.Text,
    }),
    'unknown_user' : IDL.Null,
  });
  return IDL.Service({
    'download_file' : IDL.Func([file_id], [download_file_response], []),
    'get_alias_info' : IDL.Func([IDL.Text], [get_alias_info_response], []),
    'get_requests' : IDL.Func([], [IDL.Vec(file_metadata)], []),
    'get_shared_files' : IDL.Func([], [IDL.Vec(file_metadata)], []),
    'get_users' : IDL.Func([], [get_users_response], []),
    'hello_world' : IDL.Func([], [IDL.Text], []),
    'request_file' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_user' : IDL.Func([user], [], []),
    'share_file' : IDL.Func(
        [IDL.Principal, file_id, IDL.Vec(IDL.Nat8)],
        [share_file_response],
        [],
      ),
    'upload_file' : IDL.Func([upload_file_request], [upload_file_response], []),
    'upload_file_atomic' : IDL.Func([upload_file_atomic_request], [], []),
    'who_am_i' : IDL.Func([], [who_am_i_response], []),
  });
};
export const init = ({ IDL }) => { return []; };
