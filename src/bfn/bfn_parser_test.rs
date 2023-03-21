#[cfg(test)]
mod tests {
    const TEST_JSON: &str = r#"{
        "version": "1.0.0",
        "name": "test",
        "defines": [
            {
                "name": "test_define",
                "children": [
                    {
                        "BfnJsonByte":{"name": "test_define_byte", "len": 1}
                    },
                    {
                        "BfnJsonString":{"name": "test_define_string", "len": 2}
                    },
                    {
                        "BfnJsonNumber":{"name": "test_define_number","len": 3}
                    }
                ]
            }
        ],
        "children": [
            {
                "BfnJsonInstance": {"name": "test_instance", "define_name": "test_define"}
            },
            {
                "BfnJsonByte": {"name": "test_byte", "len": 4}
            },
            {
                "BfnJsonString": {"name": "test_string", "len": 5}
            },
            {
                "BfnJsonNumber": {"name": "test_number", "len": 6}
            }
        ]
    }"#;

    const TEST_BYTES: &[u8] = &[
        0x01, 0x61, 0x62, 0x01, 0x02, 0x03, 0x01, 0x02, 0x03, 0x04, 0x31, 0x32, 0x33, 0x34, 0x35,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
    ];

    const TEST_BITMAP_JSON: &str = r#"{
        "version": "0",
        "name": "BitmapFormat",
        "defines": [
            {
                "name": "FileHeader",
                "children": [
                    {
                        "BfnJsonByte": {
                            "name": "FileType",
                            "len": 2
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "FileSize",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "Reserved",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "DataOffset",
                            "len": 4
                        }
                    }
                ]
            },
            {
                "name": "InfoHeader",
                "children": [
                    {
                        "BfnJsonNumber": {
                            "name": "InfoHeaderSize",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "ImageWidth",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "ImageHeight",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "Planes",
                            "len": 2
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "BitCount",
                            "len": 2
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "Compression",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "ImageSize",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "XPixelPerMeter",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "YPixelPerMeter",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "ColorUsed",
                            "len": 4
                        }
                    },
                    {
                        "BfnJsonNumber": {
                            "name": "ColorImportant",
                            "len": 4
                        }
                    }
                ]
            }
        ],
        "children": [
            {
                "BfnJsonInstance": {
                    "name": "FileHeader",
                    "define_name": "FileHeader"
                }
            },
            {
                "BfnJsonInstance": {
                    "name": "InfoHeader",
                    "define_name": "InfoHeader"
                }
            },
            {
                "BfnJsonAnchorLenMultipleByte": {
                    "name": "ColorPalette",
                    "len": "ColorUsed",
                    "multiple_num": 4
                }
            },
            {
                "BfnJsonAnchorLenMultipleByte": {
                    "name": "ImageData",
                    "len": "ImageSize",
                    "multiple_num": 1
                }
            }
        ]
    }"#;

    const TEST_BITMAP_BYTES: &[u8] = &[
        0x42, 0x4d, 0x7e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x28,
        0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xfe, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00, 0x40,
        0x02, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00,
        0x40, 0x02, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00, 0x40, 0x02, 0x00,
        0x00, 0x40, 0x02, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00, 0x40, 0x02, 0x00, 0x00, 0x7f, 0xfe,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn parse_json_test() {
        let parsed = crate::bfn::bfn_parser::parse_json(TEST_JSON).unwrap();

        assert_eq!(parsed.version, "1.0.0");
        assert_eq!(parsed.name.unwrap(), "test");
        assert_eq!(parsed.defines.unwrap().len(), 1);
        assert_eq!(parsed.children.len(), 6);
    }

    #[test]
    fn convert_binary_to_bfn_visualize_pair_test() {
        let parsed = crate::bfn::bfn_parser::parse_json(TEST_JSON).unwrap();

        let pair =
            crate::bfn::bfn_parser::convert_binary_to_bfn_visualize_pair(TEST_BYTES, &parsed);
        assert_eq!(pair[0].0, "test_define_byte");
        assert_eq!(pair[0].1, "01");

        assert_eq!(pair[1].0, "test_define_string");
        assert_eq!(pair[1].1, "ab");
    }

    #[test]
    fn bitmap_format_test() {
        let parsed = crate::bfn::bfn_parser::parse_json(TEST_BITMAP_JSON).unwrap();

        let pair = crate::bfn::bfn_parser::convert_binary_to_bfn_visualize_pair(
            TEST_BITMAP_BYTES,
            &parsed,
        );

        assert_eq!(pair[0].0, "FileType");
    }
}
