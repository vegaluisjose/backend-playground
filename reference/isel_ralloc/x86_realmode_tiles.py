from instruction_select import gen
from structures import *
from patterns import *
tiles = []

def tile(pat, cost):
    def _deco_(code):
        tiles.append((pat, cost, code))
        return code
    return _deco_

@tile(PConst(0), 10)
def _(const, block):
    dst = VReg()
    block.append(Code(
        ('xor {}, {}', dst, dst),
        defs={dst}))
    return dst

@tile(Int, 10)
def _(const, block):
    dst = VReg()
    block.append(Code(
        ('mov {}, {}', dst, const.value),
        defs={dst}))
    return dst

@tile(POp('add', [Any, Int]), 15)
def _(op, block):
    src1 = gen(op[0], block)
    src2 = op[1].value
    dst = VReg()
    block.append(Motion(dst, src1))
    block.append(Code(
        ('add {}, {}', dst, src2),
        uses={dst},
        defs={dst}))
    return dst

@tile(POp('add', [Any, Any]), 20)
def _(op, block):
    src1 = gen(op[0], block)
    src2 = gen(op[1], block)
    dst = VReg()
    block.append(Motion(dst, src1))
    block.append(Code(
        ('add {}, {}', dst, src2),
        uses={dst, src2},
        defs={dst}))
    return dst

@tile(POp('sub', [Any, Int]), 15)
def _(op, block):
    src1 = gen(op[0], block)
    src2 = op[1].value
    dst = VReg()
    block.append(Motion(dst, src1))
    block.append(Code(
        ('sub {}, {}', dst, src2),
        uses={dst},
        defs={dst}))
    return dst

@tile(POp('sub', [Any, Any]), 20)
def _(op, block):
    src1 = gen(op[0], block)
    src2 = gen(op[1], block)
    dst = VReg()
    block.append(Motion(dst, src1))
    block.append(Code(
        ('sub {}, {}', dst, src2),
        uses={dst, src2},
        defs={dst}))
    return dst
