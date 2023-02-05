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

#[pyfunction]
pub fn build_complete_deck() -> Vec<Card> {

    let mut number_cards = build_number_cards();
    let mut face_cards = build_face_cards();

    number_cards.append(&mut face_cards);

    number_cards
}

// Expose the base object to the Python interface. We'll be dealing with cards
#[pyclass]
#[derive(Clone, Debug)]
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



// The Deck class object represents the initial interface for dealing with gameplay. Card objects
// exist within the Deck and offer the player and dealer a variety of options.
#[pyclass]
#[derive(Clone, Debug)]
pub struct Deck {
    deck: Vec<Card>,
}

#[pymethods]
impl Deck {
   
    // Constructor for the Deck class. We only need to construct the deck in it's base form. Other utilities
    // from the Deck class will be called as needed.
    #[new]
    fn new(deck_: Vec<Card>) -> Self {
        Self {
            deck: deck_,
        }
    }

    // Select a random card from the deck and remove that card from the deck
    fn deal_card(&mut self) -> (Card, Vec<Card>) {

        // Get a clone of the deck to operate on
        let mut deck = self.deck.clone();

        // We not only want to select a random card, we need to remove it from the deck.
        // We'll do that by keeping track of the index of the randomly selected card, and removing that
        // card by the given index.
        let index = (rand::random::<f32>() * deck.len() as f32).floor() as usize;
        let card = deck.remove( index );

        (card, deck)

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
    fn new() -> Self {
        Self {
            pre_flop: true,
            flop: false,
            post_flop: false,
        }
    }

    fn play_flop(&mut self) -> Self {
        Self {
            pre_flop: false,
            flop: true,
            post_flop: false

        }
    }

    fn play_postflop(&mut self) -> Self {
        Self {
            pre_flop: false,
            flop: false,
            post_flop: true

        }
    }

}

// Struct for managing the wagering state of the game
#[pyclass]
#[derive(Clone, Debug)]
pub struct Wager {
    ante_play: bool,
    ante_amt: u32, // Can't bid negative
    blinds_play: bool,
    blinds_amt: u32,
    play_wager: bool,
    play_amt: u32,
    trips_play: bool,
    trips_amt: u32,
    balance: i32 // Overall Balance can be negative
}

// // Set wagering rules
#[pymethods]
impl Wager {
    #[new]
    fn new(ante_play_: bool, ante_amt_: u32, blinds_play_: bool, blinds_amt_: u32, play_wager_: bool, play_amt_: u32, trips_play_: bool, trips_amt_: u32, balance_: i32) -> Self {
        Self {
            ante_play: ante_play_,
            ante_amt: ante_amt_, // Can't bid negative
            blinds_play: blinds_play_,
            blinds_amt: blinds_amt_,
            play_wager: play_wager_,
            play_amt: play_amt_,
            trips_play: trips_play_,
            trips_amt: trips_amt_,
            balance: balance_
        }
    }
}

// Base object for manipulating actions. 
#[pyclass]
#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    first_card: Card,
    second_card: Card,
    wagers: Wager,
}


// What are the actions a player can do?
#[pymethods]
impl Player {
    
    // "Instantiate" a new player
    #[new]
    fn new(name_: String, first: Card, second: Card, wagers_: Wager) -> Self {
        Self {
            name: name_,
            first_card: first,
            second_card: second,
            wagers: wagers_
        }
    }

     // Allows for printing to stdout
     fn __repr__(&self) -> String {
        format!("Player Name: {}, First Card: {:?}, Second Card: {:?}", self.name, self.first_card, self.second_card)
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn ultimate_holdem(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Card>()?;
    m.add_class::<Player>()?;
    m.add_class::<Wager>()?;
    m.add_class::<State>()?;
    m.add_class::<Deck>()?;
    m.add_function(wrap_pyfunction!(build_number_cards, m)?)?;
    m.add_function(wrap_pyfunction!(build_face_cards, m)?)?;
    m.add_function(wrap_pyfunction!(build_complete_deck, m)?)?;
    Ok(())
}
