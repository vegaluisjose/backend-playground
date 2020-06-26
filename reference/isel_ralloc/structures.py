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

class Code(object):
    def __init__(self, form, uses=None, defs=None):
        self.form = form
        self.uses = set() if uses is None else uses
        self.defs = set() if defs is None else defs

    def __str__(self):
        line = self.form[0].format(*self.form[1:])
        if len(self.defs) > 0:
            line = line.ljust(25) + format_vregs(" def[{}]", self.defs)
        if len(self.uses) > 0:
            line = line.ljust(40) + format_vregs(" use[{}]", self.uses)
        return line

class Motion(object):
    def __init__(self, dst, src):
        self.dst = dst
        self.src = src
        self.uses = {src}
        self.defs = {dst}

    def __str__(self):
        return "{} = {}".format(self.dst, self.src)

def format_vregs(form, vregs):
    return form.format(', '.join(map(repr, vregs)))