use crate::size::Size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::read_dir;

#[derive(PartialEq, Debug)]
pub struct FileTree {
    root: PathBuf,
    map: HashMap<PathBuf, EntryNode>,
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]

/// EntryNode est un noeud de l'arbre qui est soit un fichier soir un dossier
enum EntryNode {Fichier{nom: PathBuf, taille: Size}, Dossier{nom: PathBuf, enfant: Vec<PathBuf>}}

impl EntryNode {
    pub fn new(path: &Path) -> std::io::Result<Self> {
        if path.is_dir() {
            let mut enf = Vec::<PathBuf>::new();
            for entry in read_dir(path)? {
                let path = entry?.path();
                enf.push(path);
            }
            Ok(EntryNode::Dossier{nom: path.to_path_buf(), enfant: enf})
        } else if path.is_file() {
            let taille = path.metadata()?;
            Ok(EntryNode::Fichier{nom :path.to_path_buf(), taille: Size::new(taille.len())})
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a valid path"))
        }
    }
}

impl FileTree {
    /// permet d'instancier un nouvel arbre à partir d'une path donné, renvoie une erreur si la path est invalide
    pub fn new(root: &Path) -> std::io::Result<Self> {
        let mut map = HashMap::<PathBuf, EntryNode>::new();
        if root.is_dir() {
            map.insert(root.to_path_buf(), EntryNode::new(root).unwrap());
            FileTree::visit_dir(&mut map, root);
            Ok(FileTree { root: root.to_path_buf(), map })
        } else if root.is_file() {
            map.insert(root.to_path_buf(), EntryNode::new(root).unwrap());
            Ok(FileTree { root: root.to_path_buf(), map })
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a valid path"))
        }
        
    }

    /// visite récursivement chaque dossier et ajoute dans la HashMap l'EntryNode du fichier ou dossier trouvé
    fn visit_dir(map: &mut HashMap::<PathBuf, EntryNode>, path: &Path) {
            let entry = EntryNode::new(path).unwrap();
            match &entry.clone() {
                EntryNode::Dossier { nom, enfant } => {
                    map.insert(nom.clone(), entry);
                    for enf in enfant {
                        FileTree::visit_dir(map, enf);
                        }
                    },
                EntryNode::Fichier { nom, taille: _ } => {
                    map.insert(nom.clone(), entry);
                }
            }
        }

    /// retourne le chemin de la racine de l'arbre
    pub fn get_root(&self) -> &Path {
        &self.root
    }

    /// donne la liste des enfants du path donné si le path est celui d'un dossier
    pub fn get_children(&self, path: &Path) -> Option<&[PathBuf]> {
        match &self.is_valid_path(path) {
            Ok(_) => {
                let map = &self.map.get(path).unwrap();
                match map {
                    EntryNode::Dossier { nom: _, enfant } => Some(enfant),
                    EntryNode::Fichier { nom: _, taille: _ } => None
                }
            }
            Err(_) => None
        }
    }

    /// donne la taille du path donné si il existe
    pub fn get_size(&self, path: &Path) -> Option<Size> {
        match &self.is_valid_path(path) {
            Ok(_) => {
                let mut size = Size::new(0);
                for entry in &self.map {
                    if entry.0.starts_with(path) {
                        match entry.1 {
                            EntryNode::Dossier { nom: _, enfant: _ } => continue,
                            EntryNode::Fichier { nom: _, taille } => size = size + *taille
                        }
                    } else {
                        continue;
                    }
                }
                Some(size)
            }
            Err(_) => None
        }
    }

    /// retourne la liste des fichiers contenue dans l'arbre
    pub fn files(&self) -> Vec<PathBuf> {
        let mut vec_fic = Vec::<PathBuf>::new();
        for entry in &self.map {
            match entry.1 {
                EntryNode::Dossier { nom: _, enfant: _ } => continue,
                EntryNode::Fichier { nom, taille: _ } => vec_fic.append(&mut vec![nom.to_path_buf()])
            }
        }
        vec_fic

    }

    /// retourne la liste des dossiers contenu dans l'arbre ainsi que leur taille
    pub fn directory(&self) -> Vec<(Size, PathBuf)> {        
        let mut vec_dir = Vec::<(Size, PathBuf)>::new();
        for entry in &self.map {
            match entry.1 {
                EntryNode::Dossier { nom, enfant: _ } => {
                    let taille = FileTree::get_size(self, nom).unwrap();
                    vec_dir.append(&mut vec![(taille, nom.to_path_buf())]);
                },
                EntryNode::Fichier { nom: _, taille: _ } => continue
            }
        }
        vec_dir
    }

    /// vérifie si une path donné en argument est contenue dans l'arbre ou si elle est invalide
    fn is_valid_path(&self, path: &Path) -> Result<(),std::io::Error> {
        match &self.map.get(path) {
            Some(_) => Ok(()),
            None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a valid path"))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_create_entry_node() {
        let a = EntryNode::new(Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin/dossloin2/txt1.txt"));
        assert_eq!(a.unwrap(),
        EntryNode::Fichier { nom: Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin/dossloin2/txt1.txt").to_path_buf(), taille: Size::new(1)});
        let a = EntryNode::new(Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin"));
        let vec: Vec<PathBuf> = vec![Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin/dossloin2").to_path_buf()];
        assert_eq!(a.unwrap(),
        EntryNode::Dossier { nom: Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin").to_path_buf(), enfant: (vec) });
    }
    #[test]
    fn not_valid_path() {
        let a = EntryNode::new(Path::new("Not a valid path"));
        assert!(a.is_err())
    }

    #[test]
    fn should_create_file_tree() {
        let tree = FileTree::new(Path::new("C:/programation/L3/IN 512 prog système/testt"));
        assert!(tree.is_ok());
        assert!(FileTree::new(Path::new("erreur")).is_err());
    }
    #[test]
    fn should_get_root() {
        let tree = FileTree::new(Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin"));
        assert_eq!(tree.unwrap().get_root(), Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin"))
    }

    #[test]
    fn should_get_children() {
        let tree = FileTree::new(Path::new("C:/programation/L3/IN 512 prog système/testt")).unwrap();
        let res = vec![Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin/dossloin2")];
        assert_eq!(tree.get_children(Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin")).unwrap(), res);
        assert!(tree.get_children(Path::new("Not a valid path")).is_none())
    }

    #[test]
    fn should_return_files() {
        let tree = FileTree::new(Path::new("C:/programation/L3/IN 512 prog système/testt")).unwrap();
        let mut res = vec![Path::new("C:/programation/L3/IN 512 prog système/testt\\doss1\\txt2.txt").to_path_buf(), Path::new("C:/programation/L3/IN 512 prog système/testt\\doss1\\dossloin\\txt1.txt").to_path_buf(), Path::new("C:/programation/L3/IN 512 prog système/testt\\doss2\\txt3.txt").to_path_buf(), Path::new("C:/programation/L3/IN 512 prog système/testt\\doss1\\dossloin\\dossloin2\\txtloin.txt").to_path_buf()];
        assert_eq!(tree.files().sort(), res.sort());
    }

    #[test]
    fn should_get_size() {
        let tree = FileTree::new(Path::new("C:/programation/L3/IN 512 prog système/testt")).unwrap();
        let res = Size::new(1887);
        assert_eq!(tree.get_size(Path::new("C:/programation/L3/IN 512 prog système/testt/doss1")).unwrap(), res);
        let res = Size::new(119);
        assert_eq!(tree.get_size(Path::new("C:/programation/L3/IN 512 prog système/testt/doss1/dossloin")).unwrap(), res);
        assert!(tree.get_size(Path::new("Not a Valid path")).is_none());
    }
}