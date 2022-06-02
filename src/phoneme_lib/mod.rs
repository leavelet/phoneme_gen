use std::collections::{HashMap, HashSet};
use std::fs::{File, self};
use std::path::Path;
use std::io::{self, BufRead};
use std::io::prelude::*;

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn make_dic(path: &Path, dic: &mut HashMap<String, String>) -> io::Result<()>{
    println!("using dict {}", path.display());
    let lines = read_lines(path)?;

    for line in lines {
        let line = line?;
        let all = line.split('\t').collect::<Vec<&str>>();
        match all.len() {
            2 => {
                dic.insert(all[0].trim().to_string(), all[1].to_string());
                // println!("add {} {}", all[0].trim().to_string(), all[1].to_string())
                }
            _ => {println!("{:?} the line contains less than two words, skip", all);}
        }
    }
    Ok(())
}
pub fn write_to_words(dic:&HashMap<String, String>, path_of_word: &Path) -> io::Result<()> {
    let all_file = fs::read_dir(path_of_word)?;
    let mut missing = HashSet::new();
    for file in all_file {
        //dataset0002_word_list
        let file = file?;
        if !fs::metadata(file.path()).unwrap().is_file(){
            continue;
        }
        let file_name = file.file_name().into_string().expect("not utf-8, invalid file name!");
        let dataset_name = 
            match file_name.find("_word_list"){
                Some(num) => {
                    file_name[..num].to_string()
                }
                None => {
                    println!(r#"file {} do not match the "_word_list" pattern "#, file_name);
                    continue;
                }
            };
        println!("into dataset {}", dataset_name);
        let full_path = file.path().to_str().unwrap().to_string();
        let path_of_words = Path::new(&full_path);

        let lines = read_lines(path_of_words)
                .expect(&format!("can't open {:?}!", &path_of_words));
        //dataset0042_word2phonemes
        let out = format!("{}/{}_word2phonemes.txt", path_of_word.display().to_string(), dataset_name);
        println!("write to {}", &out);
        let mut outfile = File::create(out).expect("invalid outfile!");
        for word in lines {
            let mut word = word?.trim().to_string();
            //write to format as "wofc phonemes"
            if word == "\n".to_string() {
                continue;
            }
            match dic.get(&word) {
                Some(phonemes) => {
                    word.push_str("\t");
                    let mut phonemes_str = phonemes.to_string();
                    phonemes_str.push_str("\n");
                    outfile.write_all(word.as_bytes()).expect("invalid outfile!");
                    outfile.write_all(phonemes_str.as_bytes()).expect("can't write!");
                    },
                None => {
                    println!("no word {} ", word);
                    missing.insert(word);
                }
            };
        }
    }
    let mut missing_file = File::create("missing.txt")?;
    for mut word in missing {
        word.push('\n');
        missing_file.write_all(word.as_bytes()).unwrap();
    }
    Ok(())
}