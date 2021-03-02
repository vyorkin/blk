use crate::{ByteHash, Hashable};
use std::collections::HashSet;

type Address = String;

pub struct Output {
    pub to_adress: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_adress.as_bytes());
        bytes.extend(&self.value.to_be_bytes());
        bytes
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn input_hashes(&self) -> HashSet<Vec<u8>> {
        self.inputs
            .iter()
            .map(|input| input.hash().to_bytes())
            .collect::<HashSet<Vec<u8>>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }

    pub fn input_value(&self) -> u64 {
        self.value(true)
    }

    pub fn output_value(&self) -> u64 {
        self.value(false)
    }

    fn value(&self, is_input: bool) -> u64 {
        let items = if is_input {
            &self.inputs
        } else {
            &self.outputs
        };

        items.iter().map(|item| item.value).sum()
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}
