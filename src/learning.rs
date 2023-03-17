use rurel::mdp::Agent;
use rurel::mdp::State;
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::FixedIterations;
use rurel::AgentTrainer;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MyState {
    pub x: i32, // rows
    pub y: i32, // cols
    pub goal: (i32, i32),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MyAction {
    dx: i32,
    dy: i32,
}

impl State for MyState {
    type A = MyAction;
    fn reward(&self) -> f64 {
        // negative Euclidean distance
        -((((self.goal.0 - self.x).pow(2) + (self.goal.1 - self.y).pow(2)) as f64).sqrt())
    }
    fn actions(&self) -> Vec<MyAction> {
        vec![
            MyAction { dx: 0, dy: -1 }, // up
            MyAction { dx: 0, dy: 1 },  // down
            MyAction { dx: -1, dy: 0 }, // left
            MyAction { dx: 1, dy: 0 },  // right
        ]
    }
}

pub struct MyAgent {
    state: MyState,
}

impl Agent<MyState> for MyAgent {
    fn current_state(&self) -> &MyState {
        &self.state
    }
    fn take_action(&mut self, action: &MyAction) -> () {
        match action {
            &MyAction { dx, dy } => {
                self.state = MyState {
                    x: (((self.state.x + dx) % 11) + 11) % 11, // (x+dx) mod 11
                    y: (((self.state.y + dy) % 11) + 11) % 11, // (y+dy) mod 11
                    goal: self.state.goal,
                }
            }
        }
    }
}

pub fn train(trainer: &mut AgentTrainer<MyState>) {
    println!("TRAINING");
    let mut agent = MyAgent {
        state: MyState {
            x: 0,
            y: 0,
            goal: (5, 5),
        },
    };
    trainer.train(
        &mut agent,
        &QLearning::new(0.2, 0.01, 2.),
        &mut FixedIterations::new(100_000),
        &RandomExploration::new(),
    );
    println!("STOP TRAINING");
}

pub fn query(trainer: &AgentTrainer<MyState>, current_state: &MyState) -> Option<MyAction> {
    trainer.best_action(current_state)
}
