import random
from copy import deepcopy

from engine import Durak

# Player 0 is max, player 1 is min
def utility(game):
    durak = game.durak
    return 0 if durak is None else 1 if durak == 1 else -1


def minimax(game, alpha=float("-inf"), beta=float("inf")):
    if game.phase == "OVER":
        return utility(game)

    if game.current_player() == 0:
        value = float("-inf")
        for action in game.get_actions():
            child = deepcopy(game)
            child.apply(action)
            value = max(value, minimax(child, alpha, beta))
            alpha = max(alpha, value)
            if value >= beta:
                break
        return value
    else:
        value = float("inf")
        for action in game.get_actions():
            child = deepcopy(game)
            child.apply(action)
            value = min(value, minimax(child, alpha, beta))
            beta = min(beta, value)
            if value <= alpha:
                break
        return value


def best_action(game):
    chosen_action = None

    if game.current_player() == 0:
        chosen_value = float("-inf")
        for action in game.get_actions():
            child = deepcopy(game)
            child.apply(action)
            value = minimax(child, chosen_value, float("inf"))
            if value > chosen_value:
                chosen_action = action
                chosen_value = value
    else:
        chosen_value = float("inf")
        for action in game.get_actions():
            child = deepcopy(game)
            child.apply(action)
            value = minimax(child, float("-inf"), chosen_value)
            if value < chosen_value:
                chosen_action = action
                chosen_value = value

    return chosen_action


def play():
    game = Durak()
    while game.phase != "OVER":
        if not game.talon:
            action = best_action(game)
        else:
            action = random.choice(game.get_actions())
        game.apply(action)
    return game.durak


def run_games(num_games):
    results = {
        0: 0,
        1: 0,
        None: 0,
    }

    for _ in range(num_games):
        results[play()] += 1

    print(results)

run_games(1000)
