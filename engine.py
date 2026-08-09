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
        self.discard = []
        self.phase = "ATTACK"
        self.max_attacks = min(6, len(self.hands[1 - self.attacker]))

        self.durak = None

    def copy(self):
        game = Durak.__new__(Durak)
        game.hands = [self.hands[0][:], self.hands[1][:]]
        game.talon = self.talon[:]
        game.trump = self.trump
        game.trump_card = self.trump_card
        game.table = [row[:] for row in self.table]
        game.discard = self.discard[:]
        game.attacker = self.attacker
        game.phase = self.phase
        game.max_attacks = self.max_attacks
        game.durak = self.durak
        return game

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
        if self.phase == "OVER":
            return []

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

    def apply(self, action):
        action_type, card = action

        if action_type == "attack":
            self.hands[self.attacker].remove(card)
            self.table.append([card, None])
            if self.phase != "TAKING":
                self.phase = "DEFEND"

        elif action_type == "defend":
            self.hands[1 - self.attacker].remove(card)
            self.table[-1][-1] = card
            self.phase = "ATTACK"

        elif action_type == "take":
            self.phase = "TAKING"

        elif action_type == "end":
            cards = []
            for row in self.table:
                for card in row:
                    if card is not None:
                        cards.append(card)

            defender = 1 - self.attacker
            if self.phase == "TAKING":
                self.hands[defender].extend(cards)
            else:
                self.discard.extend(cards)

            self.table.clear()
            self.draw(self.attacker)
            self.draw(defender)

            if not self.talon:
                if not self.hands[self.attacker] or not self.hands[defender]:
                    self.phase = "OVER"
                    if not self.hands[self.attacker] and not self.hands[defender]:
                        self.durak = None
                    elif not self.hands[self.attacker]:
                        self.durak = defender
                    else:
                        self.durak = self.attacker
                    return

            if self.phase != "TAKING":
                self.attacker = defender

            self.phase = "ATTACK"
            self.max_attacks = min(6, len(self.hands[1 - self.attacker]))

    def draw(self, player):
        hand = self.hands[player]
        while len(hand) < 6 and self.talon:
            hand.append(self.talon.pop())

    def current_player(self):
        if self.phase == "DEFEND":
            return 1 - self.attacker
        return self.attacker

    def hidden_from(self, player):
        seen = set(self.hands[player])
        seen.update(self.discard)
        for row in self.table:
            for card in row:
                if card is not None:
                    seen.add(card)
        return [card for card in DECK if card not in seen]
