#![allow(dead_code)]

use std::{vec::*, time::SystemTime, fs};

#[derive(Clone)]
pub struct Sequence {
    name: String,
    seq: String,
}


impl Sequence {
    
    pub fn new() -> Sequence{
        Sequence {
            name: String::new(),
            seq: String::new(),
        }
    }

    pub fn new_with_string(name: String, seq: String) -> Sequence { 
        Sequence {
            name,
            seq,
        }
    }

    pub fn new_with_sequence(name: String, s: Sequence) -> Sequence { 
        Sequence {
            name,
            seq: s.seq,
        }
    }

    pub fn get_name(&self) -> String {
        self.clone().name
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

/**
 * we're able to create a pure copy of a cluster with a new ref by the Clone derive
 * 
 * sub_clusters correspond to all the clusters contained in this clusters
 * elements contains all element in this cluster, eventually from the sub_clusters
 */
#[derive(Clone)]
pub struct ClusterOfSequence {
    name: String,
    sub_clusters: Vec<ClusterOfSequence>,
    elements: Vec<Sequence>,
}

impl ClusterOfSequence
{
    pub fn new(element: Sequence) -> ClusterOfSequence
    {
        ClusterOfSequence {
            name: element.clone().name,
            sub_clusters: Vec::new(),
            elements: vec![element],
        }
    }

    pub fn new_with_sequences(elements:Vec<Sequence>) -> ClusterOfSequence
    {
        ClusterOfSequence {
            name:
            {
                let mut res = String::new();
                res.push('-');
                for elem in &elements {
                    // to avoid Err :
                    if elem.name.len() <= 3 {
                        res.push_str(&elem.name[0..elem.name.len()]);
                    }else {
                        res.push_str(&elem.name[0..3]);
                    }
                    res.push('-');
                }
                res
            },
            sub_clusters:
            {
                let mut res = Vec::new();
                for e in &elements {
                    res.push(ClusterOfSequence::new(e.clone()))
                }
                res
            },
            elements,
        }
    }

    /**
     * ClusterOfSequence without reference &?
     */
    pub fn new_with_clusters(clusters_1: ClusterOfSequence,
                        clusters_2: ClusterOfSequence) -> ClusterOfSequence
    {
        ClusterOfSequence {
            name:
            {
                // we have to make sure there is no doublon
                // to avoid conflict and wrong relation in foam
                let mut foo= String::new();
                foo.push('-');
                for elem in &clusters_1.elements {
                    if elem.name.len() <= 3 {
                        foo.push_str(&elem.name[0..elem.name.len()]);
                    }else {
                        foo.push_str(&elem.name[0..3]);
                    }
                    foo.push('-');
                }
                for elem in &clusters_2.elements {
                    if elem.name.len() <= 3 {
                        foo.push_str(&elem.name[0..elem.name.len()]);
                    }else {
                        foo.push_str(&elem.name[0..3]);
                    }
                    foo.push('-');
                }
                foo
            },
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

    pub fn get_newick_old(&self) -> String 
    {
        let mut res = String::from("(");
        if self.sub_clusters.len() != 0 {
            for i in 0..self.sub_clusters.len() {
                res.push_str(&self.sub_clusters[i].get_newick_old().as_str());

                // e != self.elements.last().unwrap()
                if i != self.sub_clusters.len()-1 {
                    res.push(',');
                }
                
            }
        }else {
            for e in 0..self.elements.len() {
                res.push_str(&self.elements[e].name.as_str());

                if e != self.elements.len()-1 {
                    res.push(',');
                }
                
            }
        }
        res.push(')');
        res
    }

    /**
     * TODO recheck step 6 and 7
     * if you want to get a cleaner look change :
     *      .elements[e].seq to .elements[e].name
     * if you want to get the precise sequence in the representation :
     *      .elements[e].name to .elements[e].seq
     */
    pub fn get_newick(&self) -> String
    {
        self.get_newick_with_space(0)
    }

    fn get_newick_with_space(&self, space: i32) -> String
    {
        let mut res = String::new();
        if self.sub_clusters.len() != 0 {
            for i in 0..self.sub_clusters.len() {    
                // res.push_str("|");     
                res.push_str(
                    &self.sub_clusters[i].get_newick_with_space(space+1)
                                         .as_str());

                // e != self.elements.last().unwrap()
                // TODO : check this if else usefulness
                if i != self.sub_clusters.len()-1 {
                    res.push_str("\n");
                }else {
                    res.push_str("\n");
                }
                
            }
        }else {
            for e in 0..self.elements.len() {
                if e ==0 || e == &self.elements.len() -1{
                    for _count in 0..space {
                        res.push_str("-");
                    }
                }
                // .elements[e].seq or .elements[e].name depending
                res.push_str(&self.elements[e].name.as_str());

                if e != self.elements.len()-1 {
                    res.push_str("\n");
                }
                
            }
        }
        // res.push_str("\n");
        res
    }

    /**
     * create a serie of markdown file :
     *  Each file contains 
     *      - [[wikilinks]] to subclusters
     *      - the list of elements contained
     * Make sure you install foam vscode, and have theses folder :
     *      - _layout
     *      - .vscode
     *      - .foam
     *      - assets
     * At the end, you will prompt the foam graph in vscode by :
     * ctrl+p ">Foam: Show Graph"
     */
    pub fn create_foam_rep(&self, folder_name: &str) // -> Result<()>
    {
        
        let mut foo =
        "---
        \ntitle: name
        \ntype: bio_inf_results
        \n---
        \n".to_string();

        // Add up all sub_cluster links
        for counter in 0..self.sub_clusters.len() {
            let mut bar = "\n [[wikilink]]"
                        .replace("wikilink", &self.sub_clusters[counter].clone().name);
            
            if counter != self.sub_clusters.len()-1 {
                bar.push_str(", ");
            }
            foo.push_str(&bar);

            // now create the md file associated with this sub cluster
            self.sub_clusters[counter].create_foam_rep(folder_name);
        }

        // separator
        foo.push_str("\n");

        // if you prefer have the sequence prompted
        // change elment.name to element.seq
        for element in &self.elements {
            let mut bar = "element"
                        .replace("element", &element.name);
            if element.name != self.elements.last().unwrap().name {
                bar.push_str(", ");
            }
            foo.push_str(&bar);
        }

        // write this foobar to a .md file

        let name = self.clone().name;
        // change title
        foo = foo.replace("name", &name);

        // println!("{}", name);


        // create docs folder if it did not exist
        let folder_path = "./foam_rep/docs/folder_name/"
                                .replace("folder_name", folder_name);
        fs::create_dir_all(&folder_path);
        let md_path = "./foam_rep/docs/folder_name/name.md"
                                .replace("folder_name", folder_name)
                                .replace("name", &name);

        fs::write(md_path, foo)
                .expect("Unable to write file");
            
        // Ok(())

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
        // start timer
        let st = SystemTime::now();

        let mut i = 0;
        'main_loop:
        while i < self.sub_clusters.len() && self.sub_clusters.len() > 2 {

            let mut min: f32 = f32::MAX;
            let mut keep = 0;
            // state 2
            for j in 0.. self.sub_clusters.len() {
                if i != j {
                    // state 2.1
                    let dist = self.sub_clusters[i]
                                   .linkage(self.sub_clusters[j].clone())
                                   .unwrap();
                    if dist < min {
                        min = dist;
                        keep = j;
                    }
                }

                // we tried here to verify at the same time the cluster's coherence
                // but we need to keep track of a smallest distance BEFORE checking
                // the coherence (in the case if the closest is after j)

            }
            // state 2.2
            for j in 0..self.sub_clusters.len() {
                if j != keep && j != i {
                    if self.sub_clusters[keep]
                           .linkage(self.sub_clusters[j].clone())
                           .unwrap() < min {
                        i+=1;
                        continue 'main_loop;
                        // the while exit is ONLY here
                    }
                }
            }

            // create a new subCluster which contains two coherent element 
            let sub_1 = self.sub_clusters[i].clone();
            let sub_2 = self.sub_clusters[keep].clone();

            // commit: from here I remove all the ref & to self call
            
            // remove the two clusters from the first cluster
            self.sub_clusters.remove(i);
            if keep > i {
                self.sub_clusters.remove(keep-1);
            }else {
                self.sub_clusters.remove(keep);
            }

            // add the new cluster which contains the two last clusters
            self.sub_clusters.insert(
                0, ClusterOfSequence::new_with_clusters(sub_1, sub_2));
            
            i = 0;
        }

        // check timer
        let ed = SystemTime::now();
        println!("{:#?}", ed.duration_since(st).unwrap());
    }

    /**
     * reform the cluster structure of self
     * @param himself
     * @param n, the number of sub division needed
     */
    pub fn clusterize_divisive(&self, _n: i32) {

    }
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