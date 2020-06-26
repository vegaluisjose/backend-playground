"""
    Forms the graphs to map instructions
    into tiles.
"""
from structures import *

class PFunc(object):
    def __init__(self, match):
        self.match = match

    def estimate(self, node, base):
        return base

    def tile_estimate(self, node):
        return node.tile.cost if node.tile is not None else float('inf')

class PConst(object):
    def __init__(self, value):
        self.value = value

    def match(self, node):
        if isinstance(node, Const):
            return node.value == self.value

    def estimate(self, node, base):
        return base

    def tile_estimate(self, node):
        return node.tile.cost if node.tile is not None else float('inf')

class POp(object):
    def __init__(self, name, operands):
        self.name = name
        self.operands = operands
        self.leaf = False

    def __getitem__(self, index):
        return self.operands[index]

    def __len__(self):
        return len(self.operands)

    def match(self, node):
        if not isinstance(node, Op):
            return False
        if node.name != self.name:
            return False
        if len(self) != len(node):
            return False
        return all(pat.match(sn) for pat, sn in zip(self, node))

    def estimate(self, node, base):
        return base + sum(pat.tile_estimate(sn) for pat, sn in zip(self, node))

    def tile_estimate(self, node):
        return self.estimate(node, 0)

Any = PFunc(lambda node: True)
Int = PFunc(lambda node: isinstance(node, Const) and isinstance(node.value, int))
