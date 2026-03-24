use std::{
    collections::HashMap, f32::consts::E, fs::OpenOptions, hash::Hash, io::{self, BufWriter, Read, Write}
};

use crate::graph::{edge::Edge, node::{
    Node, NodeId, properties::property_map::{PropertyMap, PropertyValue}
}};

use crate::persistence::Store;

impl Store {
    pub fn write_node(&self, id: u64, label: &str, properties: &PropertyMap) -> io::Result<()> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.node_data)?;

        let mut writer = BufWriter::new(file);

        let mut buf = Vec::new();

        buf.extend_from_slice(&id.to_le_bytes());

        let label_bytes = label.as_bytes();
        let len = label_bytes.len() as u64;
        buf.extend_from_slice(&len.to_le_bytes());
        buf.extend_from_slice(label_bytes);

        let prop_count = properties.len() as u64;
        buf.extend_from_slice(&prop_count.to_le_bytes());

        for (k, v) in properties {
            let k_bytes = k.as_bytes();
            let k_len = k_bytes.len() as u64;
            buf.extend_from_slice(&k_len.to_le_bytes());
            buf.extend_from_slice(k_bytes);

            match v {
                PropertyValue::Bool(b) => {
                    buf.push(0);
                    buf.push(*b as u8);
                }
                PropertyValue::Int(i) => {
                    buf.push(1);
                    buf.extend_from_slice(&i.to_le_bytes());
                }
                PropertyValue::Str(s) => {
                    buf.push(2);
                    let s_bytes = s.as_bytes();
                    let s_len = s_bytes.len() as u64;
                    buf.extend_from_slice(&s_len.to_le_bytes());
                    buf.extend_from_slice(s_bytes);
                }
            }
        }

        let total_len = buf.len() as u64;

        writer.write_all(&total_len.to_le_bytes())?;
        writer.write_all(&buf)?;
        writer.flush()?;

        Ok(())
    }

    pub fn read_nodes(&self) -> io::Result<Vec<Node>> {
        let file = match OpenOptions::new().read(true).open(&self.node_data) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(vec![]),
            Err(e) => return Err(e),
        };

        let mut reader = io::BufReader::new(file);
        let mut nodes = Vec::new();

        loop {
            let mut len_buf = [0u8; 8];

            match reader.read_exact(&mut len_buf) {
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }

            let total_len = u64::from_le_bytes(len_buf) as usize;

            let mut buf = vec![0u8; total_len];
            reader.read_exact(&mut buf)?;

            let mut cursor = 0;

            let id = u64::from_le_bytes(buf[cursor..cursor + 8].try_into().unwrap());
            cursor += 8;

            let label_len =
                u64::from_le_bytes(buf[cursor..cursor + 8].try_into().unwrap()) as usize;
            cursor += 8;

            let label = String::from_utf8(buf[cursor..cursor + label_len].to_vec()).unwrap();
            cursor += label_len;

            let prop_count =
                u64::from_le_bytes(buf[cursor..cursor + 8].try_into().unwrap()) as usize;
            cursor += 8;

            let mut property_map = HashMap::new();

            for _ in 0..prop_count {
                let k_len =
                    u64::from_le_bytes(buf[cursor..cursor + 8].try_into().unwrap()) as usize;
                cursor += 8;

                let k = String::from_utf8(buf[cursor..cursor + k_len].to_vec()).unwrap();
                cursor += k_len;

                let v_type = buf[cursor];
                cursor += 1;

                let v = match v_type {
                    0 => {
                        let b = buf[cursor] != 0;
                        cursor += 1;
                        PropertyValue::Bool(b)
                    }
                    1 => {
                        let i = i32::from_le_bytes(buf[cursor..cursor + 4].try_into().unwrap());
                        cursor += 4;
                        PropertyValue::Int(i)
                    }
                    2 => {
                        let s_len = u64::from_le_bytes(buf[cursor..cursor + 8].try_into().unwrap())
                            as usize;
                        cursor += 8;
                        let s = String::from_utf8(buf[cursor..cursor + s_len].to_vec()).unwrap();
                        cursor += s_len;
                        PropertyValue::Str(s)
                    }
                    _ => panic!("Unknown property type"),
                };

                property_map.insert(k, v);
            }

            nodes.push(Node {
                id,
                label,
                property_map,
            });
        }

        Ok(nodes)
    }
}
