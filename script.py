# Generates assets/test.nwbook for development and testing

from pathlib import Path
import struct

title = "Alice in Wonderland"

pages = [
    """Alice was beginning to get very tired of
sitting by her sister on the bank, and of
having nothing to do.""",

    """There was nothing so very remarkable in
that; nor did Alice think it so very much
out of the way.""",

    """The rabbit actually took a watch out of
its waistcoat-pocket, and looked at it.""",
]

encoded_pages = [page.encode("utf-8") for page in pages]

offsets = [0]
for page in encoded_pages:
    offsets.append(offsets[-1] + len(page))

data = bytearray()

data += b"NWBK"
data += struct.pack("<H", 1)
data += struct.pack("<I", len(pages))

title_bytes = title.encode("utf-8")
data += struct.pack("<I", len(title_bytes))
data += title_bytes

for offset in offsets:
    data += struct.pack("<I", offset)

for page in encoded_pages:
    data += page

Path("assets/test.nwbook").write_bytes(data)