import ultimate_holdem


def build_deck() -> list:
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

if __name__ == '__main__':
    deck = build_deck()
    print(deck)
   

