use std::{
    fs::{File, OpenOptions},
    io::prelude::*,
    process::Command,
};

use assert_cmd::prelude::*;
use predicates::{
    ord::eq,
    str::{contains, is_empty, PredicateStrExt},
};
use tempfile::TempDir;

#[test]
fn cli_no_args() {
    Command::cargo_bin("assignment_big")
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn cli_version() {
    Command::cargo_bin("assignment_big")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_lz_encode() {
    let source = "data.txt";
    let target = "encoded";
    let alphabet = "alphabet.txt";
    let temp_dir = TempDir::new().unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(alphabet))
        .and_then(|mut f| f.write_all(b"abc"))
        .unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(source))
        .and_then(|mut f| f.write_all(b"aaaaabbbcc"))
        .unwrap();

    Command::cargo_bin("assignment_big")
        .unwrap()
        .args(&[
            "lz",
            "--source",
            source,
            "--target",
            target,
            "--encode",
            "--alphabet",
            alphabet,
        ])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(eq("prefix_code_length: 3\tletter_code_length: 2").trim());

    let mut encoded_path = temp_dir.path().join(target);
    assert!(encoded_path.set_extension("txt"));

    let mut encode = String::new();
    File::open(encoded_path)
        .and_then(|mut x| x.read_to_string(&mut encode))
        .unwrap();

    assert_eq!(&encode, "000000010001001000011001000010");
}

#[test]
fn cli_lz_decode() {
    let source = "encoded.txt";
    let target = "data.txt";
    let alphabet = "alphabet.txt";
    let temp_dir = TempDir::new().unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(alphabet))
        .and_then(|mut f| f.write_all(b"abc"))
        .unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(source))
        .and_then(|mut f| f.write_all(b"000000010001001000011001000010"))
        .unwrap();

    Command::cargo_bin("assignment_big")
        .unwrap()
        .args(&[
            "lz",
            "--source",
            source,
            "--target",
            target,
            "--decode",
            "--alphabet",
            alphabet,
            "--prefix-code-length",
            "3",
            "--letter-code-length",
            "2",
        ])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(eq("aaaaabbbcc").trim());
}

#[test]
fn cli_huffman_encode() {
    let source = "data.txt";
    let target = "encoded";
    let temp_dir = TempDir::new().unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(source))
        .and_then(|mut f| f.write_all(b"aaaaabbbcc"))
        .unwrap();

    Command::cargo_bin("assignment_big")
        .unwrap()
        .args(&[
            "huffman", "--source", source, "--target", target, "--encode",
        ])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(eq("average coding length: 1.5\tentropy: 1.4854752972273344").trim());

    let mut encoded_path = temp_dir.path().join(target);
    assert!(encoded_path.set_extension("txt"));

    let mut encode = String::new();
    File::open(encoded_path)
        .and_then(|mut x| x.read_to_string(&mut encode))
        .unwrap();

    assert_eq!(&encode, "000001111111010");
}

#[test]
fn cli_huffman_decode() {
    let codebook = "codebook.cb";
    let source = "encoded.txt";
    let target = "data.txt";
    let temp_dir = TempDir::new().unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(codebook))
        .and_then(|mut f| f.write_all(b"{\"a\":[1,\"0\"],\"c\":[2,\"10\"],\"b\":[2,\"11\"]}"))
        .unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(source))
        .and_then(|mut f| f.write_all(b"000001111111010"))
        .unwrap();

    Command::cargo_bin("assignment_big")
        .unwrap()
        .args(&[
            "huffman",
            "--source",
            source,
            "--target",
            target,
            "--decode",
            "--codebook",
            codebook,
        ])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(eq("aaaaabbbcc").trim());
}

#[test]
fn cli_arithmetic_encode() {
    let source = "data.txt";
    let target = "encoded";
    let temp_dir = TempDir::new().unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(source))
        .and_then(|mut f| f.write_all(b"aaaaabbbcc"))
        .unwrap();

    Command::cargo_bin("assignment_big")
        .unwrap()
        .args(&[
            "arithmetic",
            "--source",
            source,
            "--target",
            target,
            "--encode",
        ])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(is_empty());

    let mut encoded_path = temp_dir.path().join(target);
    assert!(encoded_path.set_extension("txt"));

    let mut encode = String::new();
    File::open(encoded_path)
        .and_then(|mut x| x.read_to_string(&mut encode))
        .unwrap();

    assert_eq!(&encode, "0000010111000101");
}

#[test]
fn cli_arithmetic_decode() {
    let distribution = "distribution.prob";
    let source = "encoded.txt";
    let target = "data.txt";
    let temp_dir = TempDir::new().unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(distribution))
        .and_then(|mut f| f.write_all(b"{\"a\":0.5,\"b\":0.3,\"c\":0.2}"))
        .unwrap();

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_dir.path().join(source))
        .and_then(|mut f| f.write_all(b"0000010111000101"))
        .unwrap();

    Command::cargo_bin("assignment_big")
        .unwrap()
        .args(&[
            "arithmetic",
            "--source",
            source,
            "--target",
            target,
            "--decode",
            "--distribution",
            distribution,
        ])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(eq("aaaaabbbcc").trim());
}
