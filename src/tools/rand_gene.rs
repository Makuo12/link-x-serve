use rand::{thread_rng, Rng};

fn random_char() -> char {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = thread_rng();
    let idx = rng.gen_range(0..CHARSET.len());
    CHARSET[idx] as char
}

fn random_string(length: usize) -> String {
    (0..length).map(|_| random_char()).collect()
}