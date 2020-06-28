class Tile(object):
    def __init__(self, cost, code):
        self.cost = cost
        self.code = code

class Op(object):
    def __init__(self, opcode, operands, dtype):
        self.opcode = opcode
        self.operands = operands
        self.dtype = dtype
        self.tile = None

    def __getitem__(self, index):
        return self.operands[index]

    def __len__(self):
        return len(self.operands)

    def postorder(self, prefix):
        for operand in self.operands:
            operand.postorder(prefix)
        prefix.append(self)
        return prefix

class Pattern(object):
    def __init__(self, opcode, operands, dtype, code="input", cost=0):
        self.opcode = opcode
        self.operands = operands
        self.dtype = dtype
        self.code = code
        self.cost = cost

    def __getitem__(self, index):
        return self.operands[index]

    def __len__(self):
        return len(self.operands)

    def match(self, node):
        if not isinstance(node, Op):
            return False
        if node.opcode != self.opcode:
            return False
        if len(self) != len(node):
            return False
        return all(p.match(n) for p, n in zip(self, node))

    def estimate(self, node):
        return self.cost + sum(p.__estimate(n) for p, n in zip(self, node))

    def __estimate(self, node):
        return node.tile.cost if node.tile is not None else float("inf")

def instruction_selection(code, patterns):
    for node in code.postorder([]):
        best = float('inf')
        for pat in patterns:
            if pat.match(node):
                cost = pat.estimate(node)
                if cost < best:
                    best = cost
                    node.tile = Tile(cost, pat.code)

def codegen(code):
    for node in code.postorder([]):
        print(node.tile.cost, node.tile.code)

if __name__ == "__main__":
    code = Op("add", [Op("input", [], 8), Op("input", [], 8)], 8)
    patterns = []
    patterns.append(Pattern("input", [], 8, code="input"))
    patterns.append(Pattern("add", [Pattern("input", [], 8), Pattern("input", [], 8)], 8, code="dsp_add", cost=1))
    patterns.append(Pattern("add", [Pattern("input", [], 8), Pattern("input", [], 8)], 8, code="lut_add", cost=2))
    instruction_selection(code, patterns)
    codegen(code)
