import argparse
import unicorn.arm64_const
from unicorn import Uc, UC_ARCH_ARM64, UC_MODE_LITTLE_ENDIAN
from keystone import Ks, KS_ARCH_ARM64, KS_MODE_LITTLE_ENDIAN


def round_to_pgsz(n):
    return (n + 0xfff) & (~0xfff)


def write_prog(asm):
    with open('out.bin', 'wb') as f:
        f.write(bytes(Ks(KS_ARCH_ARM64, KS_MODE_LITTLE_ENDIAN).asm(asm)[0]))

def test(asm):
    inst_bytes, inst_count = Ks(KS_ARCH_ARM64, KS_MODE_LITTLE_ENDIAN).asm(asm)
    uc = Uc(UC_ARCH_ARM64, UC_MODE_LITTLE_ENDIAN)

    uc.mem_map(0, round_to_pgsz(len(inst_bytes)))
    uc.mem_write(0, bytes(inst_bytes))
    uc.reg_write(unicorn.arm64_const.UC_ARM64_REG_LR, 0x42424242)
    uc.mem_map(0x42424242 & (~0xfff), 4096)

    uc.emu_start(0, 0x42424242)
    return {f'x{i}': uc.reg_read(getattr(unicorn.arm64_const, f'UC_ARM64_REG_X{i}')) & 0xffffffff
            for i in range(31)}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Test an input')
    parser.add_argument('prog')
    parser.add_argument('input')

    args = parser.parse_args()

    input_ = args.input.encode('utf8')
    print('input:', input_)

    import solve
    with open(args.prog, 'r') as f:
        code = solve.arm_code(solve.parse(f.read()))

    uc = Uc(UC_ARCH_ARM64, UC_MODE_LITTLE_ENDIAN)

    uc.mem_map(0x10_000, round_to_pgsz(len(code)))
    uc.mem_write(0x10_000, code)
    uc.reg_write(unicorn.arm64_const.UC_ARM64_REG_LR, 0x42424242)
    uc.mem_map(0x42424242 & (~0xfff), 4096)
    uc.mem_map(0x20_000, 4096)
    uc.mem_write(0x20_000, bytes(b - 0x30 for b in input_))
    uc.reg_write(unicorn.arm64_const.UC_ARM64_REG_X0, 0x20_000)

    uc.emu_start(0x10_000, 0x42424242)
    results =  {f'x{i}': uc.reg_read(getattr(unicorn.arm64_const, f'UC_ARM64_REG_X{i}')) & 0xffffffff
                for i in range(31)}
    import pprint

    pprint.pprint(results)
