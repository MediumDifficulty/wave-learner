use std::convert::TryInto;

use rand::random;
use rand_distr::Distribution;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

pub const WAVE_RES: usize = 512;

const STARTING_FUNCTIONS: usize = 4;
const SELECTION_FRACTION: f32 = 0.2;
const MUTATION_PROBABILITY: f32 = 0.4;
const MUTATION_STRENGTH: f32 = 0.1;
const FUNCTION_ADDITION_PROBABILITY: f32 = 0.1;
const FUNCTION_SUBTRACTION_PROBABILITY: f32 = 0.1;
const MAX_FUNCTIONS: usize = 12;

#[derive(Debug)]
pub struct Trainer<const A: usize> {
    goal: [f32; WAVE_RES],
    agents: [Agent; A],
}

impl <const A: usize>Trainer<A> {
    pub fn new_rand(goal: &[f32]) -> Self {
        let goal = goal.try_into().unwrap();

        Self {
            goal,
            agents: (0..A).map(|_| {
                let mut agent = Agent::new_rand(STARTING_FUNCTIONS);
                agent.calculate_fitness(&goal);
                agent
            }).collect::<Vec<_>>().try_into().unwrap(),
        }
    }

    pub fn step(&mut self) {
        // Selection
        let top = (self.agents.len() as f32 * SELECTION_FRACTION) as usize;
        let bottom = self.agents.len() - top;
        for i in 0..bottom {
            let parent_a = &self.agents[random::<usize>() % top];
            let parent_b = &self.agents[random::<usize>() % top];

            // Crossover
            let mut child = parent_a.crossover(parent_b);

            // Mutation
            child.mutate();
            // Evaluate
            child.calculate_fitness(&self.goal);

            self.agents[top + i] = child;
        }
        
        // Sort
        self.agents.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }

    pub fn best(&self) -> &Agent {
        &self.agents[0]
    }
}

#[derive(Debug)]
pub struct Agent {
    functions: Vec<FunctionCoefficients>,
    pub fitness: f32,
}

impl ToString for Agent {
    fn to_string(&self) -> String {
        self.functions
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(" + ")
    }
}

impl Agent {
    pub fn evaluate(&self, x: f32) -> f32 {
        self.functions
            .iter()
            .map(|f| f.evaluate(x))
            .sum()
    }

    pub fn new_rand(functions: usize) -> Self {
        Self {
            functions: (0..functions).map(|_| FunctionCoefficients::new_rand()).collect(),
            fitness: 0.0,
        }
    }

    pub fn crossover(&self, other: &Self) -> Self {
        let min = self.functions.len().min(other.functions.len());
        let mut functions = Vec::with_capacity(min);

        for i in 0..min {
            let f1 = &self.functions[i];
            let f2 = &other.functions[i];
            functions.push(if random::<bool>() {
                f1.clone()
            } else {
                f2.clone()
            });
        }
        Self {
            functions,
            fitness: 0.0,
        }
    }

    pub fn mutate(&mut self) {
        let distr = rand_distr::Normal::new(0.0, MUTATION_STRENGTH).unwrap();

        if self.functions.len() < MAX_FUNCTIONS && random::<f32>() < FUNCTION_ADDITION_PROBABILITY {
            self.functions.push(FunctionCoefficients::new_rand());
        }

        if self.functions.len() > 1 && random::<f32>() < FUNCTION_SUBTRACTION_PROBABILITY {
            self.functions.remove(random::<usize>() % self.functions.len());
        }

        for f in self.functions.iter_mut() {
            if random::<f32>() < MUTATION_PROBABILITY {
                f.scale += distr.sample(&mut rand::thread_rng());
            }
        }
        for f in self.functions.iter_mut() {
            if random::<f32>() < MUTATION_PROBABILITY {
                f.x_translation += distr.sample(&mut rand::thread_rng());
            }
        }
        // TODO: Maybe mutate function type?
    }

    fn calculate_fitness(&mut self, goal: &[f32; WAVE_RES]) {
        self.fitness = goal.iter().enumerate()
            .map(|(i, &g)| -(self.evaluate((i as f32 / WAVE_RES as f32) * 2. - 1.) - g).abs())
            .sum();
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCoefficients {
    function_type: WaveFunction,
    scale: f32,
    x_translation: f32,
}

impl FunctionCoefficients {
    pub fn evaluate(&self, x: f32) -> f32 {
        self.function_type.evaluate(x, self.scale, self.x_translation)
    }

    pub fn new_rand() -> Self {
        Self {
            function_type: WaveFunction::iter().nth(random::<usize>() % WaveFunction::COUNT).unwrap(),
            scale: rand::random::<f32>() * 2.0 - 1.0,
            x_translation: rand::random::<f32>() * 2.0 - 1.0,
        }
    }
}

impl ToString for FunctionCoefficients {
    fn to_string(&self) -> String {
        self.function_type.generate_string(self.scale, self.x_translation)
    }
}

#[derive(Debug, EnumIter, EnumCount, Clone)]
pub enum WaveFunction {
    Sine,
    SawTooth,
}

impl WaveFunction {
    pub fn new(name: &str) -> &'static str {
        match name {
            Sine => "Sine",
            SawTooth => "SawTooth",
        }
    }

    pub fn evaluate(&self, x: f32, scale: f32, x_translation: f32) -> f32 {
        match self {
            Self::Sine => (x + x_translation).sin() * scale,
            Self::SawTooth => (x + x_translation).fract() * scale,
        }
    }

    fn generate_string(&self, scale: f32, x_translation: f32) -> String {
        match self {
            Self::Sine => format!("{scale}*sin(x + {x_translation})"),
            Self::SawTooth => format!("{scale}*frac(x + {x_translation})"),
        }
    }
}
