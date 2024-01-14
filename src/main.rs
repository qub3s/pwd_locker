mod crypt;
use crypt::keyexpansion as keyexpansion;
use crypt::printstate as printstate;
use crypt::encrypt as encrypt;
use crypt::decrypt as decrypt;

use std::io::stdin;
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn create_pwd_storage() -> io::Result<()> {
    let mut file = match File::create("pwd") {
           Err(e) => return Err(e),
           Ok(f) => f,
    };
    match file.write_all(b"#storage"){
        Err(e) => return Err(e),
        Ok(f) => f,
    };

    Ok(())
}


fn create_pwd_storage() -> io::Result<()> {
    let mut file = match File::create("pwd") {
           Err(e) => return Err(e),
           Ok(f) => f,
    };
    match file.write_all(b"#"){
        Err(e) => return Err(e),
        Ok(f) => f,
    };

    Ok(())
}

/*
 * File:
 * lines that start with # are dirs
 * lines that start with ! are data
 *
 * Operations:
 * create dir ...
 * create data ...
 * delete ...
 * read ...
*/

fn main(){
    let mut dirs = Vec::new();
    let mut data = Vec::new();

    create_pwd_storage();
    
    /*
    let basekey   : [u32; 8] = [ 0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c ];
    let mut state : [u32; 4] = [ 0x3243f6a8, 0x885a308d, 0x313198a2, 0xe0370734 ];
    
    let mut enckey : [u32; 60] = [0; 60];
    let mut deckey : [u32; 60] = [0; 60];
    
    keyexpansion( &mut enckey, &mut deckey, &basekey);
    
    printstate(&state);

    encrypt( &mut state, &enckey, 1);

    printstate(&state);

    decrypt( &mut state, &deckey, 1);

    printstate(&state);

    println!("compiles");
 
    for line in stdin().lines() {
        print!("{}", line.unwrap());
        break;
    }
    */

}
