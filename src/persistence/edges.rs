use std::{
    collections::HashMap, f32::consts::E, fs::OpenOptions, hash::Hash, io::{self, BufWriter, Read, Write}
};

use crate::graph::{edge::Edge, node::{
    Node, NodeId, properties::property_map::{PropertyMap, PropertyValue}
}};

use crate::persistence::Store;

impl Store {
    pub fn write_edge(&self, src: NodeId, dst: NodeId, label: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.edge_data)?;

        let mut writer = BufWriter::new(file);

        let mut buf = Vec::new();

        buf.extend_from_slice(&src.to_le_bytes());
        buf.extend_from_slice(&dst.to_le_bytes());

        let label_bytes = label.as_bytes();
        let len = label_bytes.len() as u64;
        buf.extend_from_slice(&len.to_le_bytes());
        buf.extend_from_slice(label_bytes);

        let total_len = buf.len() as u64;

        writer.write_all(&total_len.to_le_bytes())?;
        writer.write_all(&buf)?;
        writer.flush()?;

        Ok(())
    }

    pub fn read_edges(&self) -> io::Result<Vec<Edge>> {
        let file = match OpenOptions::new().read(true).open(&self.edge_data) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(vec![]),
            Err(e) => return Err(e),
        };

        let mut reader = io::BufReader::new(file);
        let mut edges: Vec<Edge> = Vec::new();

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

            let src = NodeId::from_le_bytes(buf[0..8].try_into().unwrap());
            let dst = NodeId::from_le_bytes(buf[8..16].try_into().unwrap());

            let label_len =
                u64::from_le_bytes(buf[16..24].try_into().unwrap()) as usize;

            let label = String::from_utf8(buf[24..24 + label_len].to_vec()).unwrap();

            edges.push(Edge::new(src, dst, label));
        }

        Ok(edges)
    }
}