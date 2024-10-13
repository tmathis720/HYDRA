#[cfg(test)]
mod tests {
    use crate::input_output::gmsh_parser::GmshParser;
    use crate::domain::MeshEntity;
    use std::path::Path;

    fn assert_mesh_validity(mesh: &crate::domain::mesh::Mesh, expected_nodes: usize, expected_elements: usize, mesh_name: &str) {
        let node_count = mesh.count_entities(&MeshEntity::Vertex(0));
        let element_count = mesh.count_entities(&MeshEntity::Cell(0));

        assert!(node_count > 0, "{}: Node count should not be empty", mesh_name);
        assert!(element_count > 0, "{}: Element count should not be empty", mesh_name);
        assert_eq!(node_count, expected_nodes, "{}: Incorrect number of nodes", mesh_name);
        assert_eq!(element_count, expected_elements, "{}: Incorrect number of elements", mesh_name);
    }

    #[test]
    fn test_circle_mesh_import() {
        let temp_file_path = Path::new("inputs/circular_lake.msh2");

        let result = GmshParser::from_gmsh_file(temp_file_path.to_str().unwrap());
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_mesh_validity(&mesh, 424, 849, "Circle Mesh");
    }

    #[test]
    fn test_coastal_island_mesh_import() {
        let temp_file_path = Path::new("inputs/coastal_island.msh2");

        let result = GmshParser::from_gmsh_file(temp_file_path.to_str().unwrap());
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_mesh_validity(&mesh, 1075, 2154, "Coastal Island Mesh");
    }

    #[test]
    fn test_lagoon_mesh_import() {
        let temp_file_path = Path::new("inputs/elliptical_lagoon.msh2");

        let result = GmshParser::from_gmsh_file(temp_file_path.to_str().unwrap());
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_mesh_validity(&mesh, 848, 1697, "Lagoon Mesh");
    }

    #[test]
    fn test_meandering_river_mesh_import() {
        let temp_file_path = Path::new("inputs/meandering_river.msh2");

        let result = GmshParser::from_gmsh_file(temp_file_path.to_str().unwrap());
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_mesh_validity(&mesh, 695, 1386, "Meandering River Mesh");
    }

    #[test]
    fn test_polygon_estuary_mesh_import() {
        let temp_file_path = Path::new("inputs/polygon_estuary.msh2");

        let result = GmshParser::from_gmsh_file(temp_file_path.to_str().unwrap());
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_mesh_validity(&mesh, 469, 941, "Polygon Estuary Mesh");
    }

    #[test]
    fn test_rectangle_mesh_import() {
        let temp_file_path = Path::new("inputs/rectangle.msh2");

        let result = GmshParser::from_gmsh_file(temp_file_path.to_str().unwrap());
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_mesh_validity(&mesh, 78, 158, "Rectangle Mesh");
    }

    #[test]
    fn test_rectangle_channel_mesh_import() {
        let temp_file_path = Path::new("inputs/rectangular_channel.msh2");

        let result = GmshParser::from_gmsh_file(temp_file_path.to_str().unwrap());
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_mesh_validity(&mesh, 149, 300, "Rectangular Channel Mesh");
    }

    #[test]
    fn test_triangle_basin_mesh_import() {
        let temp_file_path = Path::new("inputs/triangular_basin.msh2");

        let result = GmshParser::from_gmsh_file(temp_file_path.to_str().unwrap());
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_mesh_validity(&mesh, 66, 133, "Triangle Basin Mesh");
    }
}

    /* #[test]
    fn test_generate_rectangle_2d() {
        let width = 10.0;
        let height = 5.0;
        let nx = 4; // Number of cells along x-axis
        let ny = 2; // Number of cells along y-axis

        let mesh = MeshGenerator::generate_rectangle_2d(width, height, nx, ny);

        // The number of vertices should be (nx + 1) * (ny + 1)
        let expected_num_vertices = (nx + 1) * (ny + 1);
        let num_vertices = mesh
            .entities
            .iter()
            .filter(|e| matches!(e, MeshEntity::Vertex(_)))
            .count();
        assert_eq!(num_vertices, expected_num_vertices, "Incorrect number of vertices");

        // The number of quadrilateral cells should be nx * ny
        let expected_num_cells = nx * ny;
        let num_cells = mesh
            .entities
            .iter()
            .filter(|e| matches!(e, MeshEntity::Cell(_)))
            .count();
        assert_eq!(num_cells, expected_num_cells, "Incorrect number of cells");
    }

    #[test]
    fn test_generate_rectangle_3d() {
        let width = 10.0;
        let height = 5.0;
        let depth = 3.0;
        let nx = 4; // Number of cells along x-axis
        let ny = 2; // Number of cells along y-axis
        let nz = 1; // Number of cells along z-axis

        let mesh = MeshGenerator::generate_rectangle_3d(width, height, depth, nx, ny, nz);

        // The number of vertices should be (nx + 1) * (ny + 1) * (nz + 1)
        let expected_num_vertices = (nx + 1) * (ny + 1) * (nz + 1);
        let num_vertices = mesh
            .entities
            .iter()
            .filter(|e| matches!(e, MeshEntity::Vertex(_)))
            .count();
        assert_eq!(num_vertices, expected_num_vertices, "Incorrect number of vertices");

        // The number of hexahedral cells should be nx * ny * nz
        let expected_num_cells = nx * ny * nz;
        let num_cells = mesh
            .entities
            .iter()
            .filter(|e| matches!(e, MeshEntity::Cell(_)))
            .count();
        assert_eq!(num_cells, expected_num_cells, "Incorrect number of cells");
    }

    #[test]
    fn test_generate_circle() {
        let radius = 5.0;
        let num_divisions = 8; // Number of divisions around the circle

        let mesh = MeshGenerator::generate_circle(radius, num_divisions);

        // The number of vertices should be num_divisions + 1 (center vertex + boundary vertices)
        let expected_num_vertices = num_divisions + 1;
        let num_vertices = mesh
            .entities
            .iter()
            .filter(|e| matches!(e, MeshEntity::Vertex(_)))
            .count();
        assert_eq!(num_vertices, expected_num_vertices, "Incorrect number of vertices");

        // The number of triangular cells should be equal to num_divisions
        let expected_num_cells = num_divisions;
        let num_cells = mesh
            .entities
            .iter()
            .filter(|e| matches!(e, MeshEntity::Cell(_)))
            .count();
        assert_eq!(num_cells, expected_num_cells, "Incorrect number of cells");
    } */
