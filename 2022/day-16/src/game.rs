use crate::parse::Graph;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum Action {
    Move {
        delta_flow: i32,
        time: i32,
        from_node_index: usize,
        node_index: usize,
        label: String,
    },
    SpinClock {
        time: i32,
    },
    EnterElephant {
        from_node: usize, // where the player is
        to_node: usize,   // where the elephant spawns
    },
}

#[derive(Debug)]
pub struct Game {
    pub graph: Graph,
    pub nodes: HashSet<usize>,
    pub current_node: usize,
    pub start_node: usize,
    pub flow_rate: i32,
    pub steam_released: i32,
    pub time_left: i32,
    pub actions: Vec<Action>,
    pub should_use_elephant: bool,
    pub am_elephanting: bool, // is the elphant currently elephanting?
}

impl Game {
    pub fn new(
        graph: Graph,
        time: i32,
        nodes: HashSet<usize>,
        start_node: usize,
        should_use_elephant: bool,
    ) -> Self {
        Game {
            graph,
            nodes,
            current_node: start_node,
            start_node: start_node,
            flow_rate: 0,
            steam_released: 0,
            time_left: time,
            actions: vec![],
            should_use_elephant,
            am_elephanting: false,
        }
    }

    pub fn borrow(&mut self) {}

    pub fn get_elephant_game(&mut self, time: i32) -> Game {
        Self {
            graph: self.graph.clone(),
            nodes: self.nodes.clone(),
            actions: vec![],
            am_elephanting: true,
            start_node: self.start_node,
            current_node: self.start_node,
            flow_rate: 0,
            time_left: time,
            should_use_elephant: true,
            steam_released: 0,
        }
    }

    pub fn goto_node(&mut self, node_index: usize) -> Option<i32> {
        let time = self.graph.distance(self.current_node, node_index) + 1;

        if time >= self.time_left {
            self.spin(self.time_left);
            Some(self.steam_released)
        } else {
            let delta_flow = self.graph.node_by_index(node_index).flow_rate;
            self.actions.push(Action::Move {
                time,
                delta_flow,
                from_node_index: self.current_node,
                node_index,
                label: self.graph.node_by_index(node_index).label.clone(),
            });
            self.travel(node_index);
            self.tick(time, delta_flow);
            None
        }
    }

    /// requires a rewind
    pub fn run_elephant(&mut self, start_node: usize, time: i32) {
        self.actions.push(Action::EnterElephant {
            from_node: self.current_node,
            to_node: start_node,
        });
        self.current_node = start_node;
        self.time_left = time;
    }

    fn spin(&mut self, time: i32) {
        self.actions.push(Action::SpinClock {
            time: self.time_left,
        });
        self.tick(self.time_left, 0);
    }

    fn travel(&mut self, node_index: usize) {
        self.current_node = node_index;
        self.nodes.remove(&node_index);
    }

    fn untravel(&mut self, node_index: usize) {
        self.nodes.insert(self.current_node);
        self.current_node = node_index;
    }

    pub fn tick(&mut self, time: i32, delta_flow: i32) {
        self.time_left -= time;
        self.steam_released += time * self.flow_rate;
        self.flow_rate += delta_flow;
    }

    pub fn untick(&mut self, time: i32, delta_flow: i32) {
        self.flow_rate -= delta_flow;
        self.steam_released -= time * self.flow_rate;
        self.time_left += time;
    }

    /// waste the rest of the time
    pub fn run_the_clock(&mut self) -> i32 {
        self.actions.push(Action::SpinClock {
            time: self.time_left,
        });
        self.tick(self.time_left, 0);
        self.steam_released
    }

    pub fn rewind(&mut self) {
        let action = self.actions.pop().unwrap();
        match action {
            Action::SpinClock { time } => {
                self.untick(time, 0);
            }
            Action::Move {
                delta_flow,
                time,
                from_node_index,
                node_index,
                label,
            } => {
                self.untick(time, delta_flow);
                self.untravel(from_node_index);
            }
            Action::EnterElephant { from_node, to_node } => {
                self.time_left = 0; // elephant only enters when time_left is 0
                self.current_node = from_node;
            }
        }
    }
}
