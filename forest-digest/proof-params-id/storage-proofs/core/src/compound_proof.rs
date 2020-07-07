#[derive(Clone)]
pub struct PublicParams<'a, S: ProofScheme<'a>> {
    pub vanilla_params: S::PublicParams,
    pub partitions: Option<usize>,
    pub priority: bool,
}