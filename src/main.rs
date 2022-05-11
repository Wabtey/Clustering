mod sequence;

use sequence::*;

fn main() {
    let seq_1 = Sequence::new_with_string("ATTACG".to_string());
    let seq_2 = Sequence::new_with_string("ATATCG".to_string());
    let seq_3 = Sequence::new_with_string("ACCCCG".to_string());
    let seq_4 = Sequence::new_with_string("GGGGAA".to_string());
    let seq_5 = Sequence::new_with_string("TTTACG".to_string());
    let seq_6 = Sequence::new_with_string("ATTAC".to_string());
    let seq_7 = Sequence::new_with_string("ATATC".to_string());

    println!("{}", seq_1.levenshtein_distance(&seq_1));
    println!("{}", seq_1.levenshtein_distance(&seq_2));
    println!("{}", seq_2.levenshtein_distance(&seq_1));
    println!("{}", seq_6.levenshtein_distance(&seq_7));
    println!("{}", seq_7.levenshtein_distance(&seq_6));
    println!("{}", seq_1.levenshtein_distance(&seq_3));
    println!("{}", seq_2.levenshtein_distance(&seq_3));
    println!("{}", seq_1.levenshtein_distance(&seq_4));

    let sequences = vec![seq_1, seq_2, seq_3, seq_4, seq_5];
    let bio_cluster = ClusterOfSequence::new_with_sequences(sequences);
    println!("bio_cluster {}", bio_cluster.get_newick());
    // bio_cluster.clusterize();
    // println!("bio_cluster {}", bio_cluster.get_newick());
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