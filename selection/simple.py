class Info(object):
    def __init__(self, cost, pattern, code):
        self.cost = cost
        self.pattern = pattern
        self.code = code

class Op(object):
    def __init__(self, opcode, operands, dtype):
        self.opcode = opcode
        self.operands = operands
        self.dtype = dtype
        self.info = None

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
    def __init__(self, opcode, operands, dtype, target="io", cost=0):
        self.opcode = opcode
        self.operands = operands
        self.dtype = dtype
        self.target = target
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
        return node.info.cost if node.info is not None else float("inf")

def selection(code, patterns):
    for node in code.postorder([]):
        best = float('inf')
        for pat in patterns:
            if pat.match(node):
                cost = pat.estimate(node)
                if cost < best:
                    print("found new best, opcode:{} target:{}".format(pat.opcode, pat.target))
                    best = cost
                    node.info = Info(cost, pat, code)

if __name__ == "__main__":
    code = Op("add", [Op("input", [], 8), Op("input", [], 8)], 8)
    patterns = []
    patterns.append(Pattern("input", [], 8, target="io"))
    patterns.append(Pattern("add", [Pattern("input", [], 8), Pattern("input", [], 8)], 8, target="dsp", cost=1))
    patterns.append(Pattern("add", [Pattern("input", [], 8), Pattern("input", [], 8)], 8, target="lut", cost=2))
    selection(code, patterns)
