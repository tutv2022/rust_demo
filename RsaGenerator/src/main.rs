use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme, Pkcs1v15Encrypt};

use rand::rngs::OsRng;

fn testEncryptAndDecrypt() {
    
    // Test Data Enc/Decryption

    let mut rng = OsRng{};

    let bits = 2048;

    let privateKeyFileName = "./clientPrivate.key";

    let publicKeyFileName = "./clientPublic.key";


    let contents = std::fs::read_to_string(publicKeyFileName).unwrap();

    println!("Content: [{}]",contents);

    //let public_key : RsaPublicKey = pkcs1::DecodeRsaPublicKey::read_pkcs1_pem_file(&publicKeyFileName).unwrap();
    let public_key : RsaPublicKey = pkcs1::DecodeRsaPublicKey::from_pkcs1_pem(&contents).unwrap();
    let private_key : RsaPrivateKey = pkcs1::DecodeRsaPrivateKey::read_pkcs1_pem_file(&privateKeyFileName).unwrap();

    let data = b"hello world";

    //let newPrivateKey = pkcs1::DecodeRsaPrivateKey::from_pkcs1_pem(strPrivateKey.as_ref());

    let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);

    println!("Enc-data: {:?}", enc_data);

    let sparkle_heart = unsafe {
        std::str::from_utf8_unchecked(&enc_data)
    };;

    println!("sparkle_heart: {}",sparkle_heart);


    let my_bytes: Vec<u8> = sparkle_heart.as_bytes().to_vec();

    assert_eq!(&my_bytes[..], &enc_data[..]);


    let byteData = contents.as_bytes();

    println!("Convert: {:?}",my_bytes);

    println!("Content: {}", contents);

    
}


fn generateKeyPairs() {
    
    let mut rng = OsRng{};

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    pkcs1::EncodeRsaPrivateKey::write_pkcs1_pem_file(&private_key, "clientPrivate.key",pkcs1::LineEnding::LF).unwrap();
    pkcs1::EncodeRsaPublicKey::write_pkcs1_pem_file(&public_key, "clientPublic.key",pkcs1::LineEnding::LF).unwrap();


    let mut rng = OsRng{};

    let bits = 2048;
    let sv_private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let sv_public_key = RsaPublicKey::from(&sv_private_key);

    pkcs1::EncodeRsaPrivateKey::write_pkcs1_pem_file(&sv_private_key, "serverPrivate.key",pkcs1::LineEnding::LF).unwrap();
    pkcs1::EncodeRsaPublicKey::write_pkcs1_pem_file(&sv_public_key, "serverPublic.key",pkcs1::LineEnding::LF).unwrap();


}


fn main() {

    generateKeyPairs();
}