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
    block.append(('xor', dst, dst))
    return dst

@tile(Int, 10)
def _(const, block):
    dst = VReg()
    block.append(('move', dst, const.value))
    return dst

@tile(POp('add', [Any, Int]), 15)
def _(op, block):
    src1 = gen(op[0], block)
    src2 = op[1].value
    dst = VReg()
    block.append(('move', dst, src1))
    block.append(('add', dst, src2))
    return dst

@tile(POp('add', [Any, Any]), 20)
def _(op, block):
    src1 = gen(op[0], block)
    src2 = gen(op[1], block)
    dst = VReg()
    block.append(('move', dst, src1))
    block.append(('add', dst, src2))
    return dst

@tile(POp('sub', [Any, Int]), 15)
def _(op, block):
    src1 = gen(op[0], block)
    src2 = op[1].value
    dst = VReg()
    block.append(('move', dst, src1))
    block.append(('sub', dst, src2))
    return dst

@tile(POp('sub', [Any, Any]), 20)
def _(op, block):
    src1 = gen(op[0], block)
    src2 = gen(op[1], block)
    dst = VReg()
    block.append(('move', dst, src1))
    block.append(('sub', dst, src2))
    return dst
