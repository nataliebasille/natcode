#!/usr/bin/env python3
"""Summarize Natcore language, infrastructure, and backend conformance separately."""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BACKENDS = ('Interpreter', 'JavaScript', 'WebAssembly', 'C', 'Python')
ITEM = re.compile(r'^- \[([ xX])\] \*\*([A-Z][A-Z0-9-]*-\d+) — (.*?)\*\*')


def read_checklist(path):
    sections = {}
    seen = set()
    section = None
    for line in path.read_text(encoding='utf-8').splitlines():
        if line.startswith('### '):
            section = line[4:]
        match = ITEM.match(line)
        if not match:
            continue
        if not section:
            raise ValueError(f'{path}: checklist item before ### section')
        checked, id, name = match.groups()
        if id in seen:
            raise ValueError(f'{path}: duplicate ID {id}')
        seen.add(id)
        record = sections.setdefault(section, {'verified': 0, 'total': 0})
        record['verified'] += (checked.lower() == 'x')
        record['total'] += 1
    if not seen:
        raise ValueError(f'{path}: no checklist items found')
    return sections, seen


def totals(sections):
    return {'verified': sum(s['verified'] for s in sections.values()),
            'total': sum(s['total'] for s in sections.values())}


def read_backend(path, feature_ids):
    lines = path.read_text(encoding='utf-8').splitlines()
    seen = set()
    counts = {x: {'verified': 0, 'total': 0} for x in BACKENDS}
    for line in lines:
        cells = [v.strip() for v in re.split(r'(?<!\\)\|', line)]
        if len(cells) < 9 or not re.fullmatch(r'`[A-Z]+-\d+`', cells[1]):
            continue
        fid = cells[1].strip('`')
        if fid not in feature_ids:
            raise ValueError(f'Unknown backend feature ID: {fid}')
        if fid in seen:
            raise ValueError(f'Duplicate backend feature ID: {fid}')
        seen.add(fid)
        values = cells[3:8]
        for col, value in zip(BACKENDS, values):
            if value not in ('☐', '☑', '—'):
                raise ValueError(f'Invalid backend cell {fid}/{col}: {value!r}')
            if value != '—':
                counts[col]['total'] += 1
                counts[col]['verified'] += (value == '☑')
    if seen != feature_ids:
        raise ValueError('Backend coverage matrix missing feature IDs: '+', '.join(sorted(feature_ids-seen)))
    return counts


def fmt(label, item):
    if not item['total']:
        return f'{label:<42} n/a'
    return f"{label:<42} {item['verified']:>3}/{item['total']:<3}  {item['verified']/item['total']:>6.1%}"


def validate_evidence(feature_ids, infrastructure_ids, matrix):
    # Requires verifiable evidence references for every checked checkbox/cell.
    def evidence_ids(path, label_index):
        data = path.read_text(encoding='utf-8').split('## Completion evidence')[-1]
        ids=set()
        for ln in data.splitlines():
            if not ln.startswith('| '):
                continue
            parts=[x.strip() for x in ln.split('|')[1:-1]]
            if len(parts)>3 and parts[0] not in ('—','Item ID','Infrastructure item ID') and parts[2] not in ('—',''):
                ids.add(parts[0])
        return ids
    for name, path, ids in [('language', ROOT/'SCORECARD.md', feature_ids),
                             ('infrastructure', ROOT/'implementation/infrastructure.md', infrastructure_ids)]:
        content=path.read_text(encoding='utf-8')
        checked={m.group(2) for ln in content.splitlines() if (m:=ITEM.match(ln)) and m.group(1).lower()=='x'}
        missing=checked-evidence_ids(path,0)
        if missing:
            raise ValueError(f'{name} checked items missing completion evidence: {sorted(missing)}')
    doc=(ROOT/'implementation/backend-coverage.md').read_text(encoding='utf-8')
    evidence=set()
    ev=doc.split('## Backend completion evidence')[-1]
    for ln in ev.splitlines():
        if not ln.startswith('| '):
            continue
        parts=[x.strip() for x in ln.split('|')[1:-1]]
        if len(parts)>=4 and parts[0] in feature_ids and parts[1] in BACKENDS and parts[2] not in ('—','') and parts[3] not in ('—',''):
            evidence.add((parts[0],parts[1]))
    for line in doc.splitlines():
        cells=[v.strip() for v in re.split(r'(?<!\\)\|', line)]
        if len(cells)<9 or not re.fullmatch(r'`[A-Z]+-\d+`', cells[1]):
            continue
        fid=cells[1].strip('`')
        for col,val in zip(BACKENDS,cells[3:8]):
            if val=='☑' and (fid,col) not in evidence:
                raise ValueError(f'Checked backend cell missing evidence: {fid}/{col}')


def main():
    cli=argparse.ArgumentParser(description=__doc__)
    cli.add_argument('--json', action='store_true', help='Print machine-readable totals')
    cli.add_argument('--check', action='store_true', help='Validate completed items have evidence records')
    args=cli.parse_args()
    features, feature_ids=read_checklist(ROOT/'SCORECARD.md')
    infra, infra_ids=read_checklist(ROOT/'implementation/infrastructure.md')
    backend=read_backend(ROOT/'implementation/backend-coverage.md',feature_ids)
    if args.check:
        validate_evidence(feature_ids,infra_ids,backend)
    report={'language': {'total': totals(features), 'sections': features},
            'infrastructure': {'total': totals(infra), 'sections': infra},
            'backends': backend}
    if args.json:
        print(json.dumps(report,indent=2))
        return
    print('Natcore implementation status (unverified work remains unchecked)')
    print('\nLANGUAGE CONFORMANCE')
    for name, stats in features.items():
        print(fmt(name,stats))
    print(fmt('LANGUAGE TOTAL',report['language']['total']))
    print('\nINFRASTRUCTURE READINESS')
    for name, stats in infra.items():
        print(fmt(name,stats))
    print(fmt('INFRASTRUCTURE TOTAL',report['infrastructure']['total']))
    print('\nBACKEND COVERAGE (applicable feature/target cells)')
    for name, stats in backend.items():
        print(fmt(name,stats))


if __name__=='__main__':
    try:
        main()
    except ValueError as exc:
        raise SystemExit(f'Scorecard validation error: {exc}')
