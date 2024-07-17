// -----BEGIN PUBLIC KEY-----
// MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAw7hQL4amu9GbMqrCgKef23uGxysYPR149WlB5txgT0sz+M6tT2Lo1vlpZWGObFFa+Cr4rJh8T7r6uaFwbIIoo7wMwGvfzVbbh3se342Dax7O0dtjzkHl2PCFzoyZiUI8I52B1aabW40zzi0PXOpxi5xKCz6S3U1KAea/X92vQr8Pu9JXnk6bcf9OLJrmbHyHW7SKQvQZ6OuQ8QP2KvzGZOL+LfIgysExAT+wcubKfTfrbB4MkpJ2ZypRv0mfhqtZiMEgnYE4Yej12em4BQ7m1wROA2fg3pc+yuKLy9/zhgsxAz9/3rXItacRA8xXIubAxkY15cwd+B1Fs3kh+cnVRQIDAQAB
// -----END PUBLIC KEY-----
// PRO0F OF OWN
// gdud2Z4Id5R9oq/C/mvqENBTLuZyRNIWl11UQwYkc8XGUGE8FBL75f46g1MwWT1Do+F7Y1/JD6j9bD8evr57ahWHzwW0nr6gigtm3BWKfLLTQ0Zzst8UYUnClNG5dJJjW7FKbMz1h+VTrcaKASD39pFi/jYXZiUgHOHewXFxAajPqQnvRDg3OTQ2nVZcFCGaxve6KO4PUEip6TFrhx9xKCIz7y5XSeS3A7MBiZpewMY6mQcGO9f/GQmNUNrAPUiwwHfCsnwmdbEfuKinOIwcX+sGtEkOHYCsCDUZdxKo69vkE+W8UzkSV6K5D94lL4XXOAuteJvunOa2oCDXwLcmAQ==
// Author of the program Opokin Dmitry Sergeevich (https://github.com/godcodehunter)
//
// Dedicated to my teachers Plato, Johann Wolfgang von Goethe, David Hilbert
// 
// We must know and we will know
//
// Solving the Halt Problem
//
// Google dive: https://drive.google.com/file/d/1XYZbE6UaOocMdqBg4v_BNsnPw8rYv43L/view?usp=drive_link
// IPFS: https://ipfs.io/ipfs/QmUvE6E3dPkRmNywpg6PSq1ZqNa9ppLLaS46H78mNETf92?filename=main.rs

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
    
    let mut start_of_loop: Option<usize> = None;
    let mut halt_point: Option<usize> = None;

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
            .entry(prev)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        transition
            .entry(prev)
            .or_insert_with(HashSet::new).insert(next);

        if number_of_occurrences[&prev] == 2 
        && transition[&prev].contains(&next) {
            halt_point = Some(prev);
            start_of_loop = Some(next);
            break;
        }

        prev = next;
    }

    print!("That's all folks! halt_point: {} start_of_loop: {}", halt_point.unwrap(), start_of_loop.unwrap())
}

