from engine import Durak
from heuristic import heuristic_action
from search import ismcts_action


def play(agents):
    game = Durak()
    while game.phase != "OVER":
        game.apply(agents[game.current_player()](game))
    return game.durak


def match(agent_a, agent_b, games=100):
    losses = [0, 0]
    draws = 0

    for game_number in range(games):
        swapped = game_number % 2 == 1
        agents = [agent_b, agent_a] if swapped else [agent_a, agent_b]

        durak = play(agents)
        if durak is None:
            draws += 1
        else:
            losses[1 - durak if swapped else durak] += 1

    print(f"A lost {losses[0] / games:.1%}, B lost {losses[1] / games:.1%}, draws {draws}")


match(ismcts_action, heuristic_action, games=50)