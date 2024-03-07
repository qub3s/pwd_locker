#![allow(warnings, unused)]

mod crypt;
use crypt::keyexpansion as keyexpansion;
use crypt::printstate as printstate;
use crypt::encrypt as encrypt;
use crypt::decrypt as decrypt;

use std::io::stdin;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;

fn create_pwd_storage( filename : &str) {
    let mut file = match File::create(filename) {
        Err(e) => panic!("Can't create file"),
        Ok(f) => f,
    };
    
    match file.write_all(b"storage"){
        Err(e) => panic!("Can't write file"),
        Ok(f) => f,
    };
}

fn read_file( path: String, pwd : &mut String, category : &mut Vec<String>, map : &mut BTreeMap<String, String> ){
    let path = Path::new(path.as_str());

    let mut file = match File::open(path) {
        Err(e) => panic!("Can't open file: {}",e),
        Ok(f) => f,
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer){
        Err(e) => panic!("Can't read file! {}",e),
        Ok(f) => f,
    };

    let lines : Vec<&str> = buffer.split('\n').collect();
    let tmp = lines[0].as_bytes();

    for i in 0..tmp.len(){
        pwd.push(tmp[i] as char);
    }

    let temp_category : Vec<&str> = lines[1].split(',').collect();
   
    // copy the array and convert "str" to String 
    for i in temp_category{
        if i != ""{
            category.push(String::from(i));
        }
    }

    for i in 2..lines.len()-1{
        let tmp = match lines[i].split_once(':'){
            Some(t) => t,
            None => panic!("There is a error in this file in line {} !",i),
        };
        map.insert(String::from( tmp.0 ), String::from( tmp.1 ));
    }
    
    println!("opened file");
    drop(file);
}

fn write_file( path: String, pwd : &mut String, category : &mut Vec<String>, map : &mut BTreeMap<String, String> ){
    let path = Path::new(path.as_str());

    let mut file = match File::create(path){
        Err(e) => panic!("Can't open file: {}",e),
        Ok(f) => f,
    };

    let mut buffer = String::new();

    let password = pwd.as_bytes();
    for i in 0..password.len(){
        buffer.push(password[i] as char);
    }

    buffer.push('\n');

    for i in 0..category.len(){
        let tmp = category[i].as_bytes();
        if tmp != "".as_bytes(){
            for j in 0..tmp.len(){
                buffer.push(tmp[j] as char);
            }
            buffer.push(',');
        }
    }
    
    buffer.push('\n');

    let mut iterator = map.iter();

    for x in map{
        let t = format!("{}:{}",x.0,x.1);
        let tmp = t.as_bytes();
        
        for i in 0..tmp.len(){
            buffer.push(tmp[i] as char);
        }
        buffer.push('\n');
    }

    match file.write_all(&mut buffer.as_bytes()){
        Err(e) => panic!("Can't write to file! {}",e),
        Ok(f) => f,
    };

    drop(file);
}

fn show_category( query : String , map : &mut BTreeMap<String, String> ){
    let out = map.get(&query);

    if out == None{
        println!("This Element doesn't exist !");
        return;
    }

    let lines : Vec<&str> = out.unwrap().split('\n').collect();
    
    for x in lines{
        println!("{}", x);
    }
}

fn list( category : &mut Vec<String> ){
    for x in category{
        print!("{}  ", x);
    }
    println!();
}

fn add( category : &mut Vec<String>, new_category : String, map: &mut BTreeMap<String, String>){
    category.push(new_category.clone());
    map.insert( new_category, String::from("") );
    return;
}

fn rm( rm_category : String, category : &mut Vec<String>, map : &mut BTreeMap<String, String> ){
    for x in 0..category.len(){
        if rm_category == category[x]{
            category.remove(x);
        }
    }

    map.remove(rm_category.as_str());
}




//fn cat( category : String, map : &mut BTreeMap<String, String> )

/*
 * first line - the password
 * second line - category csv list
 * one line for every category entry ( category: "query":"key","query":"key")
 *
 * Commands:
 * setpwd <pwd>                                 : sets the pwd the file is read and written with
 * read <filename>                              : reads a File
 * write <filename>                             : writes a File
 * cat <category>                               : prints entry to screen
 * add <category>                               : adds a category
 * ls                                           : list all categorys
 * remove <category>                            : removes a category
 * change <category> <key> <value>              : changes key value of key if value is delete, removes key
 *
*/



// get commandline filename and password as input
fn main(){
    let mut pwd : String = String::new();
    let mut map : BTreeMap<String, String> = BTreeMap::new();
    let mut category : Vec<String> = Vec::new();

    println!("Password Locker:");

    while true{

        print!("> ");
        
        let mut input = String::new();
        let stdin = io::stdin();

        io::stdout().flush().unwrap();

        match stdin.read_line(&mut input){
            Ok(x) => x,
            Err(e) => panic!("Can't read from stdin"),
        };

        
        if input.starts_with("exit"){
            break;
        }

        let mut c = match input.split_once(' '){
            Some(t) => t,
            None => (input.as_str(),""),
        };

        let string0 = String::from(c.0).replace("\n","");
        let string1 = String::from(c.1).replace("\n","");
        
        c.0 = string0.as_str();
        c.1 = string1.as_str();

        let command = c.0.clone();

        match command{
            "create" => create_pwd_storage(c.1),
            "setpwd" => pwd = String::from(c.1),
            "read" => read_file(String::from(c.1), &mut pwd, &mut category, &mut map),
            "write" => write_file(String::from(c.1), &mut pwd, &mut category, &mut map),
            "cat" => show_category(String::from(c.1), &mut map),
            "ls" => list( &mut category ),
            "add" => add( &mut category, String::from(c.1) , &mut map),
            "rm" => rm( String::from(c.1), &mut category, &mut map ),
            "change" => print!("change"),
            _ => println!("   Unknown"),
        };
    }

    //read_file( &mut pwd, &mut category, &mut map );
    //show_category( String::from("cat2") , map);

    //write_file( &mut pwd, &mut category, &mut map );
    //create_pwd_storage();
    
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
