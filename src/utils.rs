use std::{
    fs,
    time::SystemTime
};
use crate::sequence::*;

fn read_fasta(path: &str) -> String {
    let start = SystemTime::now();
    let content = fs::read_to_string(path).expect("Can't touch it");
    let mut res:String = String::new();
    //print!("{:?}", content);
    for line in content.split("\n") {
        if !line.starts_with(">") {
            res.push_str(&line.replace("\r", ""));
        }    
    }
    let end = SystemTime::now();
    println!("read content: {:?}", end.duration_since(start).unwrap());
    res
}

/**
 * it's meant to simplify the main method by just one method
 * calling as many time the read_fasta method for all the
 * fasta files in the path (in param).
 * @param a specific folder name in the res folder
 */
pub fn analyze_from_ressource_folder() -> ClusterOfSequence{
    let paths = fs::read_dir("./data_reformed").unwrap();

    /*
     * Display all files contained in the current path
    println!("Please input your choice :");

    let mut counter = 0;
    for path in paths {
        println!("{}. {}",counter, path.unwrap().path().display());
        counter+=1
    }

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    */

    let mut sequences: Vec<Sequence> = Vec::new();

    for path in paths {

        let mut name = path.as_ref()
                           .unwrap()
                           .path()
                           .display()
                           .to_string();
        
        name = name.replace("./data_reformed\\", "")
                   .replace("_sequence.fa", "")
                   .replace("_", " ");
    
        sequences.push(Sequence::new_with_string(
                        name.to_string(),
                        read_fasta(&path.unwrap().path().display().to_string()))
                      );
        println!("name of the sequence: {}", name);

    }

    let bio_cluster = ClusterOfSequence::new_with_sequences(sequences);

    bio_cluster


}