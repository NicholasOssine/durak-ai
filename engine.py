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
        self.attacker = self.first_attacker()

    def first_attacker(self):
        best_player = 0
        best_trump = None

        for player in (0, 1):
            hand = self.hands[player]
            for card in hand:
                if suit(card) == self.trump and (best_trump is None or card < best_trump):
                    best_trump = card
                    best_player = player

        return best_player

