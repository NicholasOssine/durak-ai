import math
import random

from heuristic import cost

TEMPERATURE = 2.0
TAKE_COST = 99


def softmax_action(game):
    actions = game.get_actions()
    if len(actions) == 1:
        return actions[0]

    trump = game.trump
    costs = [TAKE_COST if card is None else cost(card, trump) for _, card in actions]

    weights = []
    for card_cost in costs:
        weight = math.exp(-card_cost / TEMPERATURE)
        weights.append(weight)

    return random.choices(actions, weights)[0]
