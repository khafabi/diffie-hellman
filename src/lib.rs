pub fn private_key(p: u64) -> u64 {
    (2..p).find(|&x| x > 1).unwrap_or(2)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponensial(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponensial(b_pub, a, p)
}

fn modular_exponensial(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    match exp {
        0 => 1,
        1 => base % modulus,
        _ => {
            let half_exp = modular_exponensial(base, exp / 2, modulus);
            let half_result = (half_exp * half_exp) % modulus;

            if exp % 2 == 0 {
                half_result
            } else {
                (half_result * (base % modulus)) % modulus
            }
        }
    }
}
