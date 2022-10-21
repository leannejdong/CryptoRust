# Cryptography basic

## Hash?

md4, sha, argon2

## Salt

## HMAC

## Symmetric Encryption

## Keypairs

## Asymmetric Encryption

To summarise, 
The signing process
* Define a password
* Create a salt
* Prepend salt to password
* Create a signing key for HMAC  
* Hash password via HMAC (encrypt with priv key)
https://www.lpalmieri.com/posts/password-authentication-in-rust/
https://crypto.stackexchange.com/questions/12768/why-hash-the-message-before-signing-it-with-rsa

The signature verification process

* Decrypt the signature (with pub key)
* Compare the hash
* Continue this process down the certificate chain

