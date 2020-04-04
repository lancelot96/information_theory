use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};

use failure::Error;

use crate::helper::TreeNode;

pub struct HuffmanCoding;

impl HuffmanCoding {
    pub fn first_to_merge(alphabet_size: u32, base: u32) -> u32 {
        (alphabet_size - 2) % (base - 1) + 2
    }

    pub fn generate_mapping(
        distribution: &BTreeMap<char, f64>,
        base: u32,
    ) -> Result<HashMap<char, (u64, String)>, Error> {
        assert!(distribution.len() > 1);

        let mut p_queue = distribution
            .iter()
            .map(|(k, v)| TreeNode::new(*k, *v))
            .collect::<BinaryHeap<_>>();

        let first_merge_count = Self::first_to_merge(distribution.len() as u32, base);
        while p_queue.len() != 1 {
            let new_node = TreeNode::new('\0', 0_f64);
            let merge_count = if p_queue.len() == distribution.len() {
                first_merge_count
            } else {
                base
            };
            for _ in 0..merge_count {
                let node = p_queue.pop().expect("Will never happen!");
                TreeNode::add_child(&new_node, &node);
            }
            p_queue.push(new_node);
        }

        let tree_root = p_queue.pop().expect("Will never happen!");
        tree_root.borrow_mut().coding = Some("".into());

        let mut depth = 0;
        let mut queue = VecDeque::new();
        queue.push_back(tree_root);

        let mut i = 0;
        let mut codebook: HashMap<char, (u64, String)> = HashMap::new();
        while !queue.is_empty() {
            let width = queue.len();
            for _ in 0..width {
                let node = queue.pop_front().expect("Will never happen!");
                let mut node = node.borrow_mut();

                if let Some(parent) = &node.parent {
                    node.coding = Some(format!(
                        "{}{}",
                        parent.borrow().coding.as_ref().expect("Will never happen!"),
                        i,
                    ));
                    i = (i + 1) % base;
                }

                if node.children.is_empty() {
                    codebook.insert(
                        node.label,
                        (depth, node.coding.take().expect("Will never happen!")),
                    );
                    continue;
                }

                for child in &node.children {
                    queue.push_back(child.clone());
                }
            }
            depth += 1;
        }

        Ok(codebook)
    }

    pub fn encode(buffer: &str, codebook: &HashMap<char, (u64, String)>) -> String {
        buffer
            .chars()
            .map(|c| codebook.get(&c).unwrap().1.clone())
            .collect()
    }

    pub fn decode(coded: &str, codebook: &HashMap<char, (u64, String)>) -> Result<String, Error> {
        let codebook_reverse = codebook
            .iter()
            .map(|(c, (_, v))| (v, *c))
            .collect::<HashMap<_, _>>();

        let decoded =
            coded
                .chars()
                .fold((String::new(), None), |res: (String, Option<String>), c| {
                    let (mut decoded, mut cs) = res;
                    let mut code = cs.take().unwrap_or_default();
                    code.push(c);

                    if let Some(code) = codebook_reverse.get(&code) {
                        decoded.push(*code);
                    } else {
                        cs = Some(code)
                    }
                    (decoded, cs)
                });

        Ok(decoded.0)
    }
}
