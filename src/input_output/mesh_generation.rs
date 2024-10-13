use crate::domain::{mesh::Mesh, MeshEntity};

pub struct MeshGenerator;

impl MeshGenerator {
    /// Generates a 2D rectangular mesh with a specified width, height, and resolution (nx, ny).
    pub fn generate_rectangle_2d(width: f64, height: f64, nx: usize, ny: usize) -> Mesh {
        let mut mesh = Mesh::new();

        // Generate vertices
        let nodes = Self::generate_grid_nodes_2d(width, height, nx, ny);
        for (id, position) in nodes.into_iter().enumerate() {
            mesh.set_vertex_coordinates(id, position);
            // Note: `set_vertex_coordinates` already adds the vertex entity to `mesh.entities`
        }

        // Generate quadrilateral cells and add them to the mesh
        Self::generate_quadrilateral_cells(&mut mesh, nx, ny);

        mesh
    }

    /// Generates a 3D rectangular mesh with a specified width, height, depth, and resolution (nx, ny, nz).
    pub fn generate_rectangle_3d(width: f64, height: f64, depth: f64, nx: usize, ny: usize, nz: usize) -> Mesh {
        let mut mesh = Mesh::new();
    
        // Generate vertices
        let nodes = Self::generate_grid_nodes_3d(width, height, depth, nx, ny, nz);
        for (id, position) in nodes.into_iter().enumerate() {
            mesh.set_vertex_coordinates(id, position);
        }
    
        // Generate hexahedral cells and add them to the mesh
        Self::generate_hexahedral_cells(&mut mesh, nx, ny, nz);
    
        mesh
    }

    /// Generates circle nodes for a circular mesh.
    pub fn generate_circle(radius: f64, num_divisions: usize) -> Mesh {
        let mut mesh = Mesh::new();
    
        // Generate vertices
        let nodes = Self::generate_circle_nodes(radius, num_divisions);
        for (id, position) in nodes.into_iter().enumerate() {
            mesh.set_vertex_coordinates(id, position);
        }
    
        // Generate triangular cells and add them to the mesh
        Self::generate_triangular_cells(&mut mesh, num_divisions);
    
        mesh
    }

    // --- Internal Helper Functions ---

    /// Generate 2D grid nodes for rectangular mesh
    fn generate_grid_nodes_2d(width: f64, height: f64, nx: usize, ny: usize) -> Vec<[f64; 3]> {
        let mut nodes = Vec::new();
        let dx = width / nx as f64;
        let dy = height / ny as f64;

        for j in 0..=ny {
            for i in 0..=nx {
                nodes.push([i as f64 * dx, j as f64 * dy, 0.0]);
            }
        }
        nodes
    }

    /// Generate 3D grid nodes for rectangular mesh
    fn generate_grid_nodes_3d(width: f64, height: f64, depth: f64, nx: usize, ny: usize, nz: usize) -> Vec<[f64; 3]> {
        let mut nodes = Vec::new();
        let dx = width / nx as f64;
        let dy = height / ny as f64;
        let dz = depth / nz as f64;

        for k in 0..=nz {
            for j in 0..=ny {
                for i in 0..=nx {
                    nodes.push([i as f64 * dx, j as f64 * dy, k as f64 * dz]);
                }
            }
        }
        nodes
    }

    /// Generate circle nodes for circular mesh
    fn generate_circle_nodes(radius: f64, num_divisions: usize) -> Vec<[f64; 3]> {
        let mut nodes = Vec::new();
        nodes.push([0.0, 0.0, 0.0]);

        for i in 0..num_divisions {
            let theta = 2.0 * std::f64::consts::PI * (i as f64) / (num_divisions as f64);
            nodes.push([radius * theta.cos(), radius * theta.sin(), 0.0]);
        }

        nodes
    }

    /// Generate quadrilateral cells for a 2D rectangular mesh
    fn generate_quadrilateral_cells(mesh: &mut Mesh, nx: usize, ny: usize) {
        let mut cell_id = 0; // Use a separate counter for cell IDs

        for j in 0..ny {
            for i in 0..nx {
                // Get vertex indices for the quadrilateral
                let n1 = j * (nx + 1) + i;
                let n2 = n1 + 1;
                let n3 = n1 + (nx + 1) + 1;
                let n4 = n1 + (nx + 1);

                // Create a cell entity with a unique cell ID
                let cell = MeshEntity::Cell(cell_id);
                cell_id += 1; // Increment cell ID for the next cell
                mesh.add_entity(cell.clone());

                // Add relationships between the cell and its vertices
                mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n1));
                mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n2));
                mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n3));
                mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n4));
            }
        }
    }

    /// Generate hexahedral cells for a 3D rectangular mesh
    fn generate_hexahedral_cells(mesh: &mut Mesh, nx: usize, ny: usize, nz: usize) {
        let mut cell_id = 0; // Use a separate counter for cell IDs

        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    // Get vertex indices for the hexahedral cell
                    let n1 = k * (ny + 1) * (nx + 1) + j * (nx + 1) + i;
                    let n2 = n1 + 1;
                    let n3 = n1 + (nx + 1);
                    let n4 = n3 + 1;
                    let n5 = n1 + (ny + 1) * (nx + 1);
                    let n6 = n5 + 1;
                    let n7 = n5 + (nx + 1);
                    let n8 = n7 + 1;

                    // Create a cell entity with a unique cell ID
                    let cell = MeshEntity::Cell(cell_id);
                    cell_id += 1; // Increment cell ID for the next cell
                    mesh.add_entity(cell.clone());

                    // Add relationships between the cell and its vertices
                    mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n1));
                    mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n2));
                    mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n3));
                    mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n4));
                    mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n5));
                    mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n6));
                    mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n7));
                    mesh.add_relationship(cell.clone(), MeshEntity::Vertex(n8));
                }
            }
        }
    }

    /// Generate triangular cells for a circular mesh
    fn generate_triangular_cells(mesh: &mut Mesh, num_divisions: usize) {
        let mut cell_id = 0; // Use a separate counter for cell IDs

        for i in 0..num_divisions {
            let next = (i + 1) % num_divisions;

            // Create a cell entity with a unique cell ID
            let cell = MeshEntity::Cell(cell_id);
            cell_id += 1; // Increment cell ID for the next cell
            mesh.add_entity(cell.clone());

            // Add relationships between the cell and its vertices
            mesh.add_relationship(cell.clone(), MeshEntity::Vertex(0)); // Central vertex
            mesh.add_relationship(cell.clone(), MeshEntity::Vertex(i + 1)); // Current vertex
            mesh.add_relationship(cell.clone(), MeshEntity::Vertex(next + 1)); // Next vertex
        }
    }

    // Generate faces for a 3D rectangular mesh
/*     fn _generate_faces_3d(mesh: &mut Mesh, nx: usize, ny: usize, nz: usize) {
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    // Get the vertices for each face
                    let n1 = k * (ny + 1) * (nx + 1) + j * (nx + 1) + i;
                    let n2 = n1 + 1;
                    let n3 = n1 + (nx + 1);
                    let n4 = n3 + 1;
                    let n5 = n1 + (ny + 1) * (nx + 1);
                    let _n6 = n5 + 1;
                    let n7 = n5 + (nx + 1);
                    let _n8 = n7 + 1;

                    // Define the faces and add them as MeshEntities
                    let front_face = MeshEntity::Face(mesh.entities.len());
                    mesh.add_entity(front_face.clone());
                    mesh.add_relationship(front_face.clone(), MeshEntity::Vertex(n1));
                    mesh.add_relationship(front_face.clone(), MeshEntity::Vertex(n2));
                    mesh.add_relationship(front_face.clone(), MeshEntity::Vertex(n4));
                    mesh.add_relationship(front_face.clone(), MeshEntity::Vertex(n3));

                    // Repeat for other faces (back, left, right, top, bottom)
                    // ...
                }
            }
        }
    } */
}
