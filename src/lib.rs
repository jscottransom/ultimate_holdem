use pyo3;
use pyo3::prelude::*;
use std::vec::Vec;

// Build the list of number cards for gameplay
#[pyfunction]
pub fn build_number_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    let suits = vec!["Spades", "Hearts", "Clubs", "Diamonds"];
    for cardsuit in suits {
        for n in 2..10 {
            let number_string = n.to_string();
            let borrowed_number_string: &str = &number_string;
            let card = Card {
                suit: cardsuit.to_string(),
                value: borrowed_number_string.to_string(),
            };

            cards.push(card);
        }
    }

    cards
}

// Build the list of face cards for gameplay
#[pyfunction]
pub fn build_face_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    let suits = vec!["Spades", "Hearts", "Clubs", "Diamonds"];
    let faces = vec!["Ace", "King", "Queen", "King"];
    for cardsuit in suits {
        for face in &faces {
            let card = Card {
                suit: cardsuit.to_string(),
                value: face.to_string(),
            };

            cards.push(card);
        }
    }

    cards
}

// Expose the base object to the Python interface. We'll be dealing with cards
#[pyclass]
pub struct Card {
    suit: String,
    value: String,
}

#[pymethods]
impl Card {
    // py03 doesn't allow for instantiation in the "Pythonic" way
    // but it does offer a #[new] macro that essentially serves the same purpose
    #[new]
    fn new(cardsuit: String, cardvalue: String) -> Self {
        Self {
            suit: cardsuit,
            value: cardvalue,
        }
    }

    // Obtain the card's value
    fn get_value(&self) -> String {
        self.value.to_string()
    }

    // Obtain the card's suit
    fn get_suit(&self) -> String {
        self.suit.to_string()
    }

    // Get the path to the card's Image file for rendering
    fn get_image_path(&self) -> String {
        let numerical_rep = match self.value.as_str() {
            "2" => "two".to_string(),
            "3" => "three".to_string(),
            "4" => "four".to_string(),
            "5" => "five".to_string(),
            "6" => "six".to_string(),
            "7" => "seven".to_string(),
            "8" => "eight".to_string(),
            "9" => "nine".to_string(),
            "10" => "ten".to_string(),
            "Jack" => "jack".to_string(),
            "Queen" => "queen".to_string(),
            "King" => "king".to_string(),
            "Ace" => "ace".to_string(),
            &_ => "".to_string(),
        };

        // We need to deal with Strings allocated on the heap (size unknown at compile time).
        // So to apply operations, it makes sense to conver to a string slice, then re convert to a
        // String type.
        let suit = self.suit.as_str().to_lowercase().to_string();

        format!("card_images/{}_{}.jpg", numerical_rep, suit)
    }

    // Allows for printing to stdout
    fn __repr__(&self) -> String {
        format!("Suit: {}, Value: {}", self.suit, self.value)
    }
}

// In pure Rust, this would be an enum
// Keeps track of the state of gameplay
#[pyclass]
pub struct State {
    pre_flop: bool,
    flop: bool,
    post_flop: bool

}

#[pymethods]
impl State {
    // Once a state is instantiated, this will hold for each session of gameplay
    // We default the state of the pre-flop to true, since this is a new session of Gameplay.
    #[new]
    fn new(&mut self) -> Self {
        Self {
            pre_flop: True,
            flop: False,
            post_flop: False,
        }
    }

    fn play_flop(&mut self) -> Self {
        Self {
            pre_flop: False,
            flop: True,
            post_flop: False

        }
    }

    fn play_postflop(&mut self) -> Self {
        Self {
            pre_flop: False,
            flop: False,
            post_flop: True

        }
    }

}

// Struct for managing the wagering state of the game
#[pyclass]
pub struct Wager {
    ante_play: bool,
    ante_amt: u32 // Can't bid negative
    blinds_play: bool,
    blinds_amt: u32,
    play_wager: bool,
    play_amt: u32,
    trips_play: bool,
    trips_amt: u32,
    balance: i32 // Overall Balance can be negative
}

// Set wagering rules
#[pymethods]



// Base object for manipulating actions. 
#[pyclass]
pub struct Player {
    name: String
    first_card: Card,
    second_card: Card,
    wagers: Wager,
}


// What are the actions a player can do?
#[pymethods]
impl Player {
    
    // "Instantiate" a new player
    #[new]
    fn new(name_: String, first: String, second: String) -> Self {
        Self {
            name: name_,
            first_card: first,
            second_card: second,
        }
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn ultimate_holdem(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Card>()?;
    m.add_function(wrap_pyfunction!(build_number_cards, m)?)?;
    m.add_function(wrap_pyfunction!(build_face_cards, m)?)?;
    Ok(())
}
