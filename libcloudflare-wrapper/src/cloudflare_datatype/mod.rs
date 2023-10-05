pub mod records;
pub mod zones;
mod objresponse_impl;

use reqwest::{blocking::Client, header::HeaderMap};
use serde::Deserialize;
use zones::ZoneRes;
use records::RecordsRes;

#[derive(Deserialize, Debug, Default)]
#[serde(untagged)]
pub enum ObjResult {
    ZonesData(Vec<ZoneRes>),
    DNSRecords(Vec<RecordsRes>),
    #[default]
    None,
    ZoneData(ZoneRes),
    DNSRecord(RecordsRes)
}

#[derive(Deserialize, Debug, Default)]
pub struct ObjMsg {
    pub code: i16,
    pub message: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct ObjErr(ObjMsg);

#[derive(Deserialize, Debug, Default)]
pub struct ObjPageDetail {
    pub count: u8,
    pub page: u8,
    pub per_page: u16,
    pub total_count: u8,
}

#[derive(Deserialize, Debug, Default)]
pub struct ObjResponse {
    pub errors: Vec<ObjErr>,
    pub messages: Vec<ObjMsg>,
    pub success: bool,
    pub result_info: ObjPageDetail,
    pub result: Option<ObjResult>,
}

