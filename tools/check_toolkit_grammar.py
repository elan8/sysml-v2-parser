"""Audit the Xtext productions that support the two Spec42 pilot cases."""

import argparse
import hashlib
from pathlib import Path
import re
from urllib.request import urlopen


REVISION = "7d740b37c0666be18bee2de1b467c86a234788bc"
SHA256 = "d6da61059827ac0f0953106125a2bae668cc4aa99ac5a9c373dc241e60d27554"
EXPECTED = {
    "OccurrenceDefinitionPrefix": "BasicDefinitionPrefix?",
    "UsagePrefix": "UnextendedUsagePrefix UsageExtensionKeyword*",
    "UnextendedUsagePrefix": "EndUsagePrefix | BasicUsagePrefix",
    "FeatureDeclaration": "Identification FeatureSpecializationPart? | FeatureSpecializationPart",
    "FeatureSpecialization": "Typings | Subsettings | References | Crosses | Redefinitions",
    "RedefinesKeyword": "':>>' | 'redefines'",
    "ItemDefinition": "OccurrenceDefinitionPrefix ItemDefKeyword Definition",
    "BasicDefinitionPrefix": "isAbstract ?= 'abstract'",
    "AttributeUsage": "UsagePrefix AttributeUsageKeyword Usage",
    "BasicUsagePrefix": "RefPrefix ( isReference ?= 'ref' )?",
}


def check_grammar(data):
    source = data.decode("utf-8")
    for rule, fragment in EXPECTED.items():
        production = re.search(
            rf"^(?:fragment )?{rule}(?:\s+returns\s+\S+)?\s*:\s*(.*?)^\s*;",
            source,
            re.MULTILINE | re.DOTALL,
        )
        if production is None or fragment not in " ".join(production[1].split()):
            raise ValueError(f"{rule} is missing its expected production fragment")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("path", type=Path)
    parser.add_argument("--fetch", action="store_true", help="download the pinned toolkit grammar")
    args = parser.parse_args()
    try:
        if args.fetch:
            url = (
                "https://raw.githubusercontent.com/Open-MBEE/sysml-toolkit/"
                f"{REVISION}/spec-refs/SysML.xtext"
            )
            with urlopen(url, timeout=30) as response:
                data = response.read()
        else:
            data = args.path.read_bytes()
        data = data.replace(b"\r\n", b"\n")
        if hashlib.sha256(data).hexdigest() != SHA256:
            raise ValueError("SHA-256 mismatch for SysML.xtext")
        check_grammar(data)
        if args.fetch:
            args.path.parent.mkdir(parents=True, exist_ok=True)
            args.path.write_bytes(data)
    except (OSError, ValueError) as error:
        parser.exit(1, f"Toolkit grammar reference check: {error}\n")
    print("Pinned toolkit grammar productions verified")


if __name__ == "__main__":
    main()
