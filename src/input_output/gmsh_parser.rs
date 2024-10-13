use pest::Parser;
use pest_derive::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::domain::{mesh::Mesh, MeshEntity, Sieve};
use crate::geometry::Geometry;

#[derive(Parser)]
#[grammar = "input_output/pest/gmsh.pest"] // Refers to the GMSH grammar definition in gmsh.pest
pub struct GmshParser;

impl GmshParser {
    /// Parse a GMSH file and return a `Mesh` object containing the parsed nodes and elements.
    pub fn from_gmsh_file(file_path: &str) -> Result<Mesh, io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let file_content: String = reader.lines().collect::<Result<_, _>>()?;

        // Normalize the line endings to Unix-style (just \n)
        let normalized_content = file_content.replace("\r\n", "\n").replace("\r", "\n");

        // Attempt to parse the normalized file content
        let parsed_file = GmshParser::parse(Rule::file, &normalized_content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Parse error: {}", e)))?;

        let mut mesh = Mesh::new();
        let mut sieve = Sieve::new();
        let mut geometry = Geometry::new();

        // Parse through the sections
        for section in parsed_file {
            match section.as_rule() {
                Rule::mesh_format_section => {
                    println!("Parsing mesh format section...");
                    // Handle $MeshFormat, this section is metadata, so we just skip it
                },
                Rule::nodes_section => {
                    println!("Parsing nodes section...");
                    Self::parse_nodes(section, &mut mesh, &mut geometry);
                },
                Rule::elements_section => {
                    println!("Parsing elements section...");
                    Self::parse_elements(section, &mut mesh, &mut sieve);
                },
                _ => {
                    println!("Encountered an unhandled section.");
                    // Ignore other sections or handle them as needed (e.g., periodic data, physical names, etc.)
                }
            }
        }

        Ok(mesh)
    }

    /// Parse the node section of a GMSH file and update the mesh with node data.
    fn parse_nodes(section: pest::iterators::Pair<Rule>, mesh: &mut Mesh, geometry: &mut Geometry) {
        let mut section_pairs = section.into_inner();
    
        // The first element is the node count, ensure it's present
        if let Some(node_count) = section_pairs.next() {
            println!("Node count: {}", node_count.as_str());
        } else {
            println!("Error: Missing node count in node section.");
            return;
        }
    
        // Process each node
        for node_line in section_pairs {
            let mut node_parts = node_line.into_inner();
    
            // Defensive unwrapping with detailed error handling
            let node_id = match node_parts.next() {
                Some(id) => {
                    println!("Parsing node_id: {}", id.as_str());
                    id.as_str().parse::<usize>().expect("Invalid node ID")
                },
                None => {
                    println!("Error: Missing node ID.");
                    continue;
                }
            };
    
            let x = match node_parts.next() {
                Some(val) => {
                    println!("Parsing x: {}", val.as_str());
                    val.as_str().parse::<f64>().expect("Invalid x-coordinate")
                },
                None => {
                    println!("Error: Missing x-coordinate for node {}", node_id);
                    continue;
                }
            };
    
            let y = match node_parts.next() {
                Some(val) => {
                    println!("Parsing y: {}", val.as_str());
                    val.as_str().parse::<f64>().expect("Invalid y-coordinate")
                },
                None => {
                    println!("Error: Missing y-coordinate for node {}", node_id);
                    continue;
                }
            };
    
            let z = match node_parts.next() {
                Some(val) => {
                    println!("Parsing z: {}", val.as_str());
                    val.as_str().parse::<f64>().expect("Invalid z-coordinate")
                },
                None => {
                    println!("Error: Missing z-coordinate for node {}", node_id);
                    continue;
                }
            };
    
            println!("Adding node {}: ({}, {}, {})", node_id, x, y, z);
            mesh.set_vertex_coordinates(node_id, [x, y, z]);
            geometry.set_vertex(node_id, [x, y, z]);
        }
    }

    /// Parse the elements section of a GMSH file and update the mesh with element data.
    fn parse_elements(section: pest::iterators::Pair<Rule>, mesh: &mut Mesh, sieve: &mut Sieve) {
        let mut section_pairs = section.into_inner();

        // The first element is the element count, ensure it's present
        if let Some(element_count) = section_pairs.next() {
            println!("Element count: {}", element_count.as_str());
        } else {
            println!("Error: Missing element count in element section.");
            return;
        }

        // Process each element
        for element_line in section_pairs {
            let mut element_parts = element_line.into_inner();

            let element_id = match element_parts.next() {
                Some(id) => id.as_str().parse::<usize>().expect("Invalid element ID"),
                None => {
                    println!("Error: Missing element ID.");
                    continue;
                }
            };

            let element_type = match element_parts.next() {
                Some(t) => t.as_str().parse::<u32>().expect("Invalid element type"),
                None => {
                    println!("Error: Missing element type for element {}", element_id);
                    continue;
                }
            };

            let tag_count = match element_parts.next() {
                Some(count) => count.as_str().parse::<usize>().expect("Invalid tag count"),
                None => {
                    println!("Error: Missing tag count for element {}", element_id);
                    continue;
                }
            };

            // Skip tag values
            for _ in 0..tag_count {
                element_parts.next(); // Skipping tags
            }

            // Collect the node references
            let node_ids: Vec<usize> = element_parts.map(|n| n.as_str().parse::<usize>().expect("Invalid node ID")).collect();
            println!("Element {} with nodes {:?}", element_id, node_ids);

            // Add entity and relationships to the mesh based on element type
            GmshParser::add_mesh_entity(mesh, sieve, element_type, element_id, node_ids);
        }
    }

    /// Create mesh entities for each element type (triangle, tetrahedron, etc.) and add them to the mesh.
    fn add_mesh_entity(mesh: &mut Mesh, _sieve: &mut Sieve, element_type: u32, element_id: usize, node_ids: Vec<usize>) {
        match element_type {
            2 => { // Triangle
                println!("Adding triangle element with ID {}", element_id);
                let triangle = MeshEntity::Cell(element_id);
                mesh.add_entity(triangle);
                for &node_id in &node_ids {
                    let vertex = MeshEntity::Vertex(node_id);
                    mesh.add_relationship(triangle, vertex);
                }
            },
            4 => { // Tetrahedron
                println!("Adding tetrahedron element with ID {}", element_id);
                let tetrahedron = MeshEntity::Cell(element_id);
                mesh.add_entity(tetrahedron);
                for &node_id in &node_ids {
                    let vertex = MeshEntity::Vertex(node_id);
                    mesh.add_relationship(tetrahedron, vertex);
                }
            },
            _ => {
                println!("Unhandled element type: {}", element_type);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::input_output::gmsh_parser::{GmshParser, Rule};
    use crate::domain::{mesh::Mesh, Sieve};
    use crate::geometry::Geometry;

    #[test]
    fn test_parse_node() {
        let node_str = "$MeshFormat\n2.2 0 8\n$EndMeshFormat\n$Nodes\n1\n1 0.0 1.0 2.0\n$EndNodes\n";
        let mut mesh = Mesh::new();
        let mut geometry = Geometry::new();

        // Parse the input string using the GMSH parser
        let parser_result = GmshParser::parse(Rule::file, node_str);
        
        match parser_result {
            Ok(parsed_file) => {
                // Look for the nodes_section rule
                if let Some(first_pair) = parsed_file.into_iter().find(|p| p.as_rule() == Rule::nodes_section) {
                    println!("Nodes section found, proceeding with node parsing.");
                    GmshParser::parse_nodes(first_pair, &mut mesh, &mut geometry);
                    
                    // Verify the parsed node data
                    assert_eq!(mesh.get_vertex_coordinates(1), Some([0.0, 1.0, 2.0]));
                } else {
                    panic!("Error: nodes_section not found in parsed file.");
                }
            },
            Err(e) => {
                panic!("Parsing error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_parse_element() {
        let element_str = "$MeshFormat\n2.2 0 8\n$EndMeshFormat\n$Elements\n1\n1 2 0 1 1 2 3\n$EndElements\n";
        let mut mesh = Mesh::new();
        let mut sieve = Sieve::new();

        let parser = GmshParser::parse(Rule::file, element_str).unwrap();
        let first_pair = parser.into_iter().find(|p| p.as_rule() == Rule::elements_section).unwrap();

        GmshParser::parse_elements(first_pair, &mut mesh, &mut sieve);

        let cell_entities = mesh.get_cells();
        assert_eq!(cell_entities.len(), 1);
    }
}