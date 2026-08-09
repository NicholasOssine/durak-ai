from cards import rank, suit


def cost(card, trump):
    return rank(card) + (9 if suit(card) == trump else 0)


def heuristic_action(game):
    trump = game.trump
    actions = game.get_actions()

    if game.phase == "DEFEND":
        defences = [card for action_type, card in actions if action_type == "defend"]
        if not defences:
            return ("take", None)
        
        cheapest = min(defences, key=lambda card: cost(card, trump))
        attack_card = game.table[-1][0]
        
        if suit(cheapest) == trump and suit(attack_card) != trump and rank(attack_card) < 5:
            return ("take", None)
        return("defend", cheapest)

    attacks = [card for action_type, card in actions if action_type == "attack"]
    if not attacks:
        return ("end", None)
    return ("attack", min(attacks, key=lambda card: cost(card, trump)))
