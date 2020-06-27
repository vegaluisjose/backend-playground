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
    def __init__(self, opcode, operands, dtype):
        self.opcode = opcode
        self.operands = operands
        self.dtype = dtype

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

if __name__ == "__main__":
    code = Op("add", [Op("input", [], 8), Op("input", [], 8)], 8)
    tile = Pattern("add", [Pattern("input", [], 8), Pattern("input", [], 8)], 8)
    print(tile.match(code))
