use crate::file_tree::FileTree;
use std::path::{Path, PathBuf};
use crate::size::Size;


impl FileTree {
    /// affiche l'arbre trié en fonction de la taille des dossiers / fichiers de manière décroissante
    pub fn show(&self) {
        println!("{}\t{}", &self.get_size(self.get_root()).unwrap(), &self.get_root().display());
        self.afficher_decroissant(self.get_root(), 0)   
    }

    /// affiche l'arbre trié en fonction de la lexicographie des dossiers / fichiers
    pub fn show_lexicographic(&self) {
        println!("{}\t{}", &self.get_size(self.get_root()).unwrap(), &self.get_root().display());
        self.afficher_lexicographie(self.get_root(), 0)   
    }

    /// affiche l'arbre filtré avec un tri sur les tailles de dossiers et fichiers
    pub fn show_filter(&self, filter: &PathBuf) {
        let mut tab: Vec<(String, Size, PathBuf)> = vec![];
        self.afficher_filtre(self.get_root(), 0, filter, &mut tab);
        if !tab.is_empty() {
            println!("{}\t{}", &self.get_size(self.get_root()).unwrap(), &self.get_root().display());
            for elem in tab {
                println!("{}{}\t{}", elem.0, elem.1, elem.2.display());

            }
        }
    }

    /// affiche l'arbre filtré avec un tri lexicographique sur les dossiers et fichiers
    pub fn show_filter_lexicographic(&self, filter: &PathBuf) {
        let mut tab: Vec<(String, Size, PathBuf)> = vec![];
        self.afficher_filtre_lexicographic(self.get_root(), 0, filter, &mut tab);
        if !tab.is_empty() {
            println!("{}\t{}", &self.get_size(self.get_root()).unwrap(), &self.get_root().display());
            for elem in tab {
                println!("{}{}\t{}", elem.0, elem.1, elem.2.display());

            }
        }
    }

    /// effectue le tri décroissant sur l'arbre
    fn afficher_decroissant(&self, path: &Path, ecart: i32) {
        let mut enf: Vec<&PathBuf> = self.get_children(path).unwrap().iter().collect();
        let mut space = "\t".to_string();
        for _ in 0..ecart {
            space += "\t";
        }
        enf.sort_by(|a, b| (self.get_size(b).unwrap()).cmp(&self.get_size(a).unwrap()));
        for entry in enf {
            println!("{}{}\t{}", space, &self.get_size(entry).unwrap(), entry.display());
            if entry.is_dir() {
                self.afficher_decroissant(entry, ecart+1)
            }
        }
    }

    /// effectue le tri lexicographique sur l'arbre
    fn afficher_lexicographie(&self, path: &Path, ecart: i32) {
        let mut enf: Vec<&PathBuf> = self.get_children(path).unwrap().iter().collect();
        let mut space = "\t".to_string();
        for _ in 0..ecart {
            space += "\t";
        }
        enf.sort_by(|a, b| a.cmp(b));
        for entry in enf {
            println!("{}{}\t{}", space, &self.get_size(entry).unwrap(), entry.display());
            if entry.is_dir() {
                self.afficher_lexicographie(entry, ecart+1)
            }
        }
    }

    /// effectue le tri décroissant sur l'arbre et applique le filtre
    fn afficher_filtre(&self, path: &Path, ecart: i32, filter: &PathBuf, tab: &mut Vec<(String, Size, PathBuf)>) {
        let mut enf: Vec<&PathBuf> = self.get_children(path).unwrap().iter().collect();
        let mut space = "\t".to_string();
        let mut files = self.files();
        files.retain(|file| ".".to_owned() + file.extension().unwrap().to_str().unwrap() == filter.to_str().unwrap());
        for _ in 0..ecart {
            space += "\t";
        }
        enf.sort_by(|a, b| (self.get_size(b).unwrap()).cmp(&self.get_size(a).unwrap()));
        for entry in &enf {
            for elem in &files {
                if FileTree::new(entry).unwrap().files().contains(elem) {
                    tab.push((space.clone(), self.get_size(entry).unwrap(), entry.to_path_buf()));
                }
            }
            if entry.is_dir() {
                self.afficher_filtre(entry, ecart+1, filter, tab)
            }
        }
        tab.dedup();
    }
    
    /// effectue le tri lexicographique sur l'arbre et applique le filtre
    fn afficher_filtre_lexicographic(&self, path: &Path, ecart: i32, filter: &PathBuf, tab: &mut Vec<(String, Size, PathBuf)>) {
        let mut enf: Vec<&PathBuf> = self.get_children(path).unwrap().iter().collect();
        let mut space = "\t".to_string();
        let mut files = self.files();
        files.retain(|file| ".".to_owned() + file.extension().unwrap().to_str().unwrap() == filter.to_str().unwrap());
        for _ in 0..ecart {
            space += "\t";
        }
        enf.sort_by(|a, b| a.cmp(b));
        for entry in &enf {
            for elem in &files {
                if FileTree::new(entry).unwrap().files().contains(elem) {
                    tab.push((space.clone(), self.get_size(entry).unwrap(), entry.to_path_buf()));
                }
            }
            if entry.is_dir() {
                self.afficher_filtre_lexicographic(entry, ecart+1, filter, tab)
            }
        }
        tab.dedup();
    }
}
