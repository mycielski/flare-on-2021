use std::{convert::TryFrom, process::exit};

fn main() {
    let png_header_bytes = png_headers();
    let result = find_key(png_header_bytes.0, png_header_bytes.1);
    let key: String = result.iter().map(|b| char::try_from(*b).unwrap()).collect();

    println!("The key is:\n{}", key);

    exit(0);
}

fn find_key(cryptobytes: Vec<u8>, plainbytes: Vec<u8>) -> Vec<u8> {
    assert_eq!(cryptobytes.len(), plainbytes.len());

    let mut decryptedbytes: Vec<u8> = Vec::new();
    let mut counter: u8 = 0x00;

    for i in 0..cryptobytes.len() {
        let cryptobyte = cryptobytes[i];
        let plainbyte = plainbytes[i];

        let mut keybyte: u8 = 0x00;

        while keybyte < 0xff {
            let mut eax = cryptobyte ^ keybyte;
            eax = u8::rotate_left(eax, counter.into());
            eax = u8::wrapping_sub(eax, counter);

            if eax == plainbyte {
                decryptedbytes.push(keybyte);
                break;
            }
            keybyte += 1;
        }
        counter += 1;
    }

    return decryptedbytes;
}

fn png_headers() -> (Vec<u8>, Vec<u8>) {
    let plainbytes = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
    let cryptobytes = vec![0xc7, 0xc7, 0x25, 0x1d, 0x63, 0x0d, 0xf3, 0x56];

    return (cryptobytes, plainbytes);
}
