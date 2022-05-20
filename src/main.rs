mod sequence;
mod utils;

use sequence::*;
use std::{fs, time::SystemTime};
// use utils::*;

fn main() {

    // --First Tests about the levenshtein distance--

    let seq_1 = Sequence::new_with_string("seq_1".to_string(), "ATTACG".to_string());
    let seq_2 = Sequence::new_with_string("seq_2".to_string(), "ATATCG".to_string());
    let seq_3 = Sequence::new_with_string("seq_3".to_string(), "ACCCCG".to_string());
    let seq_4 = Sequence::new_with_string("seq_4".to_string(), "GGGGAA".to_string());
    let seq_5 = Sequence::new_with_string("seq_5".to_string(), "TTTACG".to_string());
    let seq_6 = Sequence::new_with_string("seq_6".to_string(), "ATTAC".to_string());
    let seq_7 = Sequence::new_with_string("seq_7".to_string(), "ATATC".to_string());

    let vec_seq = vec![seq_1, seq_2, seq_3, seq_4, seq_5, seq_6, seq_7];
    let mut vec_res: Vec<Vec<usize>> = vec![vec![0;vec_seq.len()]; vec_seq.len()];
    for i in 0..vec_seq.len() {
        for j in 0..vec_seq.len() {
            // println!("{} with {}: {}", vec_seq[i].get_name(),
            //                            vec_seq[j].get_name(),
            //                            vec_seq[i].levenshtein_distance(&vec_seq[j]));
            vec_res[i][j] = vec_seq[i].levenshtein_distance(&vec_seq[j]);
            print!("{} ", vec_res[i][j]);
        }
        println!("");
    }

    // println!("{}", seq_1.levenshtein_distance(&seq_1));
    // println!("{}", seq_1.levenshtein_distance(&seq_2));
    // println!("{}", seq_2.levenshtein_distance(&seq_1));
    // println!("{}", seq_6.levenshtein_distance(&seq_7));
    // println!("{}", seq_7.levenshtein_distance(&seq_6));
    // println!("{}", seq_1.levenshtein_distance(&seq_3));
    // println!("{}", seq_2.levenshtein_distance(&seq_3));
    // println!("{}", seq_1.levenshtein_distance(&seq_4));
    
    // println!("name of ATTACG: {}", seq_1.get_name());

    // --Second Tests about Clusterize--

    // let sequences = vec![seq_1, seq_2, seq_3, seq_4, seq_5];
    let seq_0_test = Sequence::new_with_string("ATTACG".to_string(), "ATTACG".to_string());
    let seq_1_test = Sequence::new_with_string("ATATCG".to_string(), "ATATCG".to_string());
    let seq_2_test = Sequence::new_with_string("GCCGAG".to_string(), "GCCGAG".to_string());
    let seq_3_test = Sequence::new_with_string("ACCCCG".to_string(), "ACCCCG".to_string());
    let seq_4_test = Sequence::new_with_string("TCCCCG".to_string(), "TCCCCG".to_string());
    let sequences_test = vec![seq_0_test, seq_1_test, seq_2_test, seq_3_test, seq_4_test];
    let mut bio_cluster = ClusterOfSequence::new_with_sequences(sequences_test);
    println!("bio_cluster: \n{}", bio_cluster.get_newick());

    bio_cluster.clusterize_agglomerative();

    println!("bio_cluster: \n{}", bio_cluster.get_newick());
    // remove this line if you want to keep previous result
    // but will conflict with new one
    fs::remove_dir_all("./foam_rep/docs");
    bio_cluster.create_foam_rep("samples");
    

    // --Third Tests about the automatisation of the fasta reading--

    let mut hemo_cluster = utils::analyze_from_ressource_folder();
    println!("bio_cluster: \n{}", hemo_cluster.get_newick_old());
    hemo_cluster.clusterize_agglomerative();

    // start timer
    let st = SystemTime::now();


    hemo_cluster.create_foam_rep("hemo_samples");

    // check timer
    let ed = SystemTime::now();
    println!("{:#?}", ed.duration_since(st).unwrap());

    // println!("bio_cluster clusterized: \n{}", hemo_cluster.get_newick_old());
    
} 

/*
 * human same hemoglobine :
 * *insert monkey* = 0 lettre
 * chimpanzé = 1 lettre de diff
 * 
 * Protéines livres 
 * 
 * 
 * Pour faire un cluster :_
 * - ascendent : 1er ex (reconstitue tous les clusters)
 *               compare tous les points pour créer les plus petits clusters
 * - descendent : découpe (mieux pour faire n paquets)
 * 
 * Un bon Cluster : Un bon caféiné, "faites des pauses"
 */


 /*
 let foo = 
        match "ATTGG".to_string().chars().next(){
            Some(c) => c,
            None => 'n'
        };
    let bar = 
        match "".to_string().chars().next() {
            Some(c) => c,
            None => 'n'
        };
    println!("{} {}", foo, bar);
*/