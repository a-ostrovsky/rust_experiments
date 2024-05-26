use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub fn create_random_distinct_vec(len: usize) -> Vec<i32> {
    let mut vec = (0..len as i32).collect::<Vec<_>>();
    vec.shuffle(&mut StdRng::seed_from_u64(0));
    vec
}
