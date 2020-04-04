use std::collections::HashMap;

use failure::Error;

use crate::{error::CodingError, helper::generate_dest_base_encoder};

#[derive(Debug, Default)]
pub struct LZCoding {
    segement: Vec<(u64, u64)>,
    pub prefix_code_length: u64,
    pub letter_code_length: u64,
    prefix_map: HashMap<Vec<char>, u64>,
    pub alphabet_map: HashMap<char, u64>,
}

impl LZCoding {
    pub fn new(alphabet: &[char]) -> Self {
        Self {
            segement: vec![(0, 0)],
            prefix_code_length: 0,
            letter_code_length: 0,
            prefix_map: HashMap::with_capacity(1024),
            alphabet_map: Self::precoding_for_alphabet(alphabet),
        }
    }

    pub fn precoding_for_alphabet(alphabet: &[char]) -> HashMap<char, u64> {
        alphabet
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i as u64))
            .collect()
    }

    pub fn reset(&mut self, alphabet: &[char]) {
        self.prefix_map = HashMap::with_capacity(1024);
        self.alphabet_map = Self::precoding_for_alphabet(alphabet);
    }

    pub fn encode(&mut self, buffer: &str, base: u32) -> Result<String, Error> {
        let mut pointer = 0;
        let buffer = buffer.chars().collect::<Vec<_>>();

        while pointer != buffer.len() {
            for i in pointer..buffer.len() {
                let sequence = &buffer[pointer..i + 1];
                if !self.prefix_map.contains_key(sequence) || i + 1 == buffer.len() {
                    let prefix = &sequence[..i - pointer];
                    let suffix = sequence.last().expect("will never happen!");

                    let prefix_index = self.prefix_map.get(prefix).unwrap_or(&0);
                    let letter_index = self
                        .alphabet_map
                        .get(suffix)
                        .ok_or(CodingError::NoSuchLetterError(*suffix))?;

                    self.segement.push((*prefix_index, *letter_index));
                    self.prefix_map
                        .insert(sequence.to_vec(), (self.segement.len() - 1) as u64);

                    pointer = i + 1;
                    break;
                }
            }
        }

        self.prefix_code_length = (self.segement.len() as f64).log(base as f64).ceil() as u64;
        self.letter_code_length = (self.alphabet_map.len() as f64).log(base as f64).ceil() as u64;
        let base_n = generate_dest_base_encoder(base);

        let coding = self
            .segement
            .iter()
            .skip(1)
            .map(|v| {
                format!(
                    "{:0>width1$}{:0>width2$}",
                    base_n.gen(v.0),
                    base_n.gen(v.1),
                    width1 = self.prefix_code_length as usize,
                    width2 = self.letter_code_length as usize,
                )
            })
            .collect();
        Ok(coding)
    }

    pub fn decode(
        buffer: &str,
        prefix_code_length: u64,
        letter_code_length: u64,
        alphabet_map: &HashMap<char, u64>,
        base: u32,
    ) -> String {
        let chunk_size = (prefix_code_length + letter_code_length) as usize;
        let buffer = buffer.chars().collect::<Vec<_>>();
        let base_n = generate_dest_base_encoder(base);

        let code_to_alphabet = alphabet_map
            .iter()
            .map(|(&k, &v)| (v, k))
            .collect::<HashMap<_, _>>();
        let mut segement = vec![(0, 0)];

        buffer
            .chunks(chunk_size)
            .map(|x| x.iter().collect::<String>())
            .map(|x| {
                let mut prefix = base_n.decimal(&x[..prefix_code_length as usize]);
                let mut letter = base_n.decimal(&x[prefix_code_length as usize..]);
                segement.push((prefix, letter));

                let mut stack = vec![letter];
                while prefix != 0 {
                    let prior = segement.get(prefix as usize).unwrap();
                    prefix = prior.0;
                    letter = prior.1;
                    stack.push(letter);
                }

                stack
                    .iter()
                    .rev()
                    .map(|x| code_to_alphabet.get(x).unwrap())
                    .collect::<String>()
            })
            .collect()
    }

    pub fn show_dictionary(&self) {
        println!("{:?}", &self.prefix_map);
    }
}
