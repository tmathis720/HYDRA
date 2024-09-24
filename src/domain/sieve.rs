use std::collections::{HashMap, HashSet};
use crate::domain::mesh_entity::MeshEntity;  // Assuming MeshEntity is defined in mesh_entity.rs

pub struct Sieve {
    pub adjacency: HashMap<MeshEntity, HashSet<MeshEntity>>, // Incidence relations (arrows)
}

impl Sieve {
    // Constructor to initialize an empty Sieve
    pub fn new() -> Self {
        Sieve {
            adjacency: HashMap::new(),
        }
    }

    // Adds an incidence (arrow) from one entity to another
    pub fn add_arrow(&mut self, from: MeshEntity, to: MeshEntity) {
        // Add the direct incidence relation
        self.adjacency.entry(from.clone()).or_insert_with(HashSet::new).insert(to.clone());
        
        // Also add the reverse relation to indicate that `to` is supported by `from`
        self.adjacency.entry(to).or_insert_with(HashSet::new).insert(from);
    }

    // Cone operation: Find points covering a given point
    pub fn cone(&self, point: &MeshEntity) -> Option<&HashSet<MeshEntity>> {
        self.adjacency.get(point)
    }

    // Closure operation: Transitive closure of cone
    pub fn closure(&self, point: &MeshEntity) -> HashSet<MeshEntity> {
        let mut result = HashSet::new();
        let mut stack = vec![point.clone()];
        while let Some(p) = stack.pop() {
            if let Some(cones) = self.cone(&p) {
                for q in cones {
                    if result.insert(q.clone()) {
                        stack.push(q.clone());
                    }
                }
            }
        }
        result
    }

    // Support operation: Find all points supported by a given point
    pub fn support(&self, point: &MeshEntity) -> HashSet<MeshEntity> {
        let mut result = HashSet::new();
        for (from, to_set) in &self.adjacency {
            if to_set.contains(point) {
                result.insert(from.clone());
            }
        }
        result
    }

    // Star operation: Transitive closure of support
    pub fn star(&self, point: &MeshEntity) -> HashSet<MeshEntity> {
        let mut result = HashSet::new();
        let mut stack = vec![point.clone()];  // Start with the point itself

        while let Some(p) = stack.pop() {
            if result.insert(p.clone()) {
                
                // Get all points that this point supports
                let support = self.support(&p);
                
                for q in support {
                    if !result.contains(&q) {
                        stack.push(q.clone());  // Add to stack if not already in the result set
                    }
                }
            }
        }

        println!("Star result for {:?}: {:?}", point, result);
        result
    }

    // Meet operation: Minimal separator of closure(p) and closure(q)
    pub fn meet(&self, p: &MeshEntity, q: &MeshEntity) -> HashSet<MeshEntity> {
        let closure_p = self.closure(p);
        let closure_q = self.closure(q);
        closure_p.intersection(&closure_q).cloned().collect()
    }

    // Join operation: Minimal separator of star(p) and star(q)
    pub fn join(&self, p: &MeshEntity, q: &MeshEntity) -> HashSet<MeshEntity> {

        let star_p = self.star(p);  // Get all entities related to p
        let star_q = self.star(q);  // Get all entities related to q

        // Return the union of both stars (the minimal separator)
        let join_result: HashSet<MeshEntity> = star_p.union(&star_q).cloned().collect();
        join_result
    }
}

// Unit tests for the Sieve structure and its operations

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::mesh_entity::MeshEntity;

    #[test]
    fn test_add_arrow_and_cone() {
        let mut sieve = Sieve::new();
        let vertex = MeshEntity::Vertex(1);
        let edge = MeshEntity::Edge(1);

        sieve.add_arrow(vertex, edge);
        let cone_result = sieve.cone(&vertex).unwrap();

        assert!(cone_result.contains(&edge));
    }

    #[test]
    fn test_closure() {
        let mut sieve = Sieve::new();
        let vertex = MeshEntity::Vertex(1);
        let edge = MeshEntity::Edge(1);
        let face = MeshEntity::Face(1);

        sieve.add_arrow(vertex, edge);
        sieve.add_arrow(edge, face);

        let closure_result = sieve.closure(&vertex);

        assert!(closure_result.contains(&edge));
        assert!(closure_result.contains(&face));
    }

    #[test]
    fn test_support() {
        let mut sieve = Sieve::new();
        let vertex = MeshEntity::Vertex(1);
        let edge = MeshEntity::Edge(1);

        sieve.add_arrow(vertex, edge);
        let support_result = sieve.support(&edge);

        assert!(support_result.contains(&vertex));
    }

    #[test]
    fn test_star() {
        let mut sieve = Sieve::new();
        let vertex = MeshEntity::Vertex(1);
        let edge = MeshEntity::Edge(1);
        let face = MeshEntity::Face(1);

        sieve.add_arrow(vertex, edge);
        sieve.add_arrow(edge, face);

        let star_result = sieve.star(&face);

        assert!(star_result.contains(&edge));
        assert!(star_result.contains(&vertex));
    }

    #[test]
    fn test_meet() {
        let mut sieve = Sieve::new();
        let vertex1 = MeshEntity::Vertex(1);
        let vertex2 = MeshEntity::Vertex(2);
        let edge = MeshEntity::Edge(1);
        let face = MeshEntity::Face(1);

        sieve.add_arrow(vertex1, edge);
        sieve.add_arrow(vertex2, edge);
        sieve.add_arrow(edge, face);

        let meet_result = sieve.meet(&vertex1, &vertex2);

        assert!(meet_result.contains(&edge));
    }

    #[test]
    fn test_join() {
        let mut sieve = Sieve::new();
        let vertex1 = MeshEntity::Vertex(1);
        let vertex2 = MeshEntity::Vertex(2);
        let edge = MeshEntity::Edge(1);
        let face = MeshEntity::Face(1);

        sieve.add_arrow(vertex1, edge);
        sieve.add_arrow(vertex2, edge);
        sieve.add_arrow(edge, face);

        let join_result = sieve.join(&vertex1, &vertex2);

        assert!(join_result.contains(&vertex1), "Join result should contain vertex1");
        assert!(join_result.contains(&vertex2), "Join result should contain vertex2");
        assert!(join_result.contains(&edge), "Join result should contain the edge");
        assert!(join_result.contains(&face), "Join result should contain the face");
    }
}