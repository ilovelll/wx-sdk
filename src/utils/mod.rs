use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub(crate) mod base64;

pub(crate) fn get_random_string(size: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    rand_string
}
