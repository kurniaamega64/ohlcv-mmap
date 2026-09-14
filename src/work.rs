use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: String,
    pub blob: Vec<u8>,
}

pub fn make_job(seed: &str) -> Job {
    let mut h = DefaultHasher::new();
    seed.hash(&mut h);
    let n = h.finish();
    Job {
        id: format!("{:08x}", n as u32),
        blob: n.to_be_bytes().to_vec(),
    }
}

pub fn hash_nonce(blob: &[u8], nonce: u32) -> u64 {
    let mut h = DefaultHasher::new();
    blob.hash(&mut h);
    nonce.hash(&mut h);
    h.finish()
}

pub fn bench(rounds: u32) -> (String, u32) {
    let job = make_job("sha256");
    for n in 0..rounds {
        let _ = hash_nonce(&job.blob, n);
    }
    (job.id, rounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_stable() {
        assert_eq!(make_job("a").id, make_job("a").id);
    }

    #[test]
    fn nonce_changes() {
        let j = make_job("a");
        assert_ne!(hash_nonce(&j.blob, 1), hash_nonce(&j.blob, 2));
    }

    #[test]
    fn bench_rounds() {
        let (_, n) = bench(8);
        assert_eq!(n, 8);
    }
}
