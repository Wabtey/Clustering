mod sequence;
mod utils;

use sequence::*;
// use utils::*;

fn main() {

    // --First Tests about the levenshtein distance--

    let seq_1 = Sequence::new_with_string("seq_1".to_string(), "ATTACG".to_string());
    let seq_2 = Sequence::new_with_string("seq_2".to_string(), "ATATCG".to_string());
    let seq_3 = Sequence::new_with_string("seq_3".to_string(), "ACCCCG".to_string());
    let seq_4 = Sequence::new_with_string("seq_4".to_string(), "GGGGAA".to_string());
    let _seq_5 = Sequence::new_with_string("seq_5".to_string(), "TTTACG".to_string());
    let seq_6 = Sequence::new_with_string("seq_6".to_string(), "ATTAC".to_string());
    let seq_7 = Sequence::new_with_string("seq_7".to_string(), "ATATC".to_string());

    println!("{}", seq_1.levenshtein_distance(&seq_1));
    println!("{}", seq_1.levenshtein_distance(&seq_2));
    println!("{}", seq_2.levenshtein_distance(&seq_1));
    println!("{}", seq_6.levenshtein_distance(&seq_7));
    println!("{}", seq_7.levenshtein_distance(&seq_6));
    println!("{}", seq_1.levenshtein_distance(&seq_3));
    println!("{}", seq_2.levenshtein_distance(&seq_3));
    println!("{}", seq_1.levenshtein_distance(&seq_4));
    
    // println!("name of ATTACG: {}", seq_1.get_name());

    // --Second Tests about Clusterize--

    // let sequences = vec![seq_1, seq_2, seq_3, seq_4, seq_5];
    let seq_0_test = Sequence::new_with_string("seq_0_test".to_string(), "ATTACG".to_string());
    let seq_1_test = Sequence::new_with_string("seq_1_test".to_string(), "ATATCG".to_string());
    let seq_2_test = Sequence::new_with_string("seq_2_test".to_string(), "GCCGAG".to_string());
    let seq_3_test = Sequence::new_with_string("seq_3_test".to_string(), "ACCCCG".to_string());
    let seq_4_test = Sequence::new_with_string("seq_4_test".to_string(), "TCCCCG".to_string());
    let sequences_test = vec![seq_0_test, seq_1_test, seq_2_test, seq_3_test, seq_4_test];
    let mut bio_cluster = ClusterOfSequence::new_with_sequences(sequences_test);
    println!("bio_cluster: \n{}", bio_cluster.get_newick());
    bio_cluster.clusterize_agglomerative();
    println!("bio_cluster: \n{}", bio_cluster.get_newick());

    // --Third Tests about the automatisation of the fasta reading--

    let mut hemo_cluster = utils::analyze_from_ressource_folder();
    println!("bio_cluster: \n{}", hemo_cluster.get_newick_old());
    hemo_cluster.clusterize_agglomerative();
    println!("bio_cluster clusterized: \n{}", hemo_cluster.get_newick_old());
    // let seq_0_hemo = Sequence::new_with_string()
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