use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fmt,
    fs::{File, OpenOptions},
    io::prelude::*,
    path::Path,
    rc::Rc,
};

use base_custom::BaseCustom;
use failure::Error;

#[derive(Debug)]
pub struct TreeNode {
    pub label: char,
    pub p: f64,
    pub coding: Option<String>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.label, self.p)
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Eq for TreeNode {}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.p > other.p {
            std::cmp::Ordering::Less
        } else if self.p < other.p {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl TreeNode {
    pub fn new(label: char, p: f64) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            label,
            p,
            coding: None,
            parent: None,
            children: Vec::new(),
        }))
    }

    pub fn add_child(parent: &Rc<RefCell<Self>>, node: &Rc<RefCell<Self>>) {
        parent.borrow_mut().p += node.borrow().p;
        node.borrow_mut().parent = Some(parent.clone());
        parent.borrow_mut().children.push(node.clone());
    }
}

pub fn generate_dest_base_encoder(base: u32) -> BaseCustom<char> {
    let alphabet = (0..base)
        .map(|x| std::char::from_u32(x + '0' as u32).unwrap())
        .collect();

    BaseCustom::<char>::new(alphabet)
}

pub fn string_to_u8(buffer: &str, base: u32) -> Vec<u8> {
    let base_n = generate_dest_base_encoder(base);

    buffer
        .chars()
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|x| x.iter().collect::<String>())
        .map(|x| base_n.decimal(x) as u8)
        .collect::<Vec<_>>()
}

pub fn read_from_file<P: AsRef<Path>>(data_file_path: P) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = File::open(data_file_path)?;
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn save_to_file<P: AsRef<Path>>(file_path: P, buffer: &[u8]) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    file.write_all(buffer)?;

    Ok(())
}

pub fn calculate_distribution(buffer: &str) -> BTreeMap<char, f64> {
    let mut distribution = BTreeMap::new();

    for c in buffer.chars() {
        *distribution.entry(c).or_default() += 1_f64;
    }

    let all = buffer.len() as f64;
    for v in &mut distribution.values_mut() {
        *v /= all;
    }

    distribution
}

pub fn average_coding_length<S: std::hash::BuildHasher>(
    distribution: &BTreeMap<char, f64>,
    coded: &HashMap<char, (u64, String), S>,
) -> f64 {
    coded
        .iter()
        .map(|(k, (length, _))| distribution.get(k).unwrap() * (*length as f64))
        .sum()
}

pub fn entropy(distribution: &BTreeMap<char, f64>, base: u32) -> f64 {
    distribution.values().map(|x| -x * x.log(base.into())).sum()
}
