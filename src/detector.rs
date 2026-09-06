use std::cmp::Reverse;

use crate::signatures::{SIGNATURES, Signature};

pub fn option_handle(header_buffer: &[u8], footer_buffer: &[u8]) {
    let matches = detect_signature(header_buffer, footer_buffer);
    let rank_list = ranking_system(matches);
    determine_matches(rank_list);
}

fn detect_signature(header_buffer: &[u8], footer_buffer: &[u8]) -> Vec<&'static Signature> {
    let matches: Vec<&'static Signature> = SIGNATURES
        .iter()
        .filter(|&sig| {
            header_buffer[sig.offset as usize..].starts_with(sig.header)
                && footer_buffer.ends_with(sig.trailer)
        })
        .collect::<Vec<&'static Signature>>();
    matches
}

fn ranking_system(matches: Vec<&Signature>) -> Vec<(usize, &Signature)> {
    let mut rank_list = vec![];
    for signature in matches {
        let offset_score = if signature.offset == 0 { 0 } else { 1 };
        let score = signature.header.len() + signature.trailer.len() + offset_score;
        rank_list.push((score, signature));
    }
    rank_list.sort_by_key(|r| Reverse(r.0));
    rank_list
}

pub fn group_signatures<'a>(input: &[(usize, &'a Signature)]) -> Vec<(usize, Vec<&'a Signature>)> {
    let mut groups: Vec<(usize, Vec<&'a Signature>)> = Vec::new();

    for &(score, signature) in input {
        match groups.last_mut() {
            Some((last_score, signatures)) if *last_score == score => {
                signatures.push(signature);
            }
            _ => {
                groups.push((score, vec![signature]));
            }
        }
    }

    groups
}

fn determine_matches(rank_list: Vec<(usize, &Signature)>) {
    const LEVEL: [&str; 5] = ["Very High", "High", "Medium", "Low", "Very Low"];

    let grouped_signatures = group_signatures(&rank_list);

    for (idx, (score, signatures)) in grouped_signatures.iter().enumerate() {
        let level = LEVEL.get(idx).copied().unwrap_or("Very Low");

        println!("Match Strength: {level} | Score: {score}");

        for signature in signatures {
            println!(
                "File Type: {} | File Class: {} | Description: {}",
                signature.extension, signature.class, signature.description
            );
        }
    }
}

/*
fn print_results(rank_list: Vec<(usize, &Signature)>, mut index_start: usize, diff: usize) {
    const LEVEL: [&str; 5] = ["Very High", "High", "Medium", "Low", "Very Low"];
    for i in 0..rank_list.len() {
        println!(
            "Match Strength: {} | File Type: {} | File Class: {} | Description: {}",
            LEVEL[index_start],
            rank_list[i].1.extension,
            rank_list[i].1.class,
            rank_list[i].1.description
        );
        if index_start < 5 && i < rank_list.len() - 1 && rank_list[i].0 != rank_list[i + 1].0 {
            index_start += diff;
        }
    }
}
*/
