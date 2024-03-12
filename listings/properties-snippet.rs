pub fn find_unsafe_sf_ids(&self) -> Vec<&String> {
    self.snapshots.iter()
        .flat_map(|snapshot| snapshot.tokens.iter())
        .filter_map(|(sf_id, amount)| {
            if *amount >= 2 {
                Some(sf_id)
            } else {
                None
            }
        })
        .collect()
}