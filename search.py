import math
import random

from rollout import softmax_action


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
        exploration_part = exploration * math.sqrt(
            math.log(child.available) / child.visits
        )
        return exploitation_part + exploration_part

    return max((node.children[action] for action in actions), key=uct)


def value(game):
    if game.durak is None:
        return 0.5
    return 1.0 if game.durak == 1 else 0.0


def simulate(game):
    while game.phase != "OVER":
        game.apply(softmax_action(game))
    return value(game)


def backpropagate(node, player_0_value):
    while node is not None:
        node.visits += 1
        if node.player is not None:
            node.wins += player_0_value if node.player == 0 else 1 - player_0_value
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
