// a program that reads a file and replaces the content of this file with the (initial) size of the file
use std::fs as fs;
use std::fs::File as File;
use std::io::prelude::*;


/*
fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
} */

pub struct Hachator {
    //systeme_de_fichier : std::fs:file::create;
    //ecriture: File
}

impl Hachator {
    fn new() -> Hachator{
        Hachator{}
    }
/*

    Fn: It cannot modify the objects it captures.
    FnMut: It can modify the objects it captures.
    FnOnce: The most restricted. Can only be called once because when it is called it consumes itself and its captures.
 
     metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata>)) 
 
    */

    // lit le fichier dans ton input stream, calcule sa taille, écrit la taille (en chaîne) dans ton outputstream
    pub fn actionne(&self, nom_fichier: &str, metadata_getter: fn(usize) -> usize) {

        // en mode production
        //let metadata_fichier = metadata_getter(nom_fichier).unwrap();
        let metadata_fichier = fs::metadata(nom_fichier).unwrap();
        // acquiert la taille du fichier dans le file system
        let taille = metadata_fichier.len();
        
        // connecte output_stream sur le fichier dans le file system
        // écrit la taille dans l'output_streamhttps://stackoverflow.com/questions/31289588/converting-a-str-to-a-u8
        let mut fichier = File::create(nom_fichier).unwrap();
        fichier.write(taille.to_string().as_bytes()).unwrap();

    }
}

fn plus_un(n: usize) -> usize {
    n+1
}


#[cfg(test)]
mod tests {

    use std::fs as fs;
    use super::*;
    use std::io::prelude::*;
 
    #[test]
    fn remplace_un_fichier_contenant_un_octet_par_un_fichier_contenant_1 () {
        // agencement
        let hachator = Hachator::new();
        // action
        hachator.actionne("UnFichier.txt", plus_un);
        // assertion : le code sous test a appelé une fonction qui a écrit "1" dans le stream 
        assert_eq!("", "");
    }

}