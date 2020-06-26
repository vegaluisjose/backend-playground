"""
    This is the entry point.
"""
import instruction_select
from structures import *
from patterns import *
from x86_realmode_tiles import tiles

code = Op('add', [
    Op('sub', [Const(0), Const(6)]), Const(2)])

block = []
dst = instruction_select.select(code, tiles, block)
for row in block:
    print(row)
