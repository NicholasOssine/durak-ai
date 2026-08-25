import math
import random

from heuristic import cost

TEMPERATURE = 2.0
TAKE_COST = 99
WEIGHT = [math.exp(-card_cost / TEMPERATURE) for card_cost in range(TAKE_COST + 1)]


def softmax_action(game):
    actions = game.get_actions()
    if len(actions) == 1:
        return actions[0]

    trump = game.trump
    costs = [TAKE_COST if card is None else cost(card, trump) for _, card in actions]
    weights = [WEIGHT[card_cost] for card_cost in costs]

    return random.choices(actions, weights)[0]
