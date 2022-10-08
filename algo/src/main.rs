mod direction;
use std::{cell::RefCell, default, rc::Rc, borrow::{Borrow, BorrowMut}};

use direction::Direction;

const PUZZLE_SIZE: u8 = 4;
const TOTAL_PUZZLE_SIZE: usize = 4 * 4;
const MOVABLE_PIECE: u8 = (TOTAL_PUZZLE_SIZE as u8) - 1;

extern crate rand;

use rand::{seq::SliceRandom, thread_rng};

pub struct SolvedState {
    pub puzzle_state: [u8; TOTAL_PUZZLE_SIZE],
    pub moves: Vec<Direction>,
}

impl SolvedState {
    pub fn new(puzzle_state: [u8; TOTAL_PUZZLE_SIZE], moves: Vec<Direction>) -> SolvedState {
        SolvedState {
            puzzle_state,
            moves,
        }
    }
}

pub trait Solve {
    fn solve(&self, puzzle_state: &mut [u8; TOTAL_PUZZLE_SIZE]) -> SolvedState;
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Node {
    pub puzzle_state: [u8; TOTAL_PUZZLE_SIZE],
    pub length: i32,
    pub distance: i32,
    pub value: i32,
    pub direction: Direction,
    pub parent_node: Option<Rc<RefCell<Node>>>, // https://stackoverflow.com/a/58683565/6134391
    pub is_movable: bool,
    pub is_ending_node: bool,
}

impl Node {
    pub fn new(puzzle_state: [u8; TOTAL_PUZZLE_SIZE]) -> Node {
        let distance: i32 = Node::manhattan_distance(puzzle_state);
        return Node {
            puzzle_state,
            length: 0,
            distance,
            value: distance, // value is the distance + length
            direction: Direction::None,
            parent_node: Option::None,
            is_movable: false,
            is_ending_node: false,
        };
    }

    pub fn manhattan_distance(puzzle_state: [u8; TOTAL_PUZZLE_SIZE]) -> i32 {
        let mut distance: i32 = 0;
        for num in 0..puzzle_state.len() {
            let current_num_in_arr: u8 = num as u8;
            let current_num: u8 = puzzle_state[num];
            if current_num_in_arr != current_num {
                let a: i32 = i32::abs(
                    (current_num_in_arr % PUZZLE_SIZE) as i32 - (current_num % PUZZLE_SIZE) as i32,
                );
                let b: i32 = i32::abs(
                    (current_num_in_arr / PUZZLE_SIZE) as i32 - (current_num / PUZZLE_SIZE) as i32,
                );
                distance += a;
                distance += b;
            }
        }
        return distance;
    }

    pub fn get_possible_nodes(&self) -> Vec<Node> {
        todo!()
    }
}

pub struct AStar {
    ending_node: Option<Node>,
    pub is_ending_node_reached: bool,

}

impl AStar {

    pub fn new() -> AStar {
        AStar {
            ending_node: Option::None,
            is_ending_node_reached: false
        }
    }
}

impl Solve for AStar {


    fn solve(&self, puzzle_state: &mut [u8; TOTAL_PUZZLE_SIZE]) -> SolvedState {
        let mut open_list: Vec<Node> = Vec::new();
        let mut closed_list: Vec<Node> = Vec::new();
        let mut starting_node: Node = Node::new(puzzle_state.clone());

        open_list.push(starting_node);

        while !self.is_ending_node_reached {
            let min_value_node: Node = *open_list.iter().min_by(|a, b| a.value.cmp(&b.value)).unwrap();
            let index: usize = open_list.iter().position(|node: &Node| node == &min_value_node).unwrap();
            
            // Remember that remove has a runtime of O(n) as all elements after the index need to be shifted. 
            // Vec::swap_remove has a runtime of O(1) as it swaps the to-be-removed element with the last one.
            // If the order of elements is not important in your case, use swap_remove instead of remove!
            // https://stackoverflow.com/a/44012406/6134391

            open_list.swap_remove(index);

            closed_list.push(min_value_node);

            for node in min_value_node.get_possible_nodes().iter() {
                if !open_list.iter().any(|n| n.puzzle_state.eq(&node.puzzle_state)) {
                    if !closed_list.iter().any(|n| n.puzzle_state.eq(&node.puzzle_state)) {
                        open_list.push(*node);
                    }
                }
            }
        }

        let mut node = &self.ending_node;
        let final_puzzle_state: [u8; TOTAL_PUZZLE_SIZE] = node.as_ref().unwrap().puzzle_state;
        let mut moves: Vec<Direction> = Vec::new();


        '_move_loop: while node.as_ref().unwrap().parent_node.is_none() {
            // let node = node.as_ref().unwrap();
            moves.push(node.as_ref().unwrap().direction);
            if node.as_ref().unwrap().parent_node.is_none() {
                break '_move_loop; 
            }
            // let x = node.parent_node.as_deref().to_owned().unwrap().borrow();
            // node = &Some(node.parent_node.unwrap().borrow_mut());
        }
        moves.reverse();

        return SolvedState {
            puzzle_state: final_puzzle_state,
            moves: moves,
        };
    }
}

fn main() {
    let mut puzzle_state: [u8; TOTAL_PUZZLE_SIZE] = create_puzzle_state();

    let solver: AStar = AStar::new();
    let solved_state: SolvedState = solver.solve(&mut puzzle_state);

    println!("Puzzle state solved!");
    println!("{:?}!", solved_state.moves.len());
    println!("{:?}!", solved_state.moves);
    println!("Final puzzle state {:?}!", solved_state.puzzle_state);
}

fn create_puzzle_state() -> [u8; TOTAL_PUZZLE_SIZE] {
    let mut puzzle_state: [u8; TOTAL_PUZZLE_SIZE] = create_array_with_increasing_value();

    let mut rng = thread_rng();
    puzzle_state.shuffle(&mut rng);

    while !check_solvability(&mut puzzle_state) {
        println!("Puzzle state not solvable: {:?}", puzzle_state);
        puzzle_state.shuffle(&mut rng);
    }
    println!("Solvable puzzle state found: {:?}", puzzle_state);
    return puzzle_state;
}

fn check_solvability(puzzle_state: &[u8; TOTAL_PUZZLE_SIZE]) -> bool {
    let mut inversions: i32 = 0;
    for i in 0..puzzle_state.len() {
        for j in i..puzzle_state.len() {
            if puzzle_state[i] > puzzle_state[j] {
                inversions += 1;
            }
        }
    }
    return inversions % 2 == 0;
}

fn create_array_with_increasing_value() -> [u8; TOTAL_PUZZLE_SIZE] {
    let puzzle_state: [u8; TOTAL_PUZZLE_SIZE] = core::array::from_fn(|i: usize| i as u8);
    return puzzle_state;
}
