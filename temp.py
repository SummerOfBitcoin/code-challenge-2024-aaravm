import hashlib
from binascii import unhexlify

data = b"03b66c2dc49fede6cd394075916b6ec2a6cdf3bc3a64fd216d908bb2e13161c583"
data1 = unhexlify(data)

sha256_hash = hashlib.sha256(data1).hexdigest()
print(sha256_hash)

data2 = unhexlify(sha256_hash)
# data = hashlib.sha256(data2).hexdigest()
ripemd_hash = hashlib.new('ripemd160', data2).hexdigest()
print(ripemd_hash)