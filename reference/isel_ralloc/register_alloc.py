"""
    Select location for every virtual register.
"""
from structures import *

def alloc(block):
    # At this point, virtual registers are replaced with real registers.
    registers = {'ax', 'bx', 'cx', 'dx'}
    colors = {}

    # Our graph contains both interferences and coalescing knowledge.
    graph = {}
    def get(vreg):
        if vreg in graph:
            return graph[vreg]
        graph[vreg] = result = (set(), set())
        return result

    active = set()
    for cell in reversed(block):
        cell.active = active = (active ^ cell.defs) | cell.uses
        for vreg in active:
            get(vreg)[0].update(active ^ {vreg})

        if isinstance(cell, Motion):
            get(cell.src)[1].add(cell.dst)
            get(cell.dst)[1].add(cell.src)
    # Going through steps in chaitin-briggs algorithm
    stack = []
    # First simplification...
    while len(graph) > 0:
        for vreg, (interfere, coalesce) in graph.items():
            if len(interfere) < len(registers):
                for other in interfere:
                    graph[other][0].discard(vreg)
                stack.append((vreg, graph.pop(vreg)))
                break
        else:
            # The code compiled doesn't cause this situation yet.
            assert False, "XXX: the next part of coloring"
    # Then an attempt to color, no failure recovery yet.
    while len(stack) > 0:
        vreg, (interfere, coalesce) = stack.pop()
        filled = set(colors[v] for v in interfere if v in colors)
        avail = registers ^ filled
        colors[vreg] = avail.pop()
    return colors