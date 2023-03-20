use rand::{distributions::{Alphanumeric, DistString}, thread_rng};



pub fn rand_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), len)
}

