import math
import random

from heuristic import heuristic_action


class Node:
    def __init__(self, parent=None, action=None, player=None):
        self.parent = parent
        self.children = {}

        self.action = action
        self.player = player

        self.visits = 0
        self.wins = 0
        self.available = 0

    def untried(self, actions):
        return [action for action in actions if action not in self.children]

    def add_child(self, action, player):
        child = Node(parent=self, action=action, player=player)
        self.children[action] = child
        return child
    

def determinize(game, player):
    game = game.copy()
    opponent = 1 - player

    hidden = game.hidden_from(player)
    if game.talon:
        hidden.remove(game.trump_card)
    random.shuffle(hidden)

    opponent_hand_size = len(game.hands[opponent])
    game.hands[opponent] = hidden[:opponent_hand_size]
    rest = hidden[opponent_hand_size:]
    game.talon = [game.trump_card] + rest if game.talon else rest

    return game


def uct_select(node, actions, exploration):
    for action in actions:
        node.children[action].available += 1

    def uct(child):
        exploitation_part = child.wins / child.visits
        exploration_part = exploration * math.sqrt(math.log(child.available) / child.visits)
        return exploitation_part + exploration_part

    return max((node.children[action] for action in actions), key=uct)


def simulate(game):
    while game.phase != "OVER":
        game.apply(heuristic_action(game, game.get_actions()))
    return game.durak


def score(durak, player):
    if durak is None:
        return 0.5
    return 1 if durak != player else 0


def backpropagate(node, durak):
    while node is not None:
        node.visits += 1
        if node.player is not None:
            node.wins += score(durak, node.player)
        node = node.parent


def iterate(root, game, player, exploration):
    node = root
    state = determinize(game, player)

    actions = state.get_actions()
    while state.phase != "OVER" and not node.untried(actions):
        node = uct_select(node, actions, exploration)
        state.apply(node.action)
        actions = state.get_actions()

    if state.phase != "OVER":
        action = random.choice(node.untried(actions))
        mover = state.current_player()
        state.apply(action)
        node = node.add_child(action, mover)

    backpropagate(node, simulate(state))


def ismcts_action(game, iterations=1000, exploration=0.7):
    player = game.current_player()
    root = Node()

    for _ in range(iterations):
        iterate(root, game, player, exploration)

    return max(root.children.items(), key=lambda item: item[1].visits)[0]
