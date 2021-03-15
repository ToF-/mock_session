// a program that reads a file and replaces the content of this file with the (initial) size of the file
use std::fs;
use std::fs::File;
use std::fs::Metadata;
use std::io::prelude::*;

/*
        let hachator = Hachator::new();
*/

fn main() {
    actionne("real_file.txt", fs::metadata);
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
}

// fn toto(a : &'a Titi) -> &'a Tata {}

/*

Fn: It cannot modify the objects it captures.
FnMut: It can modify the objects it captures.
FnOnce: The most restricted. Can only be called once because when it is called it consumes itself and its captures.

 metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata>))
*/

// lit le fichier dans ton input stream, calcule sa taille, écrit la taille (en chaîne) dans ton outputstream
pub fn actionne<'a, L: Lenny>(
    nom_fichier: &'a str,
    metadata: fn(&'a str) -> std::io::Result<L>,
) {
    // en mode production
    //let metadata_fichier = metadata_getter(nom_fichier).unwrap();
    let metadata_fichier = metadata(nom_fichier).unwrap();
    // acquiert la taille du fichier dans le file system
    let taille = metadata_fichier.len();

    // connecte output_stream sur le fichier dans le file system
    // écrit la taille dans l'output_streamhttps://stackoverflow.com/questions/31289588/converting-a-str-to-a-u8
    let mut fichier = File::create(nom_fichier).unwrap();
    fichier.write(taille.to_string().as_bytes()).unwrap();

    //fn write(&mut self, buf: &[u8]) -> Result<usize>
}

struct StubMetadata {}

pub trait Lenny {
    fn len(&self) -> u64; 
}

impl Lenny for StubMetadata {
    fn len(&self) -> u64 { 42 }
}

impl Lenny for fs::Metadata {
    fn len(&self) -> u64 { self.len() }
}

fn stub_file_size (n: &str) -> std::io::Result<StubMetadata> {
    Ok(StubMetadata{})
}




#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::io::prelude::*;

    #[test]
    fn remplace_un_fichier_contenant_un_octet_par_un_fichier_contenant_1() {
        // action
        actionne("UnFichier.txt", stub_file_size);
        // assertion : le code sous test a appelé une fonction qui a écrit "1" dans le stream
        assert_eq!("", "");
    }
}
