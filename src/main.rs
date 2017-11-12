#[macro_use]
extern crate clap;
extern crate bio;
extern crate flate2;

// use std::io;
// use std::io::prelude::*;
// use std::fs::File;
// use std::path::{Path, PathBuf};

use clap::App;
use bio::io::{fasta, fastq};
use bio::alphabets::dna::revcomp;
use flate2::read::GzDecoder;

fn main() {
    
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    fn canonical(seq: &Vec<u8>) -> Vec<u8> {
        return revcomp(seq);
    }

    if let Some(sub_matches) = matches.subcommand_matches("build") {
        let input_fa_paths: Vec<&str> = sub_matches.values_of("fasta_files").unwrap().collect();
        for input_fa_path in &input_fa_paths {
            println!("Using input files: {}", input_fa_path);
            let fa_reader = fasta::Reader::from_file(&input_fa_path).unwrap();
            println!("seqs in file: {}", fa_reader.records().count());
        }
    }
    
    if let Some(sub_matches) = matches.subcommand_matches("call") {

        let input_fq_paths: Vec<&str> = sub_matches.values_of("input_files").unwrap().collect();
        
        for input_fq_path in &input_fq_paths {
            println!("Using input files: {}", input_fq_path);
            let mut fq_reader = fastq::Reader::from_file(&input_fq_path).unwrap();
            // println!("seqs in file: {}", fq_reader.records().count());
            let mut first_seq = fastq::Record::new();
            fq_reader.read(&mut first_seq).unwrap();
        }
    }

    
}
