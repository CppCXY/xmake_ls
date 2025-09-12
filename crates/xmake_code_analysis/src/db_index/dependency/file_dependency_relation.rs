use crate::FileId;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct FileDependencyRelation<'a> {
    dependencies: &'a HashMap<FileId, HashSet<FileId>>,
}

impl<'a> FileDependencyRelation<'a> {
    pub fn new(dependencies: &'a HashMap<FileId, HashSet<FileId>>) -> Self {
        Self { dependencies }
    }

    pub fn get_best_analysis_order(&self, file_ids: Vec<FileId>) -> Vec<FileId> {
        if self.dependencies.is_empty() {
            return file_ids;
        }

        let file_set: HashSet<_> = file_ids.iter().copied().collect();

        let mut in_degree: HashMap<FileId, usize> = HashMap::new();
        let mut adjacency: HashMap<FileId, Vec<FileId>> = HashMap::new();

        for file_id in &file_ids {
            if let Some(deps) = self.dependencies.get(file_id) {
                adjacency.entry(*file_id).or_default();
                for &dep in deps {
                    if file_set.contains(&dep) {
                        adjacency.entry(dep).or_default().push(*file_id);
                        *in_degree.entry(*file_id).or_default() += 1;
                    }
                }
            } else {
                adjacency.entry(*file_id).or_default();
                in_degree.entry(*file_id).or_default();
            }
        }

        let mut queue = VecDeque::new();
        let mut sorted_keys: Vec<_> = adjacency.keys().copied().collect();
        sorted_keys.sort();
        for &file in &sorted_keys {
            if *in_degree.get(&file).unwrap_or(&0) == 0 {
                queue.push_back(file);
            }
        }

        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node);
            if let Some(neighbors) = adjacency.get(&node) {
                for &n in neighbors {
                    if let Some(x) = in_degree.get_mut(&n) {
                        *x = x.saturating_sub(1);
                        if *x == 0 {
                            queue.push_back(n);
                        }
                    }
                }
            }
        }

        let processed: HashSet<_> = order.iter().copied().collect();
        for file in file_ids {
            if !processed.contains(&file) {
                order.push(file);
            }
        }
        order
    }

    /// Get all direct and indirect dependencies for the file list
    pub fn collect_file_dependents(&self, file_ids: Vec<FileId>) -> Vec<FileId> {
        let mut reverse_map: HashMap<FileId, Vec<FileId>> = HashMap::new();
        for (&fid, deps) in self.dependencies.iter() {
            for &dep in deps {
                reverse_map.entry(dep).or_default().push(fid);
            }
        }
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        for file_id in file_ids {
            queue.push_back(file_id);
        }
        while let Some(file_id) = queue.pop_front() {
            if let Some(dependents) = reverse_map.get(&file_id) {
                for &d in dependents {
                    if result.insert(d) {
                        queue.push_back(d);
                    }
                }
            }
        }
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_analysis_order() {
        let mut map = HashMap::new();
        map.insert(FileId::new(1), {
            let mut s = HashSet::new();
            s.insert(FileId::new(2));
            s
        });
        map.insert(FileId::new(2), HashSet::new());
        let rel = FileDependencyRelation::new(&map);
        let result = rel.get_best_analysis_order(vec![FileId::new(1), FileId::new(2)]);
        assert_eq!(result, vec![FileId::new(2), FileId::new(1)]);
    }

    #[test]
    fn test_best_analysis_order2() {
        let mut map = HashMap::new();
        map.insert(1.into(), {
            let mut s = HashSet::new();
            s.insert(2.into());
            s.insert(3.into());
            s
        });
        map.insert(2.into(), {
            let mut s = HashSet::new();
            s.insert(3.into());
            s
        });
        let rel = FileDependencyRelation::new(&map);
        let result = rel.get_best_analysis_order(vec![1.into(), 2.into(), 3.into()]);
        assert_eq!(result, vec![3.into(), 2.into(), 1.into()]);
    }

    #[test]
    fn test_collect_file_dependents() {
        let mut deps = HashMap::new();
        deps.insert(
            FileId::new(1),
            [FileId::new(2), FileId::new(3)].iter().cloned().collect(),
        );
        deps.insert(FileId::new(2), [FileId::new(3)].iter().cloned().collect());
        deps.insert(FileId::new(3), HashSet::new());
        deps.insert(FileId::new(4), [FileId::new(3)].iter().cloned().collect());

        let rel = FileDependencyRelation::new(&deps);
        let mut result = rel.collect_file_dependents(vec![FileId::new(3)]);
        result.sort();
        assert_eq!(result, vec![FileId::new(1), FileId::new(2), FileId::new(4)]);
    }
}
