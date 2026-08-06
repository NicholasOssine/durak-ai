"""
Card representation for Durak (36-card deck)

Cards are plain ints rather than objects to optimize for any future
search algorithms which would involve repeatedly copying game state
"""

# A card is an int from 0 to 35, encoded as rank * 4 + suit
# Rank index: 0 = six ... 8 = ace
# Suit index: 0 = spades, 1 = clubs, 2 = hearts, 3 = diamonds

DECK = tuple(range(36))


def rank(card):
    return card // 4


def suit(card):
    return card % 4


def beats(attack, defence, trump):
    if suit(defence) == suit(attack):
        return rank(defence) > rank(attack)
    return suit(defence) == trump

