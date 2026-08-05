import random

from cards import DECK, beats, rank, suit


class Durak:
    def __init__(self):
        deck = list(DECK)
        random.shuffle(deck)

        self.hands = [sorted(deck[0:6]), sorted(deck[6:12])]

        self.talon = deck[12:]
        self.trump_card = self.talon[0]
        self.trump = suit(self.trump_card)

        self.attacker = self.first_attacker()
        self.table = []
        self.phase = "ATTACK"
        self.max_attacks = min(6, len(self.hands[1 - self.attacker]))

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

    def get_actions(self):
        if self.phase == "DEFEND":
            defender = 1 - self.attacker
            attack_card = self.table[-1][0]
            actions = [("defend", card) for card in self.hands[defender] if beats(attack_card, card, self.trump)]
            actions.append(("take", None))
            return actions
        
        if self.phase == "ATTACK" and not self.table:
            return [("attack", card) for card in self.hands[self.attacker]]
        
        actions = []
        if len(self.table) < self.max_attacks:
            table_ranks = set()
            for row in self.table:
                for card in row:
                    if card is not None:
                        table_ranks.add(rank(card))
            actions = [("attack", card) for card in self.hands[self.attacker] if rank(card) in table_ranks]

        actions.append(("end", None))
        return actions
