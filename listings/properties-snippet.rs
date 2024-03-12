pub fn find_unsafe_sf_ids(&self) -> Vec<&String> {
    self.snapshots.iter()
        .flat_map(|snapshot| snapshot.tokens.iter())
        .filter(|&(_, token_amount)| *token_amount >= 2)
        .map(|(sf_id, _)| sf_id)
        .collect()
}