from loguru import logger
from typing import Tuple

import random
import ultimate_holdem
# from ultimate_holdem import build_complete_deck





def play_ultimate() -> dict:
    """
    Play a game of Ultimate Texas HoldEm.
    
    Rounds include Pre-Flop, Flop and Post-Flop

    """
    # Instantiate deck for gameplay
    deck_constructor = ultimate_holdem.build_complete_deck()
    deck = ultimate_holdem.Deck(deck_constructor)

    # Instantiate Game Log
    game_log = {"game": 1}

    player_name = input("Enter your name: ")

    first_card, deck = deck.deal_card()
    second_card, deck = ultimate_holdem.Deck(deck).deal_card()

    wagers = ultimate_holdem.Wager(False, 0, False, 0, False, 0, False, 0, 0)

    player = ultimate_holdem.Player(player_name, first_card, second_card, wagers)
    print(player)


    return game_log
