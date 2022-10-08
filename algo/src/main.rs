mod direction;

use std::{cell::RefCell, rc::Rc};

use direction::Direction;

const PUZZLE_SIZE: i8 = 3;
const TOTAL_PUZZLE_SIZE: usize = 3 * 3;
const MOVABLE_PIECE: i8 = (TOTAL_PUZZLE_SIZE as i8) - 1;
// const PUZZLE_END_STATE: [i8; TOTAL_PUZZLE_SIZE] = 0..TOTAL_PUZZLE_SIZE.into();??

extern crate rand;

use rand::{seq::SliceRandom, thread_rng};

pub struct SolvedState {
    pub puzzle_state: [i8; TOTAL_PUZZLE_SIZE],
    pub moves: Vec<Direction>,
}

impl SolvedState {
    pub fn new(puzzle_state: [i8; TOTAL_PUZZLE_SIZE], moves: Vec<Direction>) -> SolvedState {
        SolvedState {
            puzzle_state,
            moves,
        }
    }
}

pub trait Solve {
    fn solve(&mut self, puzzle_state: &mut [i8; TOTAL_PUZZLE_SIZE]) -> SolvedState;
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Node {
    pub puzzle_state: [i8; TOTAL_PUZZLE_SIZE],
    pub length: i32,
    pub distance: i32,
    pub value: i32,
    pub direction: Direction,
    pub parent_node: Option<Rc<RefCell<Node>>>, 
    pub is_movable: bool,
    pub is_ending_node: bool,
}

impl Node {
    pub fn new(puzzle_state: [i8; TOTAL_PUZZLE_SIZE]) -> Self {
        let distance: i32 = Node::manhattan_distance(&puzzle_state);
        Self {
            puzzle_state,
            length: 0,
            distance,
            value: distance, // value is the distance + length
            direction: Direction::None,
            parent_node: None,
            is_movable: false,
            is_ending_node: false,
        }
    }

    pub fn new_with_parent(node: Rc<RefCell<Self>>, direction: Direction) -> Self {
        Self {
            puzzle_state: {let x = node.as_ref().borrow().puzzle_state; x},
            length: {let x = node.as_ref().borrow().length + 1; x},
            distance: 0,
            value: 0, // value is the distance + length
            direction,
            parent_node: Some(node),
            is_movable: false,
            is_ending_node: false,
        }
    }

    pub fn manhattan_distance(puzzle_state: &[i8; TOTAL_PUZZLE_SIZE]) -> i32 {
        let mut distance: i32 = 0;
        for (num, current_num) in puzzle_state.iter().enumerate() {
            let current_num_in_arr: i8 = num as i8;
            if current_num_in_arr != *current_num {
                let a: i32 = i32::abs(
                    (current_num_in_arr % PUZZLE_SIZE) as i32 - (*current_num % PUZZLE_SIZE) as i32,
                );
                let b: i32 = i32::abs(
                    (current_num_in_arr / PUZZLE_SIZE) as i32 - (*current_num / PUZZLE_SIZE) as i32,
                );
                distance += a;
                distance += b;
            }
        }
        distance
    }

    fn update(&mut self) {
        self.distance = Node::manhattan_distance(&self.puzzle_state);
        self.value = self.distance + self.length;
        self.is_movable = true;
    }
}

trait FindMovableNodes {
    fn get_possible_nodes(&self) -> Vec<Node>;
    fn left(&self) -> Node;
    fn right(&self) -> Node;
    fn up(&self) -> Node;
    fn down(&self) -> Node;
    fn is_out_of_bounds(current_position: usize, direction: Direction) -> bool;
    fn check_completion(node: &Node) -> bool;
}

impl FindMovableNodes for Rc<RefCell<Node>> {
    fn get_possible_nodes(&self) -> Vec<Node> {
        match self.as_ref().borrow().direction {
            Direction::Left => {
                filter_movable_nodes!([self.left(), self.up(), self.down()])
            },
            Direction::Right => {
                filter_movable_nodes!([self.right(), self.up(), self.down()])
            }
            Direction::Up => {
                filter_movable_nodes!([self.left(), self.right(), self.up()])
            },
            Direction::Down => {
                filter_movable_nodes!([self.left(), self.right(), self.down()])
            },
            Direction::None => {
                filter_movable_nodes!([self.left(), self.right(), self.up(), self.down()])
            },
        }
    }

    fn left(&self) -> Node {
        let mut left_node = Node::new_with_parent(self.clone(), Direction::Left);
        
        let movable_piece_location = left_node.puzzle_state.iter().position(|value| *value == MOVABLE_PIECE).unwrap();
        let left_of_movable_location: i8 = movable_piece_location as i8 - 1;

        if !Self::is_out_of_bounds(movable_piece_location, Direction::Left) {
            left_node.puzzle_state.swap(movable_piece_location, left_of_movable_location as usize);
            left_node.update();
        }

        if Self::check_completion(&left_node) {
            left_node.is_ending_node = true;
        }
        
        left_node
    }

    fn right(&self) -> Node {
        let mut right_node = Node::new_with_parent(self.clone(), Direction::Right);
        
        let movable_piece_location = right_node.puzzle_state.iter().position(|value| *value == MOVABLE_PIECE).unwrap();
        let right_of_movable_location: i8 = movable_piece_location as i8 + 1;

        if !Self::is_out_of_bounds(movable_piece_location, Direction::Right) {
            right_node.puzzle_state.swap(movable_piece_location, right_of_movable_location as usize);
            right_node.update();
        }

        if Self::check_completion(&right_node) {
            right_node.is_ending_node = true;
        }
        
        right_node
    }

    fn up(&self) -> Node {
        let mut up_node = Node::new_with_parent(self.clone(), Direction::Up);
        
        let movable_piece_location = up_node.puzzle_state.iter().position(|value| *value == MOVABLE_PIECE).unwrap();
        let up_of_movable_location: i8 = movable_piece_location as i8 - PUZZLE_SIZE;

        if !Self::is_out_of_bounds(movable_piece_location, Direction::Up) {
            up_node.puzzle_state.swap(movable_piece_location, up_of_movable_location as usize);
            up_node.update();
        }

        if Self::check_completion(&up_node) {
            up_node.is_ending_node = true;
        }
        
        up_node
    }

    fn down(&self) -> Node {
        let mut down_node = Node::new_with_parent(self.clone(), Direction::Down);
        
        let movable_piece_location = down_node.puzzle_state.iter().position(|value| *value == MOVABLE_PIECE).unwrap();
        let down_of_movable_location: i8 = movable_piece_location as i8 + PUZZLE_SIZE;

        if !Self::is_out_of_bounds(movable_piece_location, Direction::Down) {
            down_node.puzzle_state.swap(movable_piece_location, down_of_movable_location as usize);
            down_node.update();
        }

        if Self::check_completion(&down_node) {
            down_node.is_ending_node = true;
        }
        
        down_node
    }

    fn is_out_of_bounds(current_position: usize, direction: Direction) -> bool {
        let mut column = current_position as i8 % PUZZLE_SIZE;
        let mut row = current_position as i8 / PUZZLE_SIZE;
        match direction {
            Direction::Left => {
                column -= 1;
            },
            Direction::Right => {
                column += 1;
            },
            Direction::Up => {
                row -= 1;
            },
            Direction::Down => {
                row += 1;
            },
            Direction::None => {}
        };
        // !(0..PUZZLE_SIZE).contains(&column) || !(0..PUZZLE_SIZE).contains(&row) // stupid compiler prefers this
        column < 0 || column >= PUZZLE_SIZE || row < 0 || row >= PUZZLE_SIZE
    }

    fn check_completion(node: &Node) -> bool {
        node.puzzle_state.eq(&create_array_with_increasing_value())
    }
}


#[macro_export]
macro_rules! filter_movable_nodes {
    ($nodes:expr) => {{
        $nodes.into_iter().filter(|n| n.is_movable).collect::<Vec<_>>()
    }}
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

impl Default for AStar {
    fn default() -> Self {
        Self::new()
    }
}

impl Solve for AStar {
    fn solve(&mut self, puzzle_state: &mut [i8; TOTAL_PUZZLE_SIZE]) -> SolvedState {
        let mut open_list: Vec<Rc<RefCell<Node>>> = Vec::new();
        let mut closed_list: Vec<Rc<RefCell<Node>>> = Vec::new();
        let starting_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new(*puzzle_state)));

        open_list.push(starting_node);

        while !self.is_ending_node_reached {
            let (index, _) = open_list.iter().enumerate().min_by(|(_, a), (_, b)| { a.as_ref().borrow().value.cmp(&b.as_ref().borrow().value) }).unwrap();
            
            // Remember that remove has a runtime of O(n) as all elements after the index need to be shifted. 
            // Vec::swap_remove has a runtime of O(1) as it swaps the to-be-removed element with the last one.
            // If the order of elements is not important in your case, use swap_remove instead of remove!
            // https://stackoverflow.com/a/44012406/6134391

            closed_list.push(open_list.swap_remove(index));

            let min_value_node = closed_list.last().unwrap();

            for node in min_value_node.get_possible_nodes().into_iter() {
                if node.is_ending_node {
                    self.is_ending_node_reached = true;
                    self.ending_node = Some(node.clone());
                    break;
                }
                if !open_list.iter().any(|n| n.as_ref().borrow().puzzle_state.eq(&node.puzzle_state))
                && !closed_list.iter().any(|n| n.as_ref().borrow().puzzle_state.eq(&node.puzzle_state)) {
                    open_list.push(Rc::new(RefCell::new(node)));
                }
            }
            
            println!("openlist = {ol:?}| closedlist = {cl:?}", ol = open_list.len(), cl = closed_list.len());
        }

        let final_puzzle_state: [i8; TOTAL_PUZZLE_SIZE] = self.ending_node.as_ref().unwrap().puzzle_state;
        let mut node = Some(Rc::new(RefCell::new(self.ending_node.as_ref().unwrap().clone())));
        let mut moves: Vec<Direction> = Vec::new();

        '_move_loop: while node.as_ref().unwrap().as_ref().borrow().parent_node.is_some() {
            node = {
                let node_ = node.as_ref().unwrap().as_ref().borrow();
                moves.push(node_.direction);
                if node_.parent_node.is_none() {
                    break '_move_loop; 
                }
                node_.parent_node.clone()
            };
        }
        moves.reverse();

        SolvedState {
            puzzle_state: final_puzzle_state,
            moves,
        }
    }
}

fn main() {
    // let mut puzzle_state: [i8; TOTAL_PUZZLE_SIZE] = create_puzzle_state();
    let mut puzzle_state: [i8; TOTAL_PUZZLE_SIZE] = [5, 2, 3, 1, 8, 4, 6, 0, 7];

    let mut solver: AStar = AStar::new();
    let solved_state: SolvedState = solver.solve(&mut puzzle_state);

    println!("Puzzle state solved!");
    println!("{:?} moves!", solved_state.moves.len());
    println!("{:?}", solved_state.moves);
    println!("Final puzzle state {:?}!", solved_state.puzzle_state);
}

fn create_puzzle_state() -> [i8; TOTAL_PUZZLE_SIZE] {
    let mut puzzle_state: [i8; TOTAL_PUZZLE_SIZE] = create_array_with_increasing_value();

    let mut rng = thread_rng();
    puzzle_state.shuffle(&mut rng);

    while !check_solvability(&mut puzzle_state) {
        println!("Puzzle state not solvable: {:?}", puzzle_state);
        puzzle_state.shuffle(&mut rng);
    }
    println!("Solvable puzzle state found: {:?}", puzzle_state);
    
    puzzle_state
}

fn check_solvability(puzzle_state: &[i8; TOTAL_PUZZLE_SIZE]) -> bool {
    let temp = puzzle_state.iter().filter(|x| **x != MOVABLE_PIECE).collect::<Vec<_>>();
    let mut inversions: i32 = 0;
    for i in 0..temp.len() {
        for j in (i + 1)..temp.len() {
            if temp[i] > temp[j] {
                inversions += 1;
            }
        }
    }
    inversions % 2 == 0
}

fn create_array_with_increasing_value() -> [i8; TOTAL_PUZZLE_SIZE] {
    core::array::from_fn(|i: usize| i as i8)
}
