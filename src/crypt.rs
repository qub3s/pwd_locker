#![allow(warnings, unused)]
#![allow(dead_code)]

pub mod tables;
use tables::S_BOX as S_BOX;
use tables::I_BOX as I_BOX;
use tables::RCON as RCON;

use tables::D0 as D0;
use tables::D1 as D1;
use tables::D2 as D2;
use tables::D3 as D3;

use tables::E0 as E0;
use tables::E1 as E1;
use tables::E2 as E2;
use tables::E3 as E3;


const nr : u32 = 14;
const nk : u32 = 8;

fn rotWord( word : u32 ) -> u32{
    return (word << 8) ^ (word >> 24);
}

fn subWord( word : u32 ) -> u32{
    return (S_BOX[ (word >> 24) as usize] << 24) ^ (S_BOX[(word >> 16) as usize % 0x100] << 16) ^ (S_BOX[(word >> 8) as usize % 0x100] << 8) ^ (S_BOX[ (word % 0x100) as usize]);
}

pub fn keyexpansion( enckey : &mut[u32], deckey : &mut[u32], bkey : &[u32]){
    let mut i : u32 = 0;
    let mut t : u32;

    for j in 0..nk{
        enckey[j as usize] = bkey[j as usize];
    }

    i = nk;

    while(i < 4*(nr+1)){
        t = enckey[(i-1) as usize];
        
        if i % nk == 0 {
            t = subWord(rotWord(t)) ^ RCON[((i/nk)-1) as usize];
        }
        
        else if nk > 6 && i%nk == 4{
            t = subWord(t);
        }
        let a = i - nk;
        enckey[i as usize] = enckey[(i-nk) as usize] ^ t;
        i = i + 1;
    }

    for i in (0..(nr+1)*4).step_by(4){ 
		let ei = (nr+1)*4 - i - 4;
        for j in 0..4{
			t = enckey[(ei+j) as usize];
			if( i > 0 && i+4 < (nr+1)*4 ){
				t = D0[S_BOX[ (t >> 24 ) as usize] as usize] ^ D1[S_BOX[ (t>> 16 & 0xff) as usize] as usize] ^ D2[S_BOX[ ( t >> 8 & 0xff ) as usize ] as usize] ^ D3[ S_BOX[ ( t & 0xff) as usize] as usize];
            }
			deckey[(i+j) as usize] = t;
		}
	}
}


fn enc_block( state : &mut[u32], key : &[u32]){
    let mut t0 : u32;
    let mut t1 : u32;
    let mut t2 : u32;
    let mut t3 : u32;

    state[0] = state[0] ^ key[0];
    state[1] = state[1] ^ key[1];
    state[2] = state[2] ^ key[2];
    state[3] = state[3] ^ key[3];

    for i in 1..nr{
        t0 = E0[ ( state[0] >>24 ) as usize ] ^ E1[ ( state[1] >> 16 & 0xff ) as usize ] ^ E2[ ( state[2] >> 8 & 0xff ) as usize ] ^ E3[ ( state[3] & 0xff ) as usize ];
        t1 = E0[ ( state[1] >>24 ) as usize ] ^ E1[ ( state[2] >> 16 & 0xff ) as usize ] ^ E2[ ( state[3] >> 8 & 0xff ) as usize ] ^ E3[ ( state[0] & 0xff ) as usize ];
        t2 = E0[ ( state[2] >>24 ) as usize ] ^ E1[ ( state[3] >> 16 & 0xff ) as usize ] ^ E2[ ( state[0] >> 8 & 0xff ) as usize ] ^ E3[ ( state[1] & 0xff ) as usize ];
        t3 = E0[ ( state[3] >>24 ) as usize ] ^ E1[ ( state[0] >> 16 & 0xff ) as usize ] ^ E2[ ( state[1] >> 8 & 0xff ) as usize ] ^ E3[ ( state[2] & 0xff ) as usize ];
        state[0] = t0 ^ key[ ( i*4 ) as usize ];
        state[1] = t1 ^ key[ ( i*4+1 ) as usize ];
        state[2] = t2 ^ key[ ( i*4+2 ) as usize ];
        state[3] = t3 ^ key[ ( i*4+3 ) as usize ];
    }

    t0 = (S_BOX[ ( state[0] >> 24 ) as usize ] << 24) ^ (S_BOX[ ( state[1] >> 16 & 0xff ) as usize ] << 16) ^ (S_BOX[ ( state[2] >> 8 & 0xff ) as usize ] << 8) ^ S_BOX[ ( state[3] & 0xff ) as usize ];
    t1 = (S_BOX[ ( state[1] >> 24 ) as usize ] << 24) ^ (S_BOX[ ( state[2] >> 16 & 0xff ) as usize ] << 16) ^ (S_BOX[ ( state[3] >> 8 & 0xff ) as usize ] << 8) ^ S_BOX[ ( state[0] & 0xff ) as usize ];
    t2 = (S_BOX[ ( state[2] >> 24 ) as usize ] << 24) ^ (S_BOX[ ( state[3] >> 16 & 0xff ) as usize ] << 16) ^ (S_BOX[ ( state[0] >> 8 & 0xff ) as usize ] << 8) ^ S_BOX[ ( state[1] & 0xff ) as usize ];
    t3 = (S_BOX[ ( state[3] >> 24 ) as usize ] << 24) ^ (S_BOX[ ( state[0] >> 16 & 0xff ) as usize ] << 16) ^ (S_BOX[ ( state[1] >> 8 & 0xff ) as usize ] << 8) ^ S_BOX[ ( state[2] & 0xff ) as usize ];
    
    state[0] = t0 ^ key[ ( nr*4 ) as usize];
    state[1] = t1 ^ key[ ( nr*4+1 ) as usize];
    state[2] = t2 ^ key[ ( nr*4+2 ) as usize];
    state[3] = t3 ^ key[ ( nr*4+3 ) as usize];
    
    return;
}

//https://cs.opensource.google/go/go/+/master:src/crypto/aes/block.go;l=146?q=expansion&sq=&ss=go%2Fgo:src%2Fcrypto%2Faes%2F
fn dec_block( state : &mut[u32], key : &[u32]){
    let mut t0 : u32;
    let mut t1 : u32;
    let mut t2 : u32;
    let mut t3 : u32;
    
    state[0] = state[0] ^ key[0];
    state[1] = state[1] ^ key[1];
    state[2] = state[2] ^ key[2];
    state[3] = state[3] ^ key[3];

    for i in 1..nr{
        t0 = D0[ ( state[0] >>24 ) as usize ] ^ D1[ ( state[3] >> 16 & 0xff ) as usize ] ^ D2[ ( state[2] >> 8 & 0xff ) as usize ] ^ D3[ ( state[1] & 0xff ) as usize ];
        t1 = D0[ ( state[1] >>24 ) as usize ] ^ D1[ ( state[0] >> 16 & 0xff ) as usize ] ^ D2[ ( state[3] >> 8 & 0xff ) as usize ] ^ D3[ ( state[2] & 0xff ) as usize ];
        t2 = D0[ ( state[2] >>24 ) as usize ] ^ D1[ ( state[1] >> 16 & 0xff ) as usize ] ^ D2[ ( state[0] >> 8 & 0xff ) as usize ] ^ D3[ ( state[3] & 0xff ) as usize ];
        t3 = D0[ ( state[3] >>24 ) as usize ] ^ D1[ ( state[2] >> 16 & 0xff ) as usize ] ^ D2[ ( state[1] >> 8 & 0xff ) as usize ] ^ D3[ ( state[0] & 0xff ) as usize ];

        state[0] = t0 ^ key[ ( i*4 ) as usize ];
        state[1] = t1 ^ key[ ( i*4+1 ) as usize ];
        state[2] = t2 ^ key[ ( i*4+2 ) as usize ];
        state[3] = t3 ^ key[ ( i*4+3 ) as usize ];
    }

    t0 = (I_BOX[ ( state[0] >> 24 ) as usize ] << 24) ^ (I_BOX[ ( state[3] >> 16 & 0xff ) as usize ] << 16) ^ (I_BOX[ ( state[2] >> 8 & 0xff ) as usize ] << 8) ^ I_BOX[ ( state[1] & 0xff ) as usize ];
    t1 = (I_BOX[ ( state[1] >> 24 ) as usize ] << 24) ^ (I_BOX[ ( state[0] >> 16 & 0xff ) as usize ] << 16) ^ (I_BOX[ ( state[3] >> 8 & 0xff ) as usize ] << 8) ^ I_BOX[ ( state[2] & 0xff ) as usize ];
    t2 = (I_BOX[ ( state[2] >> 24 ) as usize ] << 24) ^ (I_BOX[ ( state[1] >> 16 & 0xff ) as usize ] << 16) ^ (I_BOX[ ( state[0] >> 8 & 0xff ) as usize ] << 8) ^ I_BOX[ ( state[3] & 0xff ) as usize ];
    t3 = (I_BOX[ ( state[3] >> 24 ) as usize ] << 24) ^ (I_BOX[ ( state[2] >> 16 & 0xff ) as usize ] << 16) ^ (I_BOX[ ( state[1] >> 8 & 0xff ) as usize ] << 8) ^ I_BOX[ ( state[0] & 0xff ) as usize ];
    
    state[0] = t0 ^ key[ ( nr*4 ) as usize];
    state[1] = t1 ^ key[ ( nr*4+1 ) as usize];
    state[2] = t2 ^ key[ ( nr*4+2 ) as usize];
    state[3] = t3 ^ key[ ( nr*4+3 ) as usize];
    
    return;
}

pub fn printstate( state : &[u32] ){
    for x in 0..4{
        println!("{:#08x}",state[x] );
    }
    println!();
    
    return;
}

// adds the block number to the block, so same blocks aren't encrypted the same way
pub fn encrypt( state: &mut[u32], key: &[u32], block_num: u32){
    state[0] = state[0] ^ block_num;
    enc_block( state, key );
}


pub fn decrypt( state: &mut[u32], key: &[u32], block_num: u32){
    dec_block( state, key );
    state[0] = state[0] ^ block_num;
}
