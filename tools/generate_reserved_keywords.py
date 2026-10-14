"""Generate the lexical keyword inventory from the pinned textual grammars."""
import argparse
from pathlib import Path
import re


def render(bnf_root: Path) -> str:
    files = [(language, filename, (bnf_root / filename).read_bytes()) for language, filename in
             [("sysml", "SysML-textual-bnf.kebnf"), ("kerml", "KerML-textual-bnf.kebnf")]]
    digest = 0xcbf29ce484222325
    for _, filename, content in files:
        for byte in filename.encode() + b"\0" + content + b"\0":
            digest = ((digest ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    pin = dict(line.split("=", 1) for line in Path("docs/conformance-target").read_text().splitlines()
               if line and not line.startswith("#"))
    actual = f"fnv1a64:{digest:016x}"
    if actual != pin["grammar_content_hash"]:
        raise ValueError(f"grammar contents do not match pin: {actual}")
    lines = ["# Generated from the pinned official RESERVED_KEYWORD productions.",
             f"grammar_content_hash={actual}"]
    for language, _, content in files:
        production = re.search(r"^RESERVED_KEYWORD\s*=\s*(.*?)(?=\n\s*\n)", content.decode(), re.M | re.S)
        if not production:
            raise ValueError(f"missing {language} RESERVED_KEYWORD production")
        words = re.findall(r"'([a-z]+)'", production.group(1))
        if not words or len(words) != len(set(words)):
            raise ValueError(f"empty or duplicate {language} reserved keywords")
        lines.append(language + "=" + " ".join(sorted(words)))
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bnf-root", type=Path, default=Path("sysml-v2-release/bnf"))
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    expected = render(args.bnf_root)
    target = Path("docs/conformance-keywords")
    if args.check:
        if target.read_text(encoding="utf-8") != expected:
            raise SystemExit("reserved-keyword inventory is stale")
    else:
        target.write_text(expected, encoding="utf-8", newline="\n")
