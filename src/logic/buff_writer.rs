use crate::logic::encoding_binary_tree::huff_base_node::HuffBaseNode;
use std::io::{BufWriter, Write};

pub struct BuffWriter {
    file: BufWriter<std::fs::File>,
}

impl BuffWriter {
    pub fn new(file_name: &str) -> std::io::Result<Self> {
        Ok(Self {
            file: BufWriter::new(std::fs::File::create(file_name)?),
        })
    }

    pub fn serialize(&mut self, node: &dyn HuffBaseNode, bytes: &mut Vec<u8>) {
        if node.is_leaf() {
            bytes.push(1);
            bytes.extend_from_slice(
                node.character()
                    .unwrap()
                    .encode_utf8(&mut [0; 4])
                    .as_bytes(),
            );
        } else {
            bytes.push(0);
            self.serialize(node.left().unwrap(), bytes);
            self.serialize(node.right().unwrap(), bytes);
        }
        

    }

   pub fn write_header(&mut self, bytes: &mut Vec<u8>) -> std::io::Result<()> {
        let h_len = bytes.len() as u32;

        self.file.write_all(&h_len.to_le_bytes())?;

        self.file.write_all(bytes)?;

        Ok(())
    }
}
