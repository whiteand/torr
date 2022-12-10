use crate::bencoding::value::Value;
use std::collections::HashMap;

pub mod keys;
pub mod read;

#[derive(Debug)]
pub enum FileTree {}

#[derive(Debug)]
pub struct Info {
    pub name: String,
    pub piece_length: u64,
    pub meta_version: u64,
    pub file_tree: FileTree,
    pub length: u64,
    pub pieces_root: Vec<u8>,
    pub private: Option<u64>,
}

#[derive(Debug)]
pub struct MetaInfo {
    pub announce: String,
    pub info: Info,
    pub piece_layers: HashMap<String, Value>,
}
