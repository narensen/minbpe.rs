use std::collections::HashMap;
const INITIAL_VOCAB_SIZE: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IntPair {
    first: u8,
    second: u8,
}

#[derive(Debug, Clone)]
struct Merge {
    pair: IntPair,
    idx: usize,
}

struct BasicTokenizer {
    merges: Vec<Merge>,
    vocab: Vec<Vec<u8>>,
}

impl BasicTokenizer {
    fn new() -> Self {
        let mut vocab = Vec::with_capacity(INITIAL_VOCAB_SIZE);
        for i in 0..INITIAL_VOCAB_SIZE {
            vocab.push(vec![i as u8]);
        }
        Self {
            merges: Vec::new(),
            vocab,
        }
    }

    fn train(&mut self, text: &str, vocab_size: usize, verbose: bool) {
        let num_merges = vocab_size - INITIAL_VOCAB_SIZE;
        let mut ids: Vec<u8> = text.bytes().collect();

        for i in 0..num_merges {
            let (pair_counts, _pair_counts_size) = Self::token_counts(&ids);

            let best_pair = pair_counts.iter().max_by_key(|entry| entry.1).map(|(pair, _)| *pair);

            if let Some(best_pair) = best_pair {
                let idx = INITIAL_VOCAB_SIZE + i;
                Self::merge(&mut ids, best_pair, idx as u8);
                self.merges.push(Merge { pair: best_pair, idx });

                self.vocab.push(vec![best_pair.first, best_pair.second]);

                if verbose {
                    println!(
                        "Merge {}/{}: ({}, {}) -> {}",
                        i + 1,
                        num_merges,
                        best_pair.first,
                        best_pair.second,
                        idx
                    );
                }
            } else {
                break;
            }
        }
    }

    fn encode(&self, text: &str) -> Vec<u8> {
        let mut ids = Vec::new();
        for c in text.chars() {
            let id = c as u8;
            ids.push(id);
        }
        ids
    }
    fn decode(&self, ids: &[u8]) -> String {
        let mut text = Vec::new();
        for &id in ids {
            let bytes = &self.vocab[id as usize];
            text.extend_from_slice(bytes);
        }
        String::from_utf8_lossy(&text).into_owned()
    }

    fn token_counts(ids: &[u8]) -> (HashMap<IntPair, usize>, usize) {
        let mut pair_counts: HashMap<IntPair, usize> = HashMap::new();
        for i in 0..ids.len() - 1 {
            let pair = IntPair {
                first: ids[i],
                second: ids[i + 1],
            };
            *pair_counts.entry(pair).or_insert(0) += 1;
        }
        let pair_counts_size = pair_counts.len();
        (pair_counts, pair_counts_size)
    }

    fn merge(ids: &mut Vec<u8>, pair: IntPair, idx: u8) {
        if ids.is_empty() {
            return;
        }
        let mut new_ids = Vec::with_capacity(ids.len());
        let mut i = 0;
        while i < ids.len() {
            if i < ids.len() - 1 && ids[i] == pair.first && ids[i + 1] == pair.second {
                new_ids.push(idx);
                i += 2;
            } else {
                new_ids.push(ids[i]);
                i += 1;
            }
        }
        *ids = new_ids;
    }
}

fn main() {
    let mut tokenizer = BasicTokenizer::new();

    let text = "Hello, world";
    let vocab_size = 200;

    println!("Input Text: {}", text);
    tokenizer.train(text, vocab_size, true);

    let ids = tokenizer.encode(text);
    println!("Encoded IDs: {:?}", ids);

    let decoded_text = tokenizer.decode(&ids);
    println!("Decoded Text: {}", decoded_text);
}