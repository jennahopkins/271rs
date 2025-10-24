import binascii

# fields on each input line: sk, pk, m, sm
# each field hex
# each field colon-terminated
# sk includes pk at end
# sm includes m at end
x = line.split(":")
sk = binascii.unhexlify(x[0][0:64])
m = binascii.unhexlify(x[2])
pk = publickey(sk)
s = signature(m, sk, pk)

assert checkvalid(s, m, pk)
assert bytes(x[0].encode("latin-1")) == binascii.hexlify(sk + pk)
assert bytes(x[1].encode("latin-1")) == binascii.hexlify(pk)
assert bytes(x[3].encode("latin-1")) == binascii.hexlify(s + m)

# sign.input excerpt
line = "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c:3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c:72:92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c0072:"