// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

// Insert std prelude in the top for the sgx feature
#[cfg(feature = "mesalock_sgx")]
use std::prelude::v1::*;

#[cfg(not(feature = "mesalock_sgx"))]
use crate::key::AeadConfig;
#[cfg(feature = "mesalock_sgx")]
use kms_proto::proto::AeadConfig;

use serde_derive::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DFSRequest {
    Create(CreateFileRequest),
    Get(GetFileRequest),
    List(ListFileRequest),
    Delete(DeleteFileRequest),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DFSResponse {
    Create(CreateFileResponse),
    Get(GetFileResponse),
    List(ListFileResponse),
    Delete(DeleteFileResponse),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateFileRequest {
    pub file_name: String,
    pub sha256: String,
    pub file_size: u32,
    pub user_id: String,
    pub user_token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateFileResponse {
    pub file_id: String,
    pub access_path: String,
    pub key_config: AeadConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetFileRequest {
    pub file_id: String,
    pub user_id: String,
    pub user_token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetFileResponse {
    pub file_info: FileInfo,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DeleteFileRequest {
    pub file_id: String,
    pub user_id: String,
    pub user_token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DeleteFileResponse {
    pub file_info: FileInfo,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub user_id: String,
    pub file_name: String,
    pub sha256: String,
    pub file_size: u32,
    pub access_path: String,
    pub task_id: Option<String>,
    pub collaborator_list: Vec<String>,
    pub key_config: AeadConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ListFileRequest {
    pub user_id: String,
    pub user_token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ListFileResponse {
    pub list: Vec<String>,
}

impl DFSRequest {
    pub fn new_create_file(
        file_name: &str,
        sha256: &str,
        file_size: u32,
        user_id: &str,
        user_token: &str,
    ) -> DFSRequest {
        DFSRequest::Create(CreateFileRequest {
            file_name: file_name.to_owned(),
            sha256: sha256.to_owned(),
            file_size,
            user_id: user_id.to_owned(),
            user_token: user_token.to_owned(),
        })
    }

    pub fn new_get_file(file_id: &str, user_id: &str, user_token: &str) -> DFSRequest {
        DFSRequest::Get(GetFileRequest {
            file_id: file_id.to_owned(),
            user_id: user_id.to_owned(),
            user_token: user_token.to_owned(),
        })
    }

    pub fn new_list_file(user_id: &str, user_token: &str) -> DFSRequest {
        DFSRequest::List(ListFileRequest {
            user_id: user_id.to_owned(),
            user_token: user_token.to_owned(),
        })
    }

    pub fn new_del_file(file_id: &str, user_id: &str, user_token: &str) -> DFSRequest {
        DFSRequest::Delete(DeleteFileRequest {
            file_id: file_id.to_owned(),
            user_id: user_id.to_owned(),
            user_token: user_token.to_owned(),
        })
    }
}

impl DFSResponse {
    pub fn new_create_file(
        file_id: &str,
        access_path: &str,
        key_config: &AeadConfig,
    ) -> DFSResponse {
        DFSResponse::Create(CreateFileResponse {
            file_id: file_id.to_owned(),
            access_path: access_path.to_owned(),
            key_config: key_config.clone(),
        })
    }

    pub fn new_get_file(file_info: &FileInfo) -> DFSResponse {
        DFSResponse::Get(GetFileResponse {
            file_info: file_info.clone(),
        })
    }

    pub fn new_list_file(list: &[&str]) -> DFSResponse {
        DFSResponse::List(ListFileResponse {
            list: list.iter().map(|s| s.to_string()).collect(),
        })
    }

    pub fn new_del_file(file_info: &FileInfo) -> DFSResponse {
        DFSResponse::Delete(DeleteFileResponse {
            file_info: file_info.clone(),
        })
    }
}
