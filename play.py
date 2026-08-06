import random

from engine import Durak


def play():
    game = Durak()
    while game.phase != "OVER":
        game.apply(random.choice(game.get_actions()))
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
