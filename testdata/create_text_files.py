import codecs

encodings = {
    "UTF-8": ("utf-8", codecs.BOM_UTF8),
    "UTF-16BE": ("utf_16_be", codecs.BOM_UTF16_BE),
    "UTF-16LE": ("utf_16_le", codecs.BOM_UTF16_LE),
    "UTF-32BE": ("utf_32_be", codecs.BOM_UTF32_BE),
    "UTF-32LE": ("utf_32_le", codecs.BOM_UTF32_LE),
}

with open("text_UTF-8.txt", "rb") as source:
    data = source.read()
    text = data.decode("utf-8")

    for name, (encoding, bom) in encodings.items():
        with open("text_{}-BOM.txt".format(name), "wb") as target:
            target.write(bom)
            target.write(text.encode(encoding))
