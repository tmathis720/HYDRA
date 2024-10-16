use crate::geometry::{Geometry, FaceShape};

impl Geometry {
    /// Computes the centroid of a pyramid cell (triangular or square base).
    ///
    /// The centroid is computed for a pyramid with either a triangular or square base.
    /// The pyramid is represented by 4 vertices (triangular base) or 5 vertices (square base).
    ///
    /// - For a triangular base, the function treats the pyramid as a tetrahedron.
    /// - For a square base, the function splits the pyramid into two tetrahedrons and combines their centroids.
    ///
    /// # Arguments
    /// * `cell_vertices` - A vector of 4 vertices for a triangular-based pyramid, or 5 vertices for a square-based pyramid.
    ///
    /// # Returns
    /// * `[f64; 3]` - The 3D coordinates of the pyramid centroid.
    ///
    /// # Panics
    /// This function will panic if the number of vertices is not 4 (for a triangular base) or 5 (for a square base).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let geometry = Geometry::new();
    /// let pyramid_vertices = vec![
    ///     [0.0, 0.0, 0.0], // base vertex 1
    ///     [1.0, 0.0, 0.0], // base vertex 2
    ///     [1.0, 1.0, 0.0], // base vertex 3
    ///     [0.0, 1.0, 0.0], // base vertex 4
    ///     [0.5, 0.5, 1.0], // apex
    /// ];
    /// let centroid = geometry.compute_pyramid_centroid(&pyramid_vertices);
    /// assert_eq!(centroid, [0.5, 0.5, 0.25]);
    /// ```
    pub fn compute_pyramid_centroid(&self, cell_vertices: &Vec<[f64; 3]>) -> [f64; 3] {
        let mut _total_volume = 0.0;
        let mut weighted_centroid = [0.0, 0.0, 0.0];

        let num_vertices = cell_vertices.len();
        assert!(num_vertices == 4 || num_vertices == 5, "Pyramid must have 4 (triangular) or 5 (square) vertices");

        // Define the apex and base vertices
        let apex = if num_vertices == 4 { cell_vertices[3] } else { cell_vertices[4] };
        let base_vertices = if num_vertices == 4 {
            vec![cell_vertices[0], cell_vertices[1], cell_vertices[2]] // Triangular base
        } else {
            vec![cell_vertices[0], cell_vertices[1], cell_vertices[2], cell_vertices[3]] // Square base
        };

        // For a square-based pyramid, split into two tetrahedrons
        if num_vertices == 5 {
            let tetra1 = vec![cell_vertices[0], cell_vertices[1], cell_vertices[2], apex];
            let tetra2 = vec![cell_vertices[0], cell_vertices[2], cell_vertices[3], apex];

            // Compute volumes and centroids
            let volume1 = self.compute_tetrahedron_volume(&tetra1);
            let volume2 = self.compute_tetrahedron_volume(&tetra2);

            let centroid1 = self.compute_tetrahedron_centroid(&tetra1);
            let centroid2 = self.compute_tetrahedron_centroid(&tetra2);

            _total_volume = volume1 + volume2;

            // Compute the weighted centroid
            weighted_centroid[0] = (centroid1[0] * volume1 + centroid2[0] * volume2) / _total_volume;
            weighted_centroid[1] = (centroid1[1] * volume1 + centroid2[1] * volume2) / _total_volume;
            weighted_centroid[2] = (centroid1[2] * volume1 + centroid2[2] * volume2) / _total_volume;
        } else {
            // For triangular base pyramid (tetrahedron)
            let tetra = vec![cell_vertices[0], cell_vertices[1], cell_vertices[2], apex];
            let volume = self.compute_tetrahedron_volume(&tetra);
            let centroid = self.compute_tetrahedron_centroid(&tetra);

            _total_volume = volume;
            weighted_centroid = centroid;
        }

        // Adjust the centroid between the base centroid and the apex
        let base_centroid = self.compute_face_centroid(
            if num_vertices == 4 { FaceShape::Triangle } else { FaceShape::Quadrilateral },
            &base_vertices,
        );

        // Apply pyramid centroid formula: 3/4 base_centroid + 1/4 apex
        weighted_centroid[0] = (3.0 * base_centroid[0] + apex[0]) / 4.0;
        weighted_centroid[1] = (3.0 * base_centroid[1] + apex[1]) / 4.0;
        weighted_centroid[2] = (3.0 * base_centroid[2] + apex[2]) / 4.0;

        weighted_centroid
    }

    /// Computes the volume of a pyramid cell (triangular or square base).
    ///
    /// The pyramid is represented by either 4 (triangular base) or 5 (square base) vertices.
    ///
    /// # Arguments
    /// * `cell_vertices` - A vector of 4 or 5 vertices representing the 3D coordinates of the pyramid's vertices.
    ///
    /// # Returns
    /// * `f64` - The volume of the pyramid.
    ///
    /// # Panics
    /// This function will panic if the number of vertices is not 4 (triangular) or 5 (square).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let geometry = Geometry::new();
    /// let pyramid_vertices = vec![
    ///     [0.0, 0.0, 0.0], // base vertex 1
    ///     [1.0, 0.0, 0.0], // base vertex 2
    ///     [1.0, 1.0, 0.0], // base vertex 3
    ///     [0.0, 1.0, 0.0], // base vertex 4
    ///     [0.5, 0.5, 1.0], // apex
    /// ];
    /// let volume = geometry.compute_pyramid_volume(&pyramid_vertices);
    /// assert!((volume - (1.0 / 3.0)).abs() < 1e-10); // Volume of a square pyramid
    /// ```
    pub fn compute_pyramid_volume(&self, cell_vertices: &Vec<[f64; 3]>) -> f64 {
        let mut _total_volume = 0.0;

        let num_vertices = cell_vertices.len();
        assert!(num_vertices == 4 || num_vertices == 5, "Pyramid must have 4 (triangular) or 5 (square) vertices");

        // Define the apex and base vertices
        let apex = if num_vertices == 4 { cell_vertices[3] } else { cell_vertices[4] };
        let _base_vertices = if num_vertices == 4 {
            vec![cell_vertices[0], cell_vertices[1], cell_vertices[2]] // Triangular base
        } else {
            vec![cell_vertices[0], cell_vertices[1], cell_vertices[2], cell_vertices[3]] // Square base
        };

        // Split into two tetrahedrons if square base
        if num_vertices == 5 {
            let tetra1 = vec![cell_vertices[0], cell_vertices[1], cell_vertices[2], apex];
            let tetra2 = vec![cell_vertices[0], cell_vertices[2], cell_vertices[3], apex];

            let volume1 = self.compute_tetrahedron_volume_local(&tetra1);
            let volume2 = self.compute_tetrahedron_volume_local(&tetra2);

            _total_volume = volume1 + volume2;
        } else {
            let tetra = vec![cell_vertices[0], cell_vertices[1], cell_vertices[2], apex];
            _total_volume = self.compute_tetrahedron_volume_local(&tetra);
        }

        _total_volume
    }

    /// Computes the volume of a tetrahedron using its four vertices.
    ///
    /// # Arguments
    /// * `vertices` - A vector of 4 vertices representing the tetrahedron.
    ///
    /// # Returns
    /// * `f64` - The volume of the tetrahedron.
    fn compute_tetrahedron_volume_local(&self, vertices: &Vec<[f64; 3]>) -> f64 {
        assert!(vertices.len() == 4, "Tetrahedron must have 4 vertices");

        let a = vertices[0];
        let b = vertices[1];
        let c = vertices[2];
        let d = vertices[3];

        let v1 = [b[0] - d[0], b[1] - d[1], b[2] - d[2]];
        let v2 = [c[0] - d[0], c[1] - d[1], c[2] - d[2]];
        let v3 = [a[0] - d[0], a[1] - d[1], a[2] - d[2]];

        let cross_product = [
            v2[1] * v3[2] - v2[2] * v3[1],
            v2[2] * v3[0] - v2[0] * v3[2],
            v2[0] * v3[1] - v2[1] * v3[0],
        ];

        let dot_product = v1[0] * cross_product[0] + v1[1] * cross_product[1] + v1[2] * cross_product[2];
        (dot_product.abs() / 6.0).abs()
    }
}


#[cfg(test)]
mod tests {
    use crate::geometry::Geometry;

    #[test]
    fn test_pyramid_volume_square() {
        let geometry = Geometry::new();

        // Define a square pyramid in 3D space
        let pyramid_vertices = vec![
            [0.0, 0.0, 0.0], // base vertex 1
            [1.0, 0.0, 0.0], // base vertex 2
            [1.0, 1.0, 0.0], // base vertex 3
            [0.0, 1.0, 0.0], // base vertex 4
            [0.5, 0.5, 1.0], // apex
        ];

        let volume = geometry.compute_pyramid_volume(&pyramid_vertices);

        // The expected volume of the square pyramid is (1/3) * base area * height
        // Base area = 1.0, height = 1.0 -> Volume = (1/3) * 1.0 * 1.0 = 1/3
        assert!((volume - (1.0 / 3.0)).abs() < 1e-10, "Volume of the square pyramid is incorrect");
    }

    #[test]
    fn test_pyramid_volume_triangular() {
        let geometry = Geometry::new();

        // Define a triangular pyramid (tetrahedron) in 3D space
        let pyramid_vertices = vec![
            [0.0, 0.0, 0.0], // base vertex 1
            [1.0, 0.0, 0.0], // base vertex 2
            [0.0, 1.0, 0.0], // base vertex 3
            [0.5, 0.5, 1.0], // apex
        ];

        let volume = geometry.compute_pyramid_volume(&pyramid_vertices);

        // The expected volume of the triangular pyramid (tetrahedron) is (1/3) * base area * height
        // Base area = 0.5, height = 1.0 -> Volume = (1/3) * 0.5 * 1.0 = 1/6
        assert!((volume - (1.0 / 6.0)).abs() < 1e-10, "Volume of the triangular pyramid is incorrect");
    }

    #[test]
    fn test_pyramid_centroid_square() {
        let geometry = Geometry::new();

        // Define a square pyramid in 3D space
        let pyramid_vertices = vec![
            [0.0, 0.0, 0.0], // base vertex 1
            [1.0, 0.0, 0.0], // base vertex 2
            [1.0, 1.0, 0.0], // base vertex 3
            [0.0, 1.0, 0.0], // base vertex 4
            [0.5, 0.5, 1.0], // apex
        ];

        let centroid = geometry.compute_pyramid_centroid(&pyramid_vertices);

        // The correct centroid is at (0.5, 0.5, 0.25)
        assert_eq!(centroid, [0.5, 0.5, 0.25]);
    }

    #[test]
    fn test_pyramid_centroid_triangular() {
        let geometry = Geometry::new();

        // Define a triangular pyramid in 3D space
        let pyramid_vertices = vec![
            [0.0, 0.0, 0.0], // base vertex 1
            [1.0, 0.0, 0.0], // base vertex 2
            [0.0, 1.0, 0.0], // base vertex 3
            [0.5, 0.5, 1.0], // apex
        ];

        let centroid = geometry.compute_pyramid_centroid(&pyramid_vertices);

        // The correct centroid is at (0.375, 0.375, 0.25)
        assert_eq!(centroid, [0.375, 0.375, 0.25]);
    }

    #[test]
    fn test_pyramid_volume_degenerate_case() {
        let geometry = Geometry::new();

        // Define a degenerate pyramid where all points are coplanar
        let degenerate_pyramid_vertices = vec![
            [0.0, 0.0, 0.0], // base vertex 1
            [1.0, 0.0, 0.0], // base vertex 2
            [0.0, 1.0, 0.0], // base vertex 3
            [0.5, 0.5, 0.0], // apex (degenerate, on the same plane as the base)
        ];

        let volume = geometry.compute_pyramid_volume(&degenerate_pyramid_vertices);

        // The volume of a degenerate pyramid should be zero
        assert_eq!(volume, 0.0);
    }

    #[test]
    fn test_pyramid_centroid_degenerate_case() {
        let geometry = Geometry::new();

        // Define a degenerate pyramid where all points are coplanar
        let degenerate_pyramid_vertices = vec![
            [0.0, 0.0, 0.0], // base vertex 1
            [1.0, 0.0, 0.0], // base vertex 2
            [0.0, 1.0, 0.0], // base vertex 3
            [0.5, 0.5, 0.0], // apex (degenerate, on the same plane as the base)
        ];

        let centroid = geometry.compute_pyramid_centroid(&degenerate_pyramid_vertices);

        // The centroid of this degenerate pyramid should still be computed correctly
        let expected_centroid = [0.375, 0.375, 0.0];
        assert_eq!(centroid, expected_centroid);
    }
}
