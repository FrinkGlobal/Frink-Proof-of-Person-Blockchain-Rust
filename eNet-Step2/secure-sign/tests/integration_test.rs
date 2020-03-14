use secure_sign::NistCryptography;

#[test]
fn tests() {

    let mut _secure: NistCryptography = NistCryptography::new();

    _secure.init();
    _secure.generate_keypair();

    print!("public key[{}]: ", _secure.public_key.len());

    for i in 0.._secure.public_key.len() {
        print!("{:02X}", _secure.public_key[i]);
    }

    print!("\n\nprivate key[{}] : ", _secure.private_key.len());

    for i in 0.._secure.private_key.len() {
        print!("{:02X}", _secure.private_key[i]);
    }
    print!("\n", );
}
