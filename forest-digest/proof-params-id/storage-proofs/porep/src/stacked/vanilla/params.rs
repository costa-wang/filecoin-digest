use super::{ LayerChallenges };

#[derive(Debug, Clone)]
pub struct SetupParams {
    // Number of nodes
    pub nodes: usize,

    // Base degree of DRG
    pub degree: usize,

    pub expansion_degree: usize,

    pub porep_id: [u8; 32],
    pub layer_challenges: LayerChallenges,
}
