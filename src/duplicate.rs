use sha256::try_digest;
use crate::file_tree::FileTree;
use std::collections::HashMap;

impl FileTree {
    /// assigne une signature au contenu des fichiers de l'arbre pour ensuite retrouver les doublons 
    pub fn signature(&self) {
        let files = &self.files();
        let mut signatures = HashMap::new();
        for file in files {
            let signature = try_digest(file).unwrap();
            signatures.entry(signature).or_insert_with(Vec::new).push(file.clone());
        }
        for elem in &signatures {
            if elem.1.len() > 1 {
                println!("Les fichiers suivant sont identique :");
                for file in elem.1 {
                    println!("{}", file.display());
                }
            }
        }
    }
}
