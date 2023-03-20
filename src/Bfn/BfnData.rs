use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BfnJsonRoot {
    pub version: String,
    pub name: Option<String>,
    pub defines: Option<Vec<BfnJsonDefine>>,
    pub children: Vec<BfnJsonValue>,
}

#[derive(Serialize, Deserialize)]
pub struct BfnJsonDefine {
    pub name: String,
    pub children: Vec<BfnJsonValue>,
}

#[derive(Serialize, Deserialize)]
pub enum BfnJsonValue {
    BfnJsonInstance {
        name: String,
        define_name: String,
    },
    BfnJsonByte {
        name: String,
        len: i32,
    },
    BfnJsonString {
        name: String,
        len: i32,
    },
    BfnJsonNumber {
        name: String,
        len: i32,
    },

    BfnJsonAnchorLenByte {
        name: String,
        len: String,
    },
    BfnJsonAnchorLenMultipleByte {
        name: String,
        multiple_num: i32,
        len: String,
    },
    BfnJsonAnchorLenString {
        name: String,
        len: String,
    },
    BfnJsonAnchorLenNumber {
        name: String,
        len: String,
    },
}

pub struct BfnData {}
