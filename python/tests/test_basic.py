import os

import pytest

import content_inspector as ci

TESTDATA = os.path.join(os.path.dirname(__file__), "..", "..", "testdata")


def test_inspect_bytes():
    assert ci.inspect(b"hello world\n").is_text()
    assert ci.inspect(b"hello world\n").name == "UTF_8"
    assert ci.inspect(b"\x00\x01\x02\x03").is_binary()
    assert ci.inspect(b"").name == "UTF_8"


def test_inspect_path(tmp_path):
    text = tmp_path / "a.txt"
    text.write_text("hello world\n")
    binary = tmp_path / "a.bin"
    binary.write_bytes(b"\x00\x01\x02\x03" * 10)
    for p in (text, str(text)):  # Path and str
        assert ci.inspect(p).is_text()
    assert ci.inspect(binary).is_binary()


def test_content_type_object():
    t = ci.inspect(b"hello")
    assert isinstance(t, ci.ContentType)
    assert t.is_text() and not t.is_binary()
    assert t.name == "UTF_8"
    assert str(t) == "UTF-8"
    assert repr(t) == "ContentType.UTF_8"
    assert t == ci.inspect("é".encode())
    assert t != ci.inspect(b"\x00")
    assert len({t, ci.inspect(b"abc"), ci.inspect(b"\x00")}) == 2


@pytest.mark.parametrize("wrap", [bytes, bytearray, memoryview])
def test_accepts_bytes_like_objects(wrap):
    assert ci.inspect(wrap(b"hello")).name == "UTF_8"
    assert ci.inspect(wrap(b"\x00\x01")).is_binary()


def test_rejects_unsupported_input(tmp_path):
    with pytest.raises(TypeError):
        ci.inspect(123)
    with pytest.raises(FileNotFoundError):
        ci.inspect("hello")  # a str is a path, not text content
    with pytest.raises(FileNotFoundError):
        ci.inspect(tmp_path / "nope")


def test_only_first_1024_bytes_are_examined(tmp_path):
    assert ci.inspect(b"a" * 1023 + b"\x00").is_binary()
    assert ci.inspect(b"a" * 1024 + b"\x00").is_text()
    p = tmp_path / "f"
    p.write_bytes(b"a" * 1024 + b"\x00")
    assert ci.inspect(p) == ci.inspect(b"a" * 1024 + b"\x00")


@pytest.mark.skipif(not os.path.isdir(TESTDATA), reason="testdata/ not available")
@pytest.mark.parametrize(
    "name, expected",
    [
        ("text_UTF-8.txt", "UTF_8"),
        ("text_UTF-8-BOM.txt", "UTF_8_BOM"),
        ("text_UTF-16LE-BOM.txt", "UTF_16LE"),
        ("text_UTF-16BE-BOM.txt", "UTF_16BE"),
        ("text_UTF-32LE-BOM.txt", "UTF_32LE"),
        ("text_UTF-32BE-BOM.txt", "UTF_32BE"),
        ("test.png", "BINARY"),
        ("test.jpg", "BINARY"),
        ("test.pdf", "BINARY"),
    ],
)
def test_upstream_fixtures(name, expected):
    path = os.path.join(TESTDATA, name)
    assert ci.inspect(path).name == expected
    with open(path, "rb") as f:
        assert ci.inspect(f.read()).name == expected
