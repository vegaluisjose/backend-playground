"""
    Select instructions to emit
"""
class Tile(object):
    def __init__(self, cost, pat, code):
        self.cost = cost
        self.pat = pat
        self.code = code

def choose(code, tiles):
    for node in code.postorder([]):
        best = float('inf')
        for pat, base, code in tiles:
            if pat.match(node):
                cost = pat.estimate(node, base)
                if cost < best:
                    best = cost
                    node.tile = Tile(cost, pat, code)

def gen(node, block):
    return node.tile.code(node, block)

def select(code, tiles, block):
    choose(code, tiles)
    return gen(code, block)
