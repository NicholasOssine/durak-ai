import random

from endgame import best_action
from engine import Durak


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
