#!/usr/bin/env python3
"""Summarize and validate the Natcore system scorecards in docs/scorecards/."""
from __future__ import annotations

import argparse
import json
import re
import sys
import unicodedata
from dataclasses import dataclass, field
from pathlib import Path

DOCS = Path(__file__).resolve().parents[1]
REPO = DOCS.parent
SCORECARDS = DOCS / 'scorecards'
SPEC = DOCS / 'spec'

STATUSES = ['Implemented', 'In progress', 'Approved', 'Open question', 'Proposed']
FIELDS = {'Status', 'Spec', 'Done when', 'Evidence', 'Questions', 'Notes'}
HEADING = re.compile(r'^### (\S+) — (.+)$')
FIELD = re.compile(r'^- \*\*([^*]+):\*\*\s*(.*)$')
QUESTION = re.compile(r'^\s+\d+\.\s+\S')
LINK = re.compile(r'\[[^\]]*\]\(([^)]+)\)')
EVIDENCE = re.compile(r'`([^`]+)`')


@dataclass
class Item:
    id: str
    title: str
    file: Path
    line: int
    fields: dict[str, str] = field(default_factory=dict)
    questions: int = 0


@dataclass
class Scorecard:
    path: Path
    prefix: str | None = None
    items: list[Item] = field(default_factory=list)

    @property
    def name(self) -> str:
        return self.path.stem


def parse(path: Path, errors: list[str]) -> Scorecard:
    card = Scorecard(path)
    current: Item | None = None
    in_questions = False
    in_fence = False
    for number, line in enumerate(path.read_text(encoding='utf-8').splitlines(), 1):
        where = f'{path.relative_to(REPO)}:{number}'
        if line.startswith('```'):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        if line.startswith('#'):
            in_questions = False
            match = HEADING.match(line)
            current = Item(match.group(1), match.group(2), path, number) if match else None
            if current:
                card.items.append(current)
            elif line.startswith('### '):
                errors.append(f'{where}: item heading must be "### <ID> — <title>"')
            continue
        field_match = FIELD.match(line)
        if field_match:
            name, value = field_match.groups()
            if current is None:
                if name == 'Prefix':
                    card.prefix = value.strip()
                continue
            if name not in FIELDS:
                errors.append(f'{where}: unknown field "{name}" in {current.id}')
            elif name in current.fields:
                errors.append(f'{where}: duplicate field "{name}" in {current.id}')
            current.fields[name] = value.strip()
            in_questions = name == 'Questions'
            continue
        if current and in_questions and QUESTION.match(line):
            current.questions += 1
    if card.prefix is None:
        errors.append(f'{path.relative_to(REPO)}: missing "- **Prefix:**" line')
    return card


def slug(heading: str) -> str:
    text = heading.strip().lower()
    text = ''.join(c for c in text if c in ' -_' or unicodedata.category(c)[0] in 'LN')
    return text.replace(' ', '-')


def anchors(path: Path) -> set[str]:
    seen: dict[str, int] = {}
    result = set()
    in_fence = False
    for line in path.read_text(encoding='utf-8').splitlines():
        if line.startswith('```'):
            in_fence = not in_fence
            continue
        if not in_fence and (match := re.match(r'^#{1,6}\s+(.*)$', line)):
            base = slug(match.group(1))
            count = seen.get(base, 0)
            seen[base] = count + 1
            result.add(base if count == 0 else f'{base}-{count}')
    return result


def validate(cards: list[Scorecard], errors: list[str]) -> None:
    all_ids: dict[str, Item] = {}
    for card in cards:
        for item in card.items:
            where = f'{item.file.relative_to(REPO)}:{item.line}'
            if item.id in all_ids:
                errors.append(f'{where}: duplicate ID {item.id}')
            all_ids[item.id] = item
            if card.prefix and not re.fullmatch(rf'{re.escape(card.prefix)}-[A-Z]+-\d{{2}}', item.id):
                errors.append(f'{where}: ID {item.id} must look like {card.prefix}-<AREA>-<NN>')
            status = item.fields.get('Status')
            if status not in STATUSES:
                errors.append(f'{where}: {item.id} has invalid status {status!r}')
                continue
            spec = item.fields.get('Spec')
            if not spec:
                errors.append(f'{where}: {item.id} is missing Spec (use "None" if there is no spec section)')
            elif spec != 'None':
                links = LINK.findall(spec)
                if not links:
                    errors.append(f'{where}: {item.id} Spec must be a Markdown link or "None"')
                for target in links:
                    check_link(item, target, errors)
            if status != 'Open question' and not item.fields.get('Done when'):
                errors.append(f'{where}: {item.id} ({status}) is missing "Done when"')
            if status == 'Open question':
                if item.questions == 0:
                    errors.append(f'{where}: {item.id} is an Open question but lists no numbered Questions')
            elif 'Questions' in item.fields:
                errors.append(f'{where}: {item.id} has Questions but is {status}; only Open question items list questions')
            if status == 'Implemented':
                evidence = EVIDENCE.findall(item.fields.get('Evidence', ''))
                if not evidence:
                    errors.append(f'{where}: {item.id} is Implemented but has no backticked Evidence')
                for reference in evidence:
                    check_evidence(item, reference, errors)
            elif 'Evidence' in item.fields:
                errors.append(f'{where}: {item.id} has Evidence but is {status}; only Implemented items record evidence')
    check_references(cards, set(all_ids), errors)


def check_link(item: Item, target: str, errors: list[str]) -> None:
    where = f'{item.file.relative_to(REPO)}:{item.line}'
    path_part, _, anchor = target.partition('#')
    resolved = (item.file.parent / path_part).resolve()
    if not resolved.exists():
        errors.append(f'{where}: {item.id} links to missing file {path_part}')
    elif anchor and anchor not in anchors(resolved):
        errors.append(f'{where}: {item.id} links to missing heading #{anchor} in {path_part}')


def check_evidence(item: Item, reference: str, errors: list[str]) -> None:
    where = f'{item.file.relative_to(REPO)}:{item.line}'
    path_part, _, symbol = reference.partition('::')
    test_name = symbol.rsplit('::', 1)[-1]
    file = REPO / path_part
    if not symbol or not file.is_file():
        errors.append(f'{where}: {item.id} evidence {reference!r} must be "<existing file>::<test>"')
    elif not re.search(rf'\bfn {re.escape(test_name)}\s*\(', file.read_text(encoding='utf-8')):
        errors.append(f'{where}: {item.id} evidence names missing test {test_name!r} in {path_part}')


def check_references(cards: list[Scorecard], ids: set[str], errors: list[str]) -> None:
    prefixes = '|'.join(sorted({c.prefix for c in cards if c.prefix}, key=len, reverse=True))
    pattern = re.compile(rf'\b(?:{prefixes})-[A-Z]+-\d{{2}}\b')
    sources = list(SPEC.glob('*.md')) + [c.path for c in cards]
    for path in sources:
        for number, line in enumerate(path.read_text(encoding='utf-8').splitlines(), 1):
            for reference in pattern.findall(line):
                if reference not in ids:
                    errors.append(f'{path.relative_to(REPO)}:{number}: reference to unknown item {reference}')


def summarize(cards: list[Scorecard]) -> dict:
    systems = {}
    for card in cards:
        counts = {status: 0 for status in STATUSES}
        for item in card.items:
            if item.fields.get('Status') in counts:
                counts[item.fields['Status']] += 1
        systems[card.name] = {'prefix': card.prefix, 'total': len(card.items), **counts}
    totals = {status: sum(s[status] for s in systems.values()) for status in STATUSES}
    totals['total'] = sum(s['total'] for s in systems.values())
    return {'systems': systems, 'total': totals}


def print_table(report: dict) -> None:
    columns = ['Implemented', 'In progress', 'Approved', 'Open question', 'Proposed', 'total']
    header = f'{"System":<16}' + ''.join(f'{c.title() if c == "total" else c:>15}' for c in columns)
    print(header)
    print('-' * len(header))
    rows = list(report['systems'].items()) + [('TOTAL', report['total'])]
    for name, counts in rows:
        print(f'{name:<16}' + ''.join(f'{counts[c]:>15}' for c in columns))


def main() -> int:
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument('--json', action='store_true', help='print machine-readable counts')
    cli.add_argument('--check', action='store_true', help='validate scorecards, links, evidence, and the merged spec')
    args = cli.parse_args()

    errors: list[str] = []
    cards = [parse(path, errors) for path in sorted(SCORECARDS.glob('*.md')) if path.name != 'README.md']
    if args.check:
        validate(cards, errors)
        sys.path.insert(0, str(Path(__file__).parent))
        from build_spec import OUTPUT, build
        if not OUTPUT.exists() or OUTPUT.read_text(encoding='utf-8') != build():
            errors.append('docs/natcore-spec.md is out of date; run python docs/tools/build_spec.py')

    report = summarize(cards)
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print_table(report)
    if errors:
        print(f'\n{len(errors)} scorecard problem(s):', file=sys.stderr)
        for error in errors:
            print(f'  {error}', file=sys.stderr)
        return 1
    if args.check:
        print('\nScorecards are valid.')
    return 0


if __name__ == '__main__':
    sys.exit(main())
