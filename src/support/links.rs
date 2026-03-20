use smallvec::SmallVec;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default)]
pub struct Links<T, const N: usize>
where
    T: Hash + Eq + Copy + Ord,
{
    adj: HashMap<T, SmallVec<[T; N]>>,
}

impl<T, const N: usize> Links<T, N>
where
    T: Hash + Eq + Copy + Ord,
{
    pub fn new() -> Self {
        Self {
            adj: HashMap::new(),
        }
    }

    /// Adds an undirected link.
    /// Stores both directions to keep neighbor lookups fast.
    pub fn add_link(&mut self, a: T, b: T) {
        if a == b {
            return;
        } // Avoid self-loops if desired

        self.adj.entry(a).or_insert_with(SmallVec::new).push(b);
        self.adj.entry(b).or_insert_with(SmallVec::new).push(a);
    }

    /// Removes the link in both directions.
    pub fn remove_link(&mut self, a: T, b: T) {
        self.remove_half(a, b);
        self.remove_half(b, a);
    }

    fn remove_half(&mut self, src: T, target: T) {
        if let Some(neighbors) = self.adj.get_mut(&src) {
            if let Some(pos) = neighbors.iter().position(|&x| x == target) {
                neighbors.swap_remove(pos);
            }
            if neighbors.is_empty() {
                self.adj.remove(&src);
            }
        }
    }

    /// Iterator over all connections of ONE specific node.
    pub fn neighbors(&self, node: T) -> impl Iterator<Item = T> + '_ {
        self.adj
            .get(&node)
            .into_iter()
            .flat_map(|neighbors| neighbors.iter().copied())
    }

    /// Iterator over ALL unique connections in the graph.
    /// Uses the Ord constraint to ensure (a, b) is only yielded once.
    pub fn all_links(&self) -> impl Iterator<Item = (T, T)> + '_ {
        self.adj.iter().flat_map(|(&u, neighbors)| {
            neighbors
                .iter()
                .filter(move |&&v| u < v) // Only yield if key is the smaller element
                .map(move |&v| (u, v))
        })
    }

    /// Merges another Links structure into this one (Union).
    /// Since T is Copy and nodes are sparse, this is very efficient.
    pub fn merge(&mut self, other: &Self) {
        // We leverage the all_links iterator we built.
        // It already ensures we only process each unique edge once (where a < b).
        for (a, b) in other.all_links() {
            self.add_link(a, b);
        }
    }
}
