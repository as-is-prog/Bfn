use serde_json;

use super::bfn_data::{BfnJsonDefine, BfnJsonRoot, BfnJsonValue};

pub fn parse_json(json: &str) -> Result<BfnJsonRoot, serde_json::Error> {
    let parsed: Result<BfnJsonRoot, serde_json::Error> = serde_json::from_str(&json);

    return match parsed {
        Ok(data) => {
            let converted = convert_define_to_instance(data);
            Ok(converted)
        }
        Err(e) => Err(e),
    };
}

fn convert_define_to_instance(root: BfnJsonRoot) -> BfnJsonRoot {
    let mut ret_children: Vec<BfnJsonValue> = Vec::new();

    let defines: Vec<BfnJsonDefine> = root.defines.unwrap().clone();

    for child in &root.children {
        match child {
            BfnJsonValue::BfnJsonInstance {
                name: _,
                define_name,
            } => {
                let define: &BfnJsonDefine =
                    defines.iter().find(|x| &x.name == define_name).unwrap();

                let mut def_children = define.children.clone();
                ret_children.append(&mut def_children);
            }
            _ => {
                ret_children.push(child.clone());
            }
        }
    }

    return BfnJsonRoot {
        version: root.version,
        name: root.name,
        defines: Some(defines),
        children: ret_children,
    };
}

pub fn convert_binary_to_bfn_visualize_pair(
    bin_data: &[u8],
    bfn_root: &BfnJsonRoot,
) -> Vec<(String, String)> {
    let mut ret: Vec<(String, String)> = Vec::new();
    let mut anchor_pair: Vec<(String, i32)> = Vec::new();

    let mut bin_data_index = 0;

    for child in &bfn_root.children {
        match child {
            BfnJsonValue::BfnJsonInstance {
                name: _,
                define_name: _,
            } => panic!("BfnJsonInstance is not allowed in convert_binary_to_bfn_visualize_pair"),
            BfnJsonValue::BfnJsonByte { name, len } => {
                let mut value = String::new();
                for _ in 0..*len {
                    value.push_str(&format!("{:02x}", bin_data[bin_data_index]));
                    bin_data_index += 1;
                }
                ret.push((name.clone(), value));
            }
            BfnJsonValue::BfnJsonString { name, len } => {
                let mut value = String::new();
                for _ in 0..*len {
                    value.push_str(&format!("{}", bin_data[bin_data_index] as char));
                    bin_data_index += 1;
                }
                ret.push((name.clone(), value));
            }
            BfnJsonValue::BfnJsonNumber { name, len } => {
                let mut value = 0;
                for i in 0..*len {
                    value += (bin_data[bin_data_index] as i32) << (i * 8);
                    bin_data_index += 1;
                }
                ret.push((name.clone(), value.to_string()));
                anchor_pair.push((name.clone(), value));
            }
            BfnJsonValue::BfnJsonAnchorLenByte { name, len } => {
                let anchor_len = find_anchor_len(len, &anchor_pair);

                let mut value = String::new();
                for _ in 0..anchor_len {
                    value.push_str(&format!("{:02x}", bin_data[bin_data_index]));
                    bin_data_index += 1;
                }
                ret.push((name.clone(), value));
            }
            BfnJsonValue::BfnJsonAnchorLenMultipleByte {
                name,
                multiple_num,
                len,
            } => {
                let anchor_len = find_anchor_len(len, &anchor_pair) * multiple_num;

                let mut value = String::new();
                for _ in 0..anchor_len {
                    value.push_str(&format!("{:02x}", bin_data[bin_data_index]));
                    bin_data_index += 1;
                }
                ret.push((name.clone(), value));
            }
            BfnJsonValue::BfnJsonAnchorLenString { name, len } => {
                let anchor_len = find_anchor_len(len, &anchor_pair);

                let mut value = String::new();
                for _ in 0..anchor_len {
                    value.push_str(&format!("{}", bin_data[bin_data_index] as char));
                    bin_data_index += 1;
                }
                ret.push((name.clone(), value));
            }
            BfnJsonValue::BfnJsonAnchorLenNumber { name, len } => {
                let anchor_len = find_anchor_len(len, &anchor_pair);

                let mut value = 0;
                for i in 0..anchor_len {
                    value += (bin_data[bin_data_index] as i32) << (i * 8);
                    bin_data_index += 1;
                }
                ret.push((name.clone(), value.to_string()));
                anchor_pair.push((name.clone(), value));
            }
        }
    }

    return ret;
}

fn find_anchor_len(name: &str, pairs: &Vec<(String, i32)>) -> i32 {
    return pairs.iter().find(|x| &x.0 == name).unwrap().1;
}
