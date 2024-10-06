use std::borrow::BorrowMut;

use rand::Rng;

use super::mcts_game_state::MCTSGameState;

const EXPLORATION_CONSTANT: f64 = 1.414;  // sqrt(2) for UCT

struct MCTSNode {
    state: MCTSGameState,
    visits: f64,
    wins: f64,
    children: Vec<MCTSNode>,
}

impl MCTSNode {
    fn new(state: MCTSGameState) -> MCTSNode {
        MCTSNode {
            state,
            visits: 0.0,
            wins: 0.0,
            children: vec![],
        }
    }

    // Monte Carlo Tree Search function
    fn mcts(&mut self, iterations: usize) {
        for _ in 0..iterations {
            let node = self.select();
            let result = node.simulate();
            node.backpropagate(result);
        }
    }

    // Selection: Traverse down the tree using UCB1 or random selection
    fn select(&mut self) -> &mut MCTSNode {
        let mut node = self;
        while !node.children.is_empty() && !node.state.is_terminal() {
            node = node.best_child_mut().borrow_mut(); // Pick the best child node
        }
        if !node.state.is_terminal() && node.children.is_empty() {
            node.expand();
        }
        node
    }

    // Expansion: Add child nodes for possible actions
    fn expand(&mut self) {
        let actions = self.state.available_actions();
        for action in actions {
            let new_state = self.state.apply_action(&action);
            self.children.push(MCTSNode::new(new_state));
        }
    }

    // Simulation: Simulate a random playout from this node
    fn simulate(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut state = self.state.clone();
        
        while !state.is_terminal() {
            let actions = state.available_actions();
            let action = &actions[rng.gen_range(0..actions.len())];
            state = state.apply_action(action);
        }
        state.calculate_score() // Return final score
    }

    // Backpropagation: Update node statistics
    fn backpropagate(&mut self, result: i32) {
        let mut node = self;
        while !node.state.is_terminal() {
            node.visits += 1.0;
            node.wins += result as f64;
            if node.children.is_empty() {
                break;
            }
            // Use mutable reference to allow node traversal
            node = node.best_child_mut();
        }
    }

    // UCB1 to find the best child (mutable for traversal in backpropagate)
    fn best_child_mut(&mut self) -> &mut MCTSNode {
        self.children
            .iter_mut()
            .max_by(|a, b| {
                let a_score = a.wins / a.visits + (2.0 * (self.visits.ln() / a.visits).sqrt());
                let b_score = b.wins / b.visits + (2.0 * (self.visits.ln() / b.visits).sqrt());
                a_score.partial_cmp(&b_score).unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("No children found")
        }
}
