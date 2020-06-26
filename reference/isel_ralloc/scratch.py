"""
    This is the entry point.
"""
import instruction_select
import register_alloc
from structures import *
from patterns import *
from x86_realmode_tiles import tiles

code = Op('sub', [
    Op('add', [
        Op('sub', [Const(0), Const(6)]),
        Const(2)]),
    Op('add', [
        Op('sub', [Const(0), Const(6)]),
        Const(2)])])

block = []
dst = instruction_select.select(code, tiles, block)
colors = register_alloc.alloc(block)

# Next map the colors while emitting the code.
def colormap(node):
    if isinstance(node, VReg):
        return colors[node]
    return node

print("use16")
print("org 0x0")
for cell in block:
    #print(';   {:<64} {}'.format(cell, format_vregs("active=[{}]", cell.active)))
    if isinstance(cell, Code):
        form = map(colormap, cell.form)
        #print("    " + form[0].format(*form[1:]))
    elif isinstance(cell, Motion):
        # there's no coalescing, but this occassionally happens by fortune.
        if colors[cell.dst] != colors[cell.src]:
            print("    mov {}, {}".format(colors[cell.dst], colors[cell.src]))
    else:
        assert False
