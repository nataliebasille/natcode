#!/usr/bin/env python3
"""Test lifecycle rules for Natcode scorecards."""
from __future__ import annotations

import sys
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))
import scorecard_status as status


CARD_PATH = status.SCORECARDS / 'lexer.md'


def item(item_id: str, state: str, **fields: str) -> status.Item:
    return status.Item(
        id=item_id,
        title='Example',
        file=CARD_PATH,
        line=1,
        fields={'Status': state, 'Spec': 'None', **fields},
    )


def problems(*items: status.Item) -> list[str]:
    card = status.Scorecard(path=CARD_PATH, prefix='LEX', items=list(items))
    errors: list[str] = []
    with patch.object(status, 'check_references'):
        status.validate([card], errors)
    return errors


class ScorecardLifecycleTests(unittest.TestCase):
    def test_needs_update_preserves_old_test_references(self) -> None:
        updated = item(
            'LEX-KW-07', 'Needs update',
            **{
                'Done when': 'when is the reserved keyword',
                'Previous evidence': '`tests/old.rs::keywords::old_test`',
            },
        )
        self.assertEqual(problems(updated), [])

    def test_needs_update_requires_historical_evidence(self) -> None:
        updated = item('LEX-KW-07', 'Needs update', **{'Done when': 'new behavior'})
        self.assertTrue(any('lacks backticked Previous evidence' in e for e in problems(updated)))

    def test_superseded_points_to_existing_replacement(self) -> None:
        retired = item(
            'LEX-KW-07', 'Superseded',
            **{'Superseded by': 'LEX-KW-09', 'Notes': 'Replaced by a different token.'},
        )
        replacement = item('LEX-KW-09', 'Approved', **{'Done when': 'new token works'})
        self.assertEqual(problems(retired, replacement), [])

    def test_superseded_can_have_no_replacement(self) -> None:
        retired = item(
            'LEX-KW-07', 'Superseded',
            **{'Superseded by': 'None', 'Notes': 'Feature removed without replacement.'},
        )
        self.assertEqual(problems(retired), [])

    def test_superseded_requires_valid_replacement_and_reason(self) -> None:
        retired = item('LEX-KW-07', 'Superseded', **{'Superseded by': 'LEX-KW-99'})
        errors = problems(retired)
        self.assertTrue(any('unknown ID' in e for e in errors))
        self.assertTrue(any('lacks Notes' in e for e in errors))

    def test_superseded_rejects_self_reference(self) -> None:
        retired = item(
            'LEX-KW-07', 'Superseded',
            **{'Superseded by': 'LEX-KW-07', 'Notes': 'Incorrect reference.'},
        )
        self.assertTrue(any('cannot supersede itself' in e for e in problems(retired)))

    def test_active_items_cannot_claim_supersession(self) -> None:
        active = item(
            'LEX-KW-07', 'Approved',
            **{'Done when': 'token works', 'Superseded by': 'LEX-KW-09'},
        )
        self.assertTrue(any('has Superseded by but is Approved' in e for e in problems(active)))

    def test_reimplemented_items_can_keep_previous_evidence(self) -> None:
        implemented = item(
            'LEX-KW-07', 'Implemented',
            **{
                'Done when': 'when token works',
                'Evidence': '`tests/new.rs::keywords::new_test`',
                'Previous evidence': '`tests/removed.rs::keywords::old_test`',
            },
        )
        with patch.object(status, 'check_evidence'):
            self.assertEqual(problems(implemented), [])

    def test_summary_counts_new_statuses(self) -> None:
        card = status.Scorecard(
            path=CARD_PATH,
            prefix='LEX',
            items=[
                item('LEX-KW-07', 'Needs update'),
                item('LEX-KW-09', 'Superseded'),
            ],
        )
        report = status.summarize([card])
        self.assertEqual(report['total']['Needs update'], 1)
        self.assertEqual(report['total']['Superseded'], 1)
        self.assertEqual(report['total']['total'], 2)


if __name__ == '__main__':
    unittest.main()
