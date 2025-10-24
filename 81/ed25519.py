import hashlib
from typing import List

b: int = 256
q: int = 2**255 - 19
l: int = 2**252 + 27742317777372353535851937790883648493


def H(m: bytes) -> bytes:
    return hashlib.sha512(m).digest()


def expmod(b: int, e: int, m: int) -> int:
    if e == 0:
        return 1
    t = expmod(b, e // 2, m) ** 2 % m
    _ = e & 1 and (t := (t * b) % m)
    return t


def inv(x: int) -> int:
    return expmod(x, q - 2, q)


d: int = -121665 * inv(121666)
I: int = expmod(2, (q - 1) // 4, q)


def xrecover(y: int) -> int:
    xx = (y * y - 1) * inv(d * y * y + 1)
    x = expmod(xx, (q + 3) // 8, q)
    _ = (x * x - xx) % q != 0 and (x := (x * I) % q)
    _ = x % 2 != 0 and (x := q - x)
    return x


By: int = 4 * inv(5)
Bx: int = xrecover(By)
B: List[int] = [Bx % q, By % q]


def edwards(P: List[int], Q: List[int]) -> List[int]:
    x1 = P[0]
    y1 = P[1]
    x2 = Q[0]
    y2 = Q[1]
    x3 = (x1 * y2 + x2 * y1) * inv(1 + d * x1 * x2 * y1 * y2)
    y3 = (y1 * y2 + x1 * x2) * inv(1 - d * x1 * x2 * y1 * y2)
    return [x3 % q, y3 % q]


def scalarmult(P: List[int], e: int) -> List[int]:
    if e == 0:
        return [0, 1]
    Q = scalarmult(P, e // 2)
    Q = edwards(Q, Q)
    _ = e & 1 and (Q := edwards(Q, P))
    return Q


def encodeint(y: int) -> bytes:
    bits = [(y >> i) & 1 for i in range(b)]
    return bytes([sum([bits[i * 8 + j] << j for j in range(8)]) for i in range(b // 8)])


def encodepoint(P: List[int]) -> bytes:
    x = P[0]
    y = P[1]
    bits = [(y >> i) & 1 for i in range(b - 1)] + [x & 1]
    return bytes([sum([bits[i * 8 + j] << j for j in range(8)]) for i in range(b // 8)])


def bit(h: bytes, i: int) -> int:
    return (h[i // 8] >> (i % 8)) & 1


def publickey(sk: bytes) -> bytes:
    h = H(sk)  # h is bytes
    a = 2 ** (b - 2) + sum(2**i * bit(h, i) for i in range(3, b - 2))
    A = scalarmult(B, a)
    return encodepoint(A)


def Hint(m: bytes) -> int:
    h = H(m)  # h is bytes
    return sum(2**i * bit(h, i) for i in range(2 * b))


def signature(m: bytes, sk: bytes, pk: bytes) -> bytes:
    h = H(sk)  # h is bytes
    a = 2 ** (b - 2) + sum(2**i * bit(h, i) for i in range(3, b - 2))
    r = Hint(h[b // 8 : b // 4] + m)
    R = scalarmult(B, r)
    h_sig = Hint(encodepoint(R) + pk + m)
    S = (r + h_sig * a) % l
    return encodepoint(R) + encodeint(S)


def isoncurve(P: List[int]) -> bool:
    x = P[0]
    y = P[1]
    return (-x * x + y * y - 1 - d * x * x * y * y) % q == 0


def decodeint(s: bytes) -> int:
    return sum(2**i * bit(s, i) for i in range(0, b))


def decodepoint(s: bytes) -> List[int]:
    y = sum(2**i * bit(s, i) for i in range(0, b - 1))
    x = xrecover(y)
    x & 1 != bit(s, b - 1) and (x := q - x)
    P = [x, y]
    if not isoncurve(P):
        raise Exception("decoding point that is not on curve")
    return P


def checkvalid(s: bytes, m: bytes, pk: bytes) -> bool:
    if len(s) != b // 4:
        raise Exception("signature length is wrong")
    if len(pk) != b // 8:
        raise Exception("public-key length is wrong")

    R = decodepoint(s[0 : b // 8])
    A = decodepoint(pk)
    S = decodeint(s[b // 8 : b // 4])
    h = Hint(encodepoint(R) + pk + m)
    return scalarmult(B, S) == edwards(R, scalarmult(A, h))