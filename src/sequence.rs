use std::{io::prelude::*, str::*, vec::*};

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

    pub fn new_with_string(s: String) -> Sequence { 
        Sequence {
            seq: s,
        }
    }

    pub fn new_with_sequence(s: Sequence) -> Sequence { 
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

#[derive(Clone)]
pub struct ClusterOfSequence {
    sub_clusters: Vec<ClusterOfSequence>,
    elements: Vec<Sequence>,
}

impl ClusterOfSequence
{
    pub fn new(element: Sequence) -> ClusterOfSequence
    {
        ClusterOfSequence {
            sub_clusters: Vec::new(),
            elements: vec![element],
        }
    }

    pub fn new_with_sequences(elements_in:Vec<Sequence>) -> ClusterOfSequence
    {
        ClusterOfSequence {
            sub_clusters: {
                let mut res = Vec::new();
                for e in &elements_in {
                    res.push(ClusterOfSequence::new(e.clone()))
                }
                res
            },
            elements: elements_in,
        }
    }

    pub fn new_with_clusters(clusters_1: &ClusterOfSequence,
                        clusters_2: &ClusterOfSequence) -> ClusterOfSequence
    {
        ClusterOfSequence {
            sub_clusters: vec![clusters_1.clone(), clusters_2.clone()],
            // get all seq of clust_1 and clust_2 to elements
            elements: 
            {
                let mut n = clusters_1.clone().elements;
                n.extend(clusters_2.clone().elements);
                n
            }
        }
    }

    pub fn linkage(&self, a_cluster: ClusterOfSequence) -> Option<f32>
    {
        // covert the division by 0
        if self.elements.len() == 0 || a_cluster.elements.len() == 0 {
            None
            // f32::MAX;
        }else {
            let mut result=0.00;
            let mut counter = 0.00;
            /*
             * compare every e1: Sequence (in the self called Cluster)
             *      with every e2: Sequence (in a_cluster)
             *      by the levenshtein_distance
             * return the average distance between all sequence
             */ 
            for e1 in &self.elements {
                for e2 in &a_cluster.elements {
                    result += e1.levenshtein_distance(&e2) as f32;
                    counter += 1 as f32;
                }
            }
    
            result = result/counter;
            Some(result)
        }
    }
    pub fn get_newick(&self) -> String 
    {
        let mut res = String::from("(");
        if self.sub_clusters.len() != 0 {
            for sub in &self.sub_clusters {
                res.push_str(sub.get_newick().as_str());
                res.push(',');
            }
        }else {
            for e in &self.elements {
                res.push_str(e.seq.as_str());
                res.push(',');
            }
        }
        res.push(')');
        res
    }

    /**
     * first approch :
     *  compare each sequence
        group 2 close sequences in new cluster
        group other sequences in another cluster
        clusterize the second cluster
        -------------------------
        second approch :
        calculate the distance for each sequence two by two
        compare this distance with ALL of the other sequence
        if the first distance is the smallest
            create the sub-cluster containning the two sequence
        verify if the members are coherent 
            distance between all of them
            is smaller than any distance of a another sequence (in the universe)
        if this cluster is good to go, create it for good.
        if the cluster the most coherent is composed only by one seq, create it.
        compare two cluster 
        -------------------------
        third approch :
        state 1 : create clusters for all sequence indiv (ok when initilization)
        state 2 : compare all sub-clusters one by one by distance
        state 2.1 :     get the smallest distance
        state 2.2 :     check cluster condition
                             if it's correct confirm this cluster
                             else switch to the next
        state 3 : back to state 2 until the total of sub-cluster =< 2 
     */
    pub fn clusterize_agglomerative(&mut self) 
    {
        while self.sub_clusters.len() > 2 {
            'main_loop:
            for i in 0..self.sub_clusters.len() {
                let mut min: f32 = f32::MAX;
                let mut keep = 0;
                for j in 0.. self.sub_clusters.len() {
                    if i != j {
                        let dist = self.sub_clusters[i]
                                       .linkage(self.sub_clusters[j].clone())
                                       .unwrap();
                        if dist < min {
                            min = dist;
                            keep = j;
                        }
                    }
                }
                for j in 0..self.sub_clusters.len() {
                    if j != keep && j != i {
                        if self.sub_clusters[keep]
                               .linkage(self.sub_clusters[j].clone())
                               .unwrap() < min {
                            continue 'main_loop;
                        }
                    }
                }
                // ALED my bad
            }
        }
    }

    /**
     * reform the cluster structure of self
     * @param himself
     * @param n, the number of sub division needed
     */
    pub fn clusterize_divisive(&self, n: i32) {

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