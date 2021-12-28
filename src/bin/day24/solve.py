import argparse
from typing import Tuple, List, Union

import angr
import archinfo
import claripy
from keystone import Ks, KS_ARCH_ARM64, KS_MODE_LITTLE_ENDIAN

REG = {
    'w': 'x2',
    'x': 'x3',
    'y': 'x4',
    'z': 'x5'
}


def convert_literal(s: str) -> Union[str, int]:
    try:
        return int(s)
    except ValueError:
        return s


def parse(s: str):
    instrs = []
    for l in s.splitlines():
        inst, *args = l.strip().split(' ')
        if not inst:
            break

        if inst == 'inp':
            instrs.append((inst, (args[0],)))
        else:
            instrs.append((inst, (convert_literal(args[0]), convert_literal(args[1]))))
    return instrs


def arm_code(prog: List[Tuple]) -> bytes:
    ks = Ks(KS_ARCH_ARM64, KS_MODE_LITTLE_ENDIAN)
    code = []

    for inst, args in prog:
        if inst == 'inp':
            code.extend(ks.asm('ldrb {}, [x0], 1'.format(REG[args[0]].replace('x', 'w')))[0])
        elif inst == 'add':
            code.extend(ks.asm('add {}, {}, {}'.format(REG[args[0]], REG[args[0]], REG.get(args[1], args[1])))[0])
        elif inst == 'mul':
            asm = '\n'.join([
                'mov x13, {}'.format(REG.get(args[1], args[1])),
                'mul {}, {}, x13'.format(REG[args[0]], REG[args[0]]),
            ])
            code.extend(ks.asm(asm)[0])
        elif inst == 'div':
            asm = '\n'.join([
                'mov x14, {}'.format(REG.get(args[1], args[1])),
                'cmp x14, 0',
                'b.ne cont',
                'mov x0, 0',
                'mov x1, 1',
                'ret',
                'cont: sdiv {}, {}, x14'.format(REG[args[0]], REG[args[0]])
            ])
            code.extend(ks.asm(asm)[0])
        elif inst == 'mod':
            asm = '\n'.join([
                'mov x13, {}'.format(REG[args[0]]),
                'mov x14, {}'.format(REG.get(args[1], args[1])),
                'cmp x13, 0',
                'b.ge afine',
                'mov x0, 0',
                'mov x1, 1',
                'ret',
                'afine: cmp x14, 0',
                'b.gt bfine',
                'mov x0, 0',
                'mov x1, 1',
                'ret',
                'bfine: sdiv x9, x13, x14',
                'msub {}, x9, x14, x13'.format(REG[args[0]]),
            ])
            code.extend(ks.asm(asm)[0])
        elif inst == 'eql':
            asm = '\n'.join([
                'cmp {}, {}'.format(REG[args[0]], REG.get(args[1], args[1])),
                'mov {}, 1'.format(REG[args[0]]),
                'b.eq equal',
                'mov {}, 0'.format(REG[args[0]]),
                'equal: nop'.format(REG[args[0]], REG[args[0]]),
            ])
            code.extend(ks.asm(asm)[0])
        else:
            assert False, "Unrecognized instruction"

    code.extend(ks.asm('mov x0, {}; mov x1, 0; ret'.format(REG['z']))[0])

    return bytes(code)


def main():
    parser = argparse.ArgumentParser(description='Solution for day 23 of aoc2021')
    parser.add_argument('input_file')

    args = parser.parse_args()

    with open(args.input_file, 'r', encoding='utf8') as f:
        prog = parse(f.read())

    code = arm_code(prog)
    with open('out.bin', 'wb') as f:
        f.write(code)

    proj = angr.project.load_shellcode(code, archinfo.ArchAArch64(), load_address=0x10_000)
    state = proj.factory.call_state(0x10_000)
    serial = claripy.BVS('serial', 14*8, explicit_name=True)
    sp = state.regs.sp
    sp -= 0x20
    state.memory.store(sp, serial)
    state.regs.x0 = sp
    sp &= (~0xf)
    state.regs.sp = sp
    state.regs.x2 = 0
    state.regs.x3 = 0
    state.regs.x4 = 0
    state.regs.x5 = 0
    for b in serial.chop(8):
        state.solver.add(claripy.And(b > 0, b < 10))

    simgr = proj.factory.simgr(state)
    simgr.run()

    part1 = 0
    for state in simgr.deadended:
        try:
            part1 = max(part1, state.solver.max(serial, extra_constraints=(state.regs.x0 == 0, state.regs.x1 == 0)))
        except angr.SimUnsatError:
            pass
    part1 = bytes(b + 0x30 for b in part1.to_bytes(14, byteorder='big'))
    print('Part 1:', part1.decode('utf8'))

    part2 = float('inf')
    for state in simgr.deadended:
        try:
            part2 = min(part2, state.solver.min(serial, extra_constraints=(state.regs.x0 == 0, state.regs.x1 == 0)))
        except angr.SimUnsatError:
            pass
    part2 = bytes(b + 0x30 for b in part2.to_bytes(14, byteorder='big'))

    print('Part 2:', part2.decode('utf8'))


if __name__ == '__main__':
    main()
