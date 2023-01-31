use pyo3::prelude::*;
use pyo3;
use std::vec::Vec;


#[pyfunction]
pub fn build_number_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    let suits = vec!["Spades", "Hearts", "Clubs", "Diamonds"];
    for cardsuit in suits {
        for n in 2..10 {

            let number_string = n.to_string();
            let borrowed_number_string: &str = &number_string;
            let mut card = Card {
                suit: cardsuit.to_string(),
                value: borrowed_number_string.to_string()
            };

            cards.push(card);
        }
    }

    cards
}

#[pyfunction]
pub fn build_face_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    let suits = vec!["Spades", "Hearts", "Clubs", "Diamonds"];
    let faces = vec!["Ace", "King", "Queen", "King"];
    for cardsuit in suits {
        for face in &faces {

            let mut card = Card {
                suit: cardsuit.to_string(),
                value: face.to_string()
            };

            cards.push(card);
        }
    }

    cards
}


#[pyclass]
pub struct Card {
    suit: String,
    value: String
}

#[pymethods]
impl Card {
    #[new]
    fn new(cardsuit: String, cardvalue: String) -> Self {
        Card {
            suit: cardsuit,
            value: cardvalue
        }
    }

    fn __repr__(&self) -> String {
        // Allows for printing to stdout
        format!("Suit: {}, Value: {}", self.suit, self.value)
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