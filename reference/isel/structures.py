"""
    Intermediate representation for
    programs.
"""
class Const(object):
    def __init__(self, value, klass=None):
        self.value = value
        self.klass = klass
        self.tile = None

    def postorder(self, prefix):
        prefix.append(self)
        return prefix

class Op(object):
    def __init__(self, name, operands):
        self.name = name
        self.operands = operands
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

class VReg(object):
    next_uid = 1
    def __init__(self, assign=None, klass=None):
        self.assign = assign
        self.klass = klass
        self.uid = VReg.next_uid
        VReg.next_uid += 1

    def __repr__(self):
        return 'vreg{}'.format(self.uid)
