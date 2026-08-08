from cards import rank, suit


def cost(card, trump):
    return rank(card) + (9 if suit(card) == trump else 0)