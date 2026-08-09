from engine import Durak


def play(agents):
    game = Durak()
    while game.phase != "OVER":
        game.apply(agents[game.current_player()](game))
    return game.durak


def match(agent_a, agent_b, games=100):
    losses = [0, 0]
    draws = 0

    for _ in range(games):
        durak = play([agent_a, agent_b])
        if durak is None:
            draws += 1
        else:
            losses[durak] += 1

    print(f"A lost {losses[0] / games:.1%}, B lost {losses[1] / games:.1%}, draws {draws}")
