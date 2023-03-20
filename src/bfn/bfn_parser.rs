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
