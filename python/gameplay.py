from loguru import logger
from typing import Tuple

import random
import ultimate_holdem


class Deck:
    """
    The Deck class object represents the initial interface for dealing with gameplay. Card objects
    exist within the Deck and offer the player and dealer a variety of options.
    """

    def __init__(self):
        """
        Constructor for the Deck class. We only need to construct the deck in it's base form. Other utilities
        from the Deck class will be called as needed.
        """
        self.base_deck = self.build_deck()

    @classmethod
    def build_deck(cls) -> list:
        """
        Builds a standard deck of cards used for gameplay.

        Args:
            None

        Returns:
            list: A list of Card objects represeenting playable values
        """
        number_cards = ultimate_holdem.build_number_cards()
        face_cards = ultimate_holdem.build_face_cards()

        return [number_cards + face_cards]

    @property
    def deck(self):
        return self.deck

    @deck.setter
    def deck(self, value):
        """
        Validation function for deck list composed of the 'Card' class
        """

        if not isinstance(value, list):
            raise TypeError("Value must be a list")

        else:
            self.deck = value

    @staticmethod
    def deal_card(deck: list) -> Tuple[list, ultimate_holdem.Card]:
        """
        Deal a number of cards from the deck


        Args:
            deck (list[ultimate_holdem.Card]): A list of Card objects

        Returns:
            list, ultimate_holdem.Card: Deck without the dealt card, and the dealt card.
        """

        # Shuffle the deck
        random.shuffle(deck)
        dealt_card = deck.pop()

        return deck, dealt_card


def play_ultimate() -> dict:
    """
    Play a game of Ultimate Texas HoldEm.
    
    Rounds include Pre-Flop, Flop and Post-Flop

    """
    # Instantiate deck for gameplay
    deck = Deck()

    # Instantiate Game Log
    game_log = {"game": 1}

    player_name = input("Enter your name: ")

    first_card = Deck.deal_card(deck)
    second_card = Deck.deal_card(deck)

    wagers = ultimate_holdem.Wager(False, 0, False, 0, False, 0, False, 0, 0)

    player = ultimate_holdem.Player(player_name, first_card, second_card, wagers)
    print(player)


    return game_log
