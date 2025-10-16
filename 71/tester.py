DEBUG = 0
CMD = "cargo run --"

import subprocess, os, random
from operator import add, sub, mul, floordiv as quo, mod as rem

bigone, bigtwo = random.randint(2 ** 500, 2 ** 512), random.randint(2 ** 500, 2 ** 512)
#bigone = 5
#bigtwo = 3
hexone, hextwo = hex(bigone), hex(bigtwo)
print(f"\nhexone: {bigone} = {hexone}", f"\nhextwo: {bigtwo} = {hextwo}")

from operator import add, sub, mul, floordiv as quo, mod as rem
#ops = {'ADD':add,'SUB':sub,'MUL':mul,'QUO':quo,'REM':rem}
ops = {"ADD":add,"SUB":sub}
for op in ops:
    result = int(subprocess.check_output(["cargo", "run", hexone, hextwo, op]),16)
    answer = ops[op](bigone,bigtwo)
    if result != answer:
        print("Operator", op, "failed.")
        print("Expected:")
        print(hex(answer))
        print("Received:")
        print(hex(result))
        exit()
    else:
        print(op, "passes:", hex(result))