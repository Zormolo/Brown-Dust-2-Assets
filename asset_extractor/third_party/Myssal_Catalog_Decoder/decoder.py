import json
import os
import base64
import string
import re
from typing import List, Any


ENABLE_BUNDLE_FILTER = True 
MIN_PRINTABLE_LENGTH = 4

BUNDLE_PATTERN = re.compile(r'^.+_[0-9a-fA-F]{32}\.bundle$')


def safe_base64_decode(data: str) -> bytes:
    if not data:
        return b""

    data = data.strip()
    padding = len(data) % 4
    if padding:
        data += "=" * (4 - padding)

    try:
        return base64.b64decode(data, validate=False)
    except Exception:
        return b""


def extract_printable_strings(data: bytes, min_length: int) -> List[str]:
    printable = set(string.printable)
    current = []
    results = []

    for b in data:
        ch = chr(b)
        if ch in printable and ch not in "\r\n\t\x0b\x0c":
            current.append(ch)
        else:
            if len(current) >= min_length:
                results.append("".join(current))
            current.clear()

    if len(current) >= min_length:
        results.append("".join(current))

    return results


def is_bundle_name(text: str) -> bool:
    return bool(BUNDLE_PATTERN.match(text))


def find_and_decode_keys(token: Any, result: List[str]) -> None:
    if isinstance(token, dict):
        for key, value in token.items():
            if key == "m_KeyDataString":
                decoded = safe_base64_decode(str(value))
                strings = extract_printable_strings(decoded, MIN_PRINTABLE_LENGTH)

                for s in strings:
                    if not ENABLE_BUNDLE_FILTER or is_bundle_name(s):
                        result.append( s.replace( ".bundle", "" ) )

            find_and_decode_keys(value, result)

    elif isinstance(token, list):
        for item in token:
            find_and_decode_keys(item, result)


def decode_catalog_key_data_strings(
    catalog_path: str
) -> List[str]:
    global ENABLE_BUNDLE_FILTER

    if not os.path.isfile(catalog_path):
        raise FileNotFoundError(f"Catalog file not found: {catalog_path}")

    with open(catalog_path, "r", encoding="utf-8") as f:
        root = json.load(f)

    decoded_strings: List[str] = []
    find_and_decode_keys(root, decoded_strings)

    mode = "with bundle filter" if ENABLE_BUNDLE_FILTER else "without filter"
    print(f"Decoded {len(decoded_strings)} entries ({mode})")

    return decoded_strings