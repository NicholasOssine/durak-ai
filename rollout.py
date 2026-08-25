import math
import random

from heuristic import cost

TEMPERATURE = 2.0
MAX_COST = 17
TAKE_COST = 99

CARD_WEIGHT = [math.exp(-card_cost / TEMPERATURE) for card_cost in range(MAX_COST + 1)]
TAKE_WEIGHT = math.exp(-TAKE_COST / TEMPERATURE)


def softmax_action(game):
    actions = game.get_actions()
    if len(actions) == 1:
        return actions[0]

    trump = game.trump

    total = 0.0
    weights = []
    for action_type, card in actions:
        weight = TAKE_WEIGHT if card is None else CARD_WEIGHT[cost(card, trump)]
        weights.append(weight)
        total += weight

    target = random.random() * total
    running = 0.0
    for action, weight in zip(actions, weights):
        running += weight
        if running >= target:
            return action
    return actions[-1]
