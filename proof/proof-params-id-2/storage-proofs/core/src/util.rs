pub const NODE_SIZE: usize = 32;

/// Returns the start position of the data, 0-indexed.
pub fn data_at_node_offset(v: usize) -> usize {
    v * NODE_SIZE
}