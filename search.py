import random

def determinize(game, player):
    game = game.copy()
    opponent = 1 - player

    hidden = game.hidden_from(player)
    random.shuffle(hidden)

    opponenet_hand_size = len(game.hands[opponent])
    game.hands[opponent] = hidden[:opponenet_hand_size]
    game.talon = hidden[opponenet_hand_size:]