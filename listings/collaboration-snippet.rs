while !unexplored_states.is_empty() {
    let current_state = unexplored_states.pop();
    // Explore the state
    let potentially_unexplored_states =
        self.explore_state(&current_state);

    let mut transitions = vec![];
    for new_state in potentially_unexplored_states {
        // Check if we know the state already
        match seen_states.get(new_state) {
            None => {
                // State is new.
                seen_states.insert(new_state, true);
                unexplored_states.push(new_state);
            }
            Some(_) => {}
        }
        // Add transitions.
        transitions.push((flow_node_id, new_state));
    }
    // Check properties
    check_on_the_fly_properties(
        &current_state,
        &properties,
        &mut property_results,
        &transitions,
    );
    // Save the state and its transitions.
    state_space.states.insert(current_state);
    state_space
        .transitions
        .insert(current_state, transitions);
}