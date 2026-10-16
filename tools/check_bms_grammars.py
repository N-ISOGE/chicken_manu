"""Recognize synthetic inputs with the published ABNF and W3C EBNF drafts.

Requires abnf==2.9.0 and lark==1.2.2. No game or third-party chart is run.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

from abnf import ParseError, Rule
from lark import Lark, UnexpectedInput


def w3c_to_lark(source: str) -> str:
    """Translate this file's W3C notation without replacing its productions."""
    source = re.sub(r"\(\*.*?\*\)", "", source, flags=re.S)
    definitions = list(re.finditer(r"(?m)^([A-Za-z_][A-Za-z_0-9]*)\s*::=", source))
    names = [match[1] for match in definitions]
    if len(names) != len(set(names)):
        raise ValueError("Duplicate EBNF rule")
    tokens = re.compile(
        r"\s+|'[^']*'|\"[^\"]*\"|\[[^\]]*\]|#x[0-9A-Fa-f]+"
        r"|[A-Za-z_][A-Za-z_0-9]*|[()|*+?]"
    )
    output = []
    for i, match in enumerate(definitions):
        end = definitions[i + 1].start() if i + 1 < len(definitions) else len(source)
        rhs = source[match.end() : end]
        translated = []
        cursor = 0
        for token in tokens.finditer(rhs):
            if rhs[cursor : token.start()].strip():
                raise ValueError(f"Unsupported EBNF text: {rhs[cursor:token.start()]!r}")
            value = token[0]
            cursor = token.end()
            if value.isspace():
                continue
            if value[0] in "'\"":
                translated.append(json.dumps(value[1:-1], ensure_ascii=False))
            elif value.startswith("["):
                charset = re.sub(
                    r"#x([0-9A-Fa-f]+)",
                    lambda m: "\\U" + f"{int(m[1], 16):08x}",
                    value,
                )
                translated.append("/" + charset + "/")
            elif value.startswith("#x"):
                translated.append(json.dumps(chr(int(value[2:], 16))))
            elif value in names:
                translated.append(value.lower())
            elif value in "()|*+?":
                translated.append(value)
            else:
                raise ValueError(f"Undefined EBNF rule: {value}")
        if rhs[cursor:].strip():
            raise ValueError(f"Unsupported EBNF suffix: {rhs[cursor:]!r}")
        output.append(match[1].lower() + ": " + " ".join(translated))
    if not definitions:
        raise ValueError("No EBNF rules")
    return "\n".join(output)


def main() -> int:
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    cli.add_argument("--ebnf-file", type=Path, help="Override EBNF for regression comparison")
    args = cli.parse_args()
    survey = args.root / "docs" / "survey"

    class BmsRule(Rule):
        pass

    BmsRule.load_grammar((survey / "bms.abnf").read_text(encoding="utf-8-sig"))
    ebnf_path = args.ebnf_file or survey / "bms.ebnf"
    ebnf = Lark(
        w3c_to_lark(ebnf_path.read_text(encoding="utf-8-sig")),
        parser="earley",
        lexer="dynamic_complete",
        start="bms_file",
    )

    def abnf_accept(rule: str, value: str) -> bool:
        try:
            BmsRule(rule).parse_all(value)
            return True
        except ParseError:
            return False

    def ebnf_accept(value: str) -> bool:
        # The EBNF draft describes terminated lines; preserve source bytes elsewhere.
        if value and not value.endswith(("\r", "\n")):
            value += "\n"
        try:
            ebnf.parse(value)
            return True
        except UnexpectedInput:
            return False

    cases = []
    for value, expected in [
        ("#00102:.25", True),
        ("#00102:1.5", True),
        ("#00102:ZZ", False),
        ("#00103:FF", True),
        ("#00103:GG", False),
        ("#00103:1.5", False),
        ("#00111:01ZZ", True),
        ("#00111:1.5", False),
        ("#00111:0", False),
        ("#00111:", True),
        ("#00103:", True),
        ("#WAV01 sound file.wav", True),
        ("#WAV01 음원.wav", True),
        ("#TITLE Synthetic", True),
        ("#TITLE", False),  # EBNF requires a separator; ABNF allows an empty text value.
    ]:
        if value != "#TITLE":
            cases.append(("ABNF strict", value, expected, abnf_accept("known-command-line", value)))
        cases.append(("EBNF", value, expected, ebnf_accept(value)))

    for rule, value, expected in [
        ("source-line", "#UNKNOWN value", True),
        ("known-command-line", "#UNKNOWN value", False),
        ("source-line", "#TOTAL invalid", True),
        ("known-command-line", "#TOTAL invalid", False),
        ("known-command-line", "#title lower", True),
        ("structure", "RANDOM IF L ENDIF ", True),
        ("structure", "IF IF L ENDIF ENDIF ", True),
        ("structure", "IF L ", False),
        ("structure", "ELSE L ", False),
        ("structure", "IF ELSE ELSE ENDIF ", False),
        ("structure", "SWITCH CASE L SKIP DEF L ENDSW ", True),
        ("structure", "SWITCH CASE L ", False),
    ]:
        cases.append(("ABNF " + rule, value, expected, abnf_accept(rule, value)))

    for value, expected in [
        ("", True),
        ("#TITLE LF\n#ARTIST Text\n", True),
        ("#TITLE CRLF\r\n#ARTIST Text\r\n", True),
        ("#TITLE CR\r#ARTIST Text\r", True),
        ("#TITLE Mix\r\n#ARTIST Text\r#00111:01\n", True),
        ("#title lower", False),
        ("#TOTAL invalid", False),
        ("#UNKNOWN value", False),
        ("  #00102:ZZ", False),
        ("#00111:01#00111:02", False),
        ("#RANDOM 2\n#IF 1\n#TITLE A\n#ENDIF\n", True),
        ("#RANDOM 2\n#IF 1\n#TITLE A\n#ENDIF\n#ENDRANDOM\n", True),
        ("#IF 1\n#TITLE A\n#ELSE\n#TITLE B\n#ENDIF\n", True),
        ("#IF 1\n#TITLE A\n", False),
        ("#ELSE\n#TITLE A\n", False),
        ("// Synthetic comment\n#TITLE A\n", True),
        ("#SWITCH 2\n#CASE 1\n#TITLE A\n#SKIP\n#DEF\n#TITLE B\n#ENDSW\n", True),
        ("#SWITCH 2\n#CASE 1\n#TITLE A\n", False),
    ]:
        cases.append(("EBNF", value, expected, ebnf_accept(value)))

    failures = [case for case in cases if case[2] != case[3]]
    for kind, value, expected, actual in failures:
        print(f"FAIL {kind}: {value!r}; expected={expected}, actual={actual}")
    print(f"{len(cases) - len(failures)}/{len(cases)} synthetic recognition checks passed")
    return bool(failures)


if __name__ == "__main__":
    raise SystemExit(main())
