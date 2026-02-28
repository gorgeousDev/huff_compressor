import struct

with open('example.huff', 'rb') as f:
    data = f.read()

# Read the single header
tree_size = struct.unpack('<I', data[0:4])[0]
print(f"Tree size from header: {tree_size} bytes")

# Extract the tree data
tree_data = data[4:4+tree_size]
print(f"Actual tree data length: {len(tree_data)} bytes")

if tree_size == len(tree_data):
    print("✅ Header matches tree data length!")
else:
    print(f"❌ Mismatch: header says {tree_size}, actual is {len(tree_data)}")

# Now parse the tree data
i = 0
while i < len(tree_data):
    marker = tree_data[i]
    if marker == 1:  # Leaf node
        char_bytes = tree_data[i+1:i+5]
        # Try to decode the character
        try:
            char = char_bytes.decode('utf-8')
            print(f"Leaf: '{char}' (0x{char_bytes.hex()})")
        except:
            print(f"Leaf: raw bytes {char_bytes.hex()}")
        i += 5
    elif marker == 0:  # Internal node
        print(f"Internal node at position {i}")
        i += 1
    else:
        print(f"Unexpected marker {marker} at {i}")
        break

# The rest should be your compressed data
compressed_data = data[4+tree_size:]
print(f"\nCompressed data size: {len(compressed_data)} bytes")