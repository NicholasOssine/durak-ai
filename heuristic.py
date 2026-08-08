from cards import rank, suit


def cost(card, trump):
    return rank(card) + (9 if suit(card) == trump else 0)


def heuristic_action(game, actions):
    trump = game.trump

    if game.phase == "DEFEND":
        defences = [card for action_type, card in actions if action_type == "defend"]
        if not defences:
            return ("take", None)
        return ("defend", min(defences, key=lambda card: cost(card, trump)))

    attacks = [card for action_type, card in actions if action_type == "attack"]
    if not attacks:
        return ("end", None)
    return ("attack", min(attacks, key=lambda card: cost(card, trump)))
