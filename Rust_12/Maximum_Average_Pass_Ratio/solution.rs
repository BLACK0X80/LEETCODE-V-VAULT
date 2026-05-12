use std::collections::BinaryHeap;
#[derive(PartialOrd, PartialEq)] struct Class(f64, i32, i32);
impl Eq for Class {} 
impl Ord for Class { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal) } }
impl Solution { pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra: i32) -> f64 { let mut black_h = classes.iter().map(|c| Class((c[0]+1) as f64/(c[1]+1) as f64 - c[0] as f64/c[1] as f64, c[0], c[1])).collect::<BinaryHeap<_>>(); for _ in 0..extra { if let Some(Class(_, p, t)) = black_h.pop() { black_h.push(Class((p+2) as f64/(t+2) as f64 - (p+1) as f64/(t+1) as f64, p+1, t+1)); } } black_h.iter().map(|&Class(_, p, t)| p as f64 / t as f64).sum::<f64>() / classes.len() as f64 } }