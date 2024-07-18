use std::{collections::{HashMap, HashSet}, iter::Map};

pub enum MoveDirection {
    Left,
    Right,
}

pub struct Transition {
    pub write: bool,
    pub move_direction: MoveDirection,
    pub new_state: Option<usize>,
}

pub struct TState {
    pub if_zero: Transition,
    pub if_one: Transition,
}

pub struct TMachine {
    pub ribbon: Vec<bool>,
    pub head_position: usize,
    pub states: Vec<TState>,
    pub current_state: usize,
}

pub enum InvalidateReason {
    OutOfRibbon(usize),
    OutOfStateSpace(usize),
}

pub enum StepResult {
    Ok,
    Halt,
    Invalidate(InvalidateReason),
}

impl TMachine {
    pub fn new(
        ribbon: impl IntoIterator<Item = bool>,
        states: impl IntoIterator<Item = TState>,
        head_position: usize,
        initial_state: usize,
    ) -> Self {
        Self {
            ribbon: ribbon.into_iter().collect(),
            head_position,
            states: states.into_iter().collect(),
            current_state: initial_state,
        }
    }

    pub fn step(&mut self) -> StepResult {
        let state = match self.states.get(self.current_state) {
            Some(s) => s,
            None => return StepResult::Invalidate(InvalidateReason::OutOfStateSpace(self.current_state)),
        };
        let &value = match self.ribbon.get(self.head_position) {
            Some(v) => v,
            None => {
                return StepResult::Invalidate(InvalidateReason::OutOfRibbon(self.head_position))
            }
        };

        let transition = if value { &state.if_one } else { &state.if_zero };

        self.ribbon[self.head_position] = transition.write;
        match transition.move_direction {
            MoveDirection::Left => self.head_position -= 1,
            MoveDirection::Right => self.head_position += 1,
        }

        if let Some(n) = transition.new_state {
            self.current_state = n;
        } else {
            return StepResult::Halt;
        }

        StepResult::Ok
    }

    pub fn ribbon(&self) -> String {
        self.ribbon
            .iter()
            .map(|&bit| if bit { '1' } else { '0' })
            .collect()
    }
}

fn main() {
    let s = vec![
        // A
        TState {
            if_zero: Transition {
                write: true,
                move_direction: MoveDirection::Right,
                new_state: Some(1),
            },
            if_one: Transition {
                write: true,
                move_direction: MoveDirection::Left,
                new_state: Some(2),
            },
        },
        // B
        TState {
            if_zero: Transition {
                write: true,
                move_direction: MoveDirection::Left,
                new_state: Some(0),
            },
            if_one: Transition {
                write: true,
                move_direction: MoveDirection::Right,
                new_state: Some(1),
            },
        },
        // C
        TState {
            if_zero: Transition {
                write: true,
                move_direction: MoveDirection::Left,
                new_state: Some(1),
            },
            if_one: Transition {
                write: true,
                move_direction: MoveDirection::Right,
                new_state: Some(0),
            },
        },
    ];

    let mut m = TMachine::new(
        vec![false; 15], 
        s, 
        7, 
        0,
    );
    

    let mut number_of_occurrences = HashMap::<usize, usize>::new();

    // Start state / end state
    let mut transition = HashMap::<usize, HashSet<usize>>::new();
    
    let mut super_position_two: Option<usize> = None;
    let mut super_position_one: Option<usize> = None;

    let mut prev: usize = 0;
    let mut next: usize = 0;
    
    prev = m.current_state;
    number_of_occurrences
            .entry(prev)
            .and_modify(|e| *e += 1)
            .or_insert(1);

    loop { 
        m.step();
        next = m.current_state;

        number_of_occurrences
            .entry(next)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        transition
            .entry(prev)
            .or_insert_with(HashSet::new).insert(next);

        if number_of_occurrences[&prev] == 2 
        && number_of_occurrences[&next] == 1
        && transition[&prev].contains(&next) {
            super_position_one = Some(prev);
            super_position_two = Some(next);
            break;
        }

        prev = next;
    }

    print!("That's all folks! start_of_loop: {} halt_point: {}", super_position_one.unwrap(), super_position_two.unwrap())
}

