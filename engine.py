import random

from cards import DECK, suit


class Durak:
    def __init__(self):
        deck = list(DECK)
        random.shuffle(deck)

        self.hands = [sorted(deck[0:6]), sorted(deck[6:12])]

        self.talon = deck[12:]
        self.trump_card = self.talon[0]
        self.trump = suit(self.trump_card)