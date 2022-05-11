use std::{io::prelude::*, str::*};

#[derive(Clone)]
pub struct Sequence {
    seq: String,
}


impl Sequence {
    pub fn new() -> Sequence{
        Sequence {
            seq: String::new(),
        }
    }

    pub fn new_string(s: String) -> Sequence { 
        Sequence {
            seq: s,
        }
    }

    pub fn new_sequecence(s: Sequence) -> Sequence { 
        Sequence {
            seq: s.seq,
        }
    }

    /**
     * ---------------
     * ^ ----------- ^
     * ^ ^ATCCCTGCA^ ^
     * ^ ^ATCCCTGCT^ ^
     * ^ ----------- ^
     * ^  AGCCCTGCT  ^
     * ---------------
     * 
     * Levenshtein distance
     * return the distance between the two sequences
     */
    pub fn levenshtein_distance(&self, other_seq: &Sequence) -> usize {
        // convert our seq to Vec<Char>
        let s1: String = self.seq.to_string();
        let s2: String = other_seq.seq.to_string();
        let seq_1: Vec<char>= s1.chars().collect();
        let seq_2: Vec<char> = s2.chars().collect();

        // initialization of result matrix
        let mut d: Vec<Vec<usize>> = vec![vec![0;s2.len()+1]; s1.len()+1];
        // println!("d.len(): {}", d.len());

        // write a meter
        for i in 1..d.len() {
            d[i][0] = i 
        }
        for j in 1..d[0].len() {
            // println!("j: {}, d[0].len(): {}", j, d[0].len());
            d[0][j] = j 
        }

        // idk what it determine
        let mut subtitution_cost: usize;
        for i in 1..d.len(){
            for j in 1..d[i].len() {
                // let subtitution_cost: i8;
                // maybe to expensive to recreate every case
                if seq_1[i-1] == seq_2[j-1] {
                    subtitution_cost = 0;
                }else {
                    subtitution_cost = 1;
                }

                d[i][j] = lib::minimum_3(d[i-1][j] + 1,                 // deletion
                                         d[i][j-1] + 1,                 // insertion
                                         d[i-1][j-1] + subtitution_cost)// substitution
            }
        }

        d[seq_1.len()][seq_2.len()]
    }



}

mod subclass{

}

mod lib {
    /**
     * (0, 1 , -1) -> 0, -1
     * (-1, 1, 0) -> -1, -1
     */
    pub fn minimum_3(a: usize, b: usize, c: usize) -> usize {
        if a < b {
            if c < a {
                c
            }else {
                a
            }
        }else {
            if c < b {
                c
            }else {
                b
            }
        }
    }
}