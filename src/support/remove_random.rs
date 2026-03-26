use hashbrown::{HashMap, HashSet, raw::{Bucket, RawTable}};
use std::hash::Hash;
use crate::evolution::Random;

pub trait RemoveRandom<TReturn>
{
    fn choose_random(&mut self, rng: &Random) -> Option<&TReturn>;
    fn remove_random(&mut self, rng: &Random) -> Option<TReturn>;
}



/// the the caller must ensure that the raw-table of the hash set outlives the bucket
/// credit: https://stackoverflow.com/a/77508258
unsafe fn rand_bucket<'a, T>(set: &'a mut HashSet<T>, rng: &Random) -> Option<(Bucket<(T,())>, &'a mut RawTable<(T,())>)> 
    where T: Eq + PartialEq + Hash
{
    if set.is_empty() {
        return None;
    }
    // If load factor is under 25%, shrink to fit.
    // We need a high load factor to ensure that the sampling succeeds in a reasonable time,
    // and the table doesn't rebalance on removals.
    // Insertions can only cause the load factor to reach as low as 50%,
    // so it's safe to shrink at 25%.
    if set.capacity() >= 8 && set.len() < set.capacity() / 4 {
        set.shrink_to_fit();
    }
    let raw_table = set.raw_table_mut();
    let num_buckets = raw_table.buckets();
    // Perform rejection sampling: Pick a random bucket, check if it's full,
    // repeat until a full bucket is found.
    loop {
        let bucket_index = rng.next_in_range(0, num_buckets as u64) as usize;
        // Safety: bucket_index is less than the number of buckets.
        // Note that we return the first time we modify the table,
        // so raw_table.buckets() never changes.
        // Also, the table has been allocated, because set is a HashSet.
        unsafe {
            if raw_table.is_bucket_full(bucket_index) {
                let bucket = raw_table.bucket(bucket_index);
                return Some((bucket, raw_table));
            }
        }
    }
}
unsafe fn rand_bucket_map<'a, T,E>(set: &'a mut HashMap<T,E>, rng: &Random) -> Option<(Bucket<(T,E)>, &'a mut RawTable<(T,E)>)> 
    where T: Eq + PartialEq + Hash
{
    if set.is_empty() {
        return None;
    }
    // If load factor is under 25%, shrink to fit.
    // We need a high load factor to ensure that the sampling succeeds in a reasonable time,
    // and the table doesn't rebalance on removals.
    // Insertions can only cause the load factor to reach as low as 50%,
    // so it's safe to shrink at 25%.
    if set.capacity() >= 8 && set.len() < set.capacity() / 4 {
        set.shrink_to_fit();
    }
    let raw_table = set.raw_table_mut();
    let num_buckets = raw_table.buckets();
    // Perform rejection sampling: Pick a random bucket, check if it's full,
    // repeat until a full bucket is found.
    loop {
        let bucket_index = rng.next_in_range(0, num_buckets as u64) as usize;
        // Safety: bucket_index is less than the number of buckets.
        // Note that we return the first time we modify the table,
        // so raw_table.buckets() never changes.
        // Also, the table has been allocated, because set is a HashSet.
        unsafe {
            if raw_table.is_bucket_full(bucket_index) {
                let bucket = raw_table.bucket(bucket_index);
                return Some((bucket, raw_table));
            }
        }
    }
}

impl<T> RemoveRandom<T> for HashSet<T>
where T: Eq + PartialEq + Hash
{
    fn choose_random(&mut self, rng: &Random) -> Option<&T> {
        unsafe {
            let (bucket, _) = rand_bucket(self, rng)?;
            Some(&bucket.as_ref().0)
        }
    }

    fn remove_random(&mut self, rng: &Random) -> Option<T> {
        unsafe {
            let (bucket, raw_table) = rand_bucket(self, rng)?;
            let ((element, ()), _insert_slot) = raw_table.remove(bucket);
            return Some(element);
        }
    }
}

impl<T,E> RemoveRandom<(T,E)> for HashMap<T,E>
where T: Eq + PartialEq + Hash
{
    fn choose_random(&mut self, rng: &Random) -> Option<&(T, E)> {
        unsafe {
            let (bucket, _) = rand_bucket_map(self, rng)?;
            Some(&bucket.as_ref())
        }
    }

    fn remove_random(&mut self, rng: &Random) -> Option<(T,E)> {
        unsafe {
            let (bucket, raw_table) = rand_bucket_map(self, rng)?;
            let (element, _insert_slot) = raw_table.remove(bucket);
            return Some(element);
        }
    }
}
