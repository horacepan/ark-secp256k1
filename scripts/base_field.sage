modulus = 115792089237316195423570985008687907853269984665640564039457584007908834671663
print(modulus.is_prime())
assert(modulus.is_prime())

Fp = GF(modulus)

generator = Fp(0);
for i in range(0, 20):
    i = Fp(i);
    neg_i = Fp(-i)
    if not(i.is_primitive_root() or neg_i.is_primitive_root()):
        continue
    elif i.is_primitive_root():
        assert(i.is_primitive_root());
        print("Generator: %d" % i)
        generator = i
        break
    else:
        assert(neg_i.is_primitive_root());
        print("Generator: %d" % neg_i)
        generator = neg_i
        break
