use rustler::{Error};
use libsodium_sys as sodium;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        invalid_scalar_length,
        invalid_input_length,
        computation_failed,
    }
}

#[rustler::nif]
fn scalarmult_ed25519_base_noclamp(scalar: Vec<u8>) -> Result<Vec<u8>, Error> {
    // Check if scalar has correct length (32 bytes)
    if scalar.len() != sodium::crypto_scalarmult_ed25519_SCALARBYTES as usize {
        return Err(Error::Term(Box::new(atoms::invalid_scalar_length())));
    }

    let mut result = vec![0u8; sodium::crypto_scalarmult_ed25519_BYTES as usize];
    
    let ret = unsafe {
        sodium::crypto_scalarmult_ed25519_base_noclamp(
            result.as_mut_ptr(),
            scalar.as_ptr()
        )
    };

    if ret == 0 {
        Ok(result)
    } else {
        Err(Error::Term(Box::new(atoms::computation_failed())))
    }
}

#[rustler::nif]
fn hash_sha512(input: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut hash = vec![0u8; sodium::crypto_hash_sha512_BYTES as usize];
    
    let ret = unsafe {
        sodium::crypto_hash_sha512(
            hash.as_mut_ptr(),
            input.as_ptr(),
            input.len() as u64
        )
    };

    if ret == 0 {
        Ok(hash)
    } else {
        Err(Error::Term(Box::new(atoms::computation_failed())))
    }
}

#[rustler::nif]
fn scalar_reduce(scalar: Vec<u8>) -> Result<Vec<u8>, Error> {
    // Input should be 64 bytes (non-reduced scalar)
    if scalar.len() != sodium::crypto_core_ed25519_NONREDUCEDSCALARBYTES as usize {
        return Err(Error::Term(Box::new(atoms::invalid_input_length())));
    }

    let mut result = vec![0u8; sodium::crypto_core_ed25519_SCALARBYTES as usize];
    
    unsafe {
        sodium::crypto_core_ed25519_scalar_reduce(
            result.as_mut_ptr(),
            scalar.as_ptr()
        );
    }

    Ok(result)
}

#[rustler::nif]
fn scalar_mul(scalar_a: Vec<u8>, scalar_b: Vec<u8>) -> Result<Vec<u8>, Error> {
    // Both scalars should be 32 bytes
    if scalar_a.len() != sodium::crypto_core_ed25519_SCALARBYTES as usize ||
       scalar_b.len() != sodium::crypto_core_ed25519_SCALARBYTES as usize {
        return Err(Error::Term(Box::new(atoms::invalid_scalar_length())));
    }

    let mut result = vec![0u8; sodium::crypto_core_ed25519_SCALARBYTES as usize];
    
    unsafe {
        sodium::crypto_core_ed25519_scalar_mul(
            result.as_mut_ptr(),
            scalar_a.as_ptr(),
            scalar_b.as_ptr()
        );
    }

    Ok(result)
}

#[rustler::nif]
fn scalar_add(scalar_a: Vec<u8>, scalar_b: Vec<u8>) -> Result<Vec<u8>, Error> {
    // Both scalars should be 32 bytes
    if scalar_a.len() != sodium::crypto_core_ed25519_SCALARBYTES as usize ||
       scalar_b.len() != sodium::crypto_core_ed25519_SCALARBYTES as usize {
        return Err(Error::Term(Box::new(atoms::invalid_scalar_length())));
    }

    let mut result = vec![0u8; sodium::crypto_core_ed25519_SCALARBYTES as usize];
    
    unsafe {
        sodium::crypto_core_ed25519_scalar_add(
            result.as_mut_ptr(),
            scalar_a.as_ptr(),
            scalar_b.as_ptr()
        );
    }

    Ok(result)
}

rustler::init!("Elixir.ExSodium.LibSodium");
