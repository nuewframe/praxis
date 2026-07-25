#!/usr/bin/env python3
"""Citation-vs-assertion detection for Praxis's literal scanners.

Two of the plugin's self-checks are literal scanners: the version single-source
audit (`bump-version.sh --audit`) and the terminology check (`validate-plugin.sh`
check #10). Neither could distinguish a document that *asserts* a literal from
one that *cites* it, and the answer used to be path allowlists — which exempted
whole directories and so inverted default-deny. See ADR.260725.

This module is the shared implementation of the replacement. It exists as one
file, consumed by every scanner, because the ADR's risk table calls out that a
second fence parser would repeat check #14's marker-length false negative.

Two layers:

  Layer 1 - structural. A literal inside a fenced code block, a blockquote, or
            an inline code span is a citation by position. No configuration.
  Layer 2 - declared. A citation in running prose carries a marker on the
            preceding line with a mandatory reason:

                <!-- praxis:allow-version-literal reason="why this is a citation" -->

            A marker with a missing or empty reason is itself a failure. An
            unexplained exemption is the defect being removed.

Library use:

    import citation_scan
    result = citation_scan.analyze(path, marker='praxis:allow-term',
                                   pattern=r'\\bthe bet\\b')
    result.exempt        # {lineno, ...} lines whose matches are all citations
    result.bad_markers   # [(lineno, message), ...]

CLI use (for the bash callers):

    python3 scripts/citation_scan.py <file> --marker <name> [--pattern <regex>]

    EXEMPT <lineno>
    BADMARKER <lineno> <message>

Standard library only. Compatible with the python3 the other checks assume.
"""

import argparse
import re
import sys

# Marker-length aware: a ```` ``` ```` inside a ```` ```` ```` block must not
# close it. A naive three-backtick toggle produced a real false negative, which
# is why this rule lives in exactly one place.
FENCE_RE = re.compile(r'^\s*(`{3,}|~{3,})')
BLOCKQUOTE_RE = re.compile(r'^\s*>')
CODE_SPAN_RE = re.compile(r'(`+)(?:(?!\1).)*?\1', re.DOTALL)
MARKER_RE_TMPL = r'<!--\s*{marker}\b(?P<rest>[^>]*?)-->'
REASON_RE = re.compile(r'reason\s*=\s*"([^"]*)"')


class Result(object):
    """Lines whose literals are citations, plus markers that explain nothing."""

    def __init__(self, exempt, bad_markers):
        self.exempt = exempt
        self.bad_markers = bad_markers


def fenced_lines(lines):
    """1-indexed line numbers inside a fenced code block, fences included."""
    inside = set()
    in_fence = False
    marker = None
    for lineno, line in enumerate(lines, 1):
        m = FENCE_RE.match(line)
        if m:
            tok = m.group(1)
            inside.add(lineno)
            if not in_fence:
                in_fence, marker = True, tok[0] * 3
            elif line.strip().startswith(marker):
                in_fence, marker = False, None
            continue
        if in_fence:
            inside.add(lineno)
    return inside


def strip_code_spans(text):
    """Blank inline code spans, preserving length so offsets stay valid.

    A literal inside `backticks` is a citation. Blanking rather than deleting
    keeps column positions meaningful for any caller that reports them.
    """
    return CODE_SPAN_RE.sub(lambda m: ' ' * len(m.group(0)), text)


def marker_lines(lines, marker):
    """Return (covered, bad) for an inline marker.

    covered: line numbers exempted by a well-formed marker on the line above.
    bad:     (lineno, message) for a marker with a missing or empty reason.
    """
    covered = set()
    bad = []
    if not marker:
        return covered, bad
    marker_re = re.compile(MARKER_RE_TMPL.format(marker=re.escape(marker)))
    for lineno, line in enumerate(lines, 1):
        m = marker_re.search(line)
        if not m:
            continue
        reason = REASON_RE.search(m.group('rest') or '')
        if reason is None:
            bad.append((lineno,
                        "%s has no reason= -- an unexplained exemption is the "
                        "defect this marker exists to remove" % marker))
            continue
        if not reason.group(1).strip():
            bad.append((lineno,
                        "%s has an empty reason= -- state why the literal is a "
                        "citation, not that it is one" % marker))
            continue
        # Scope is the following line. The range form is deliberately not
        # implemented until a real case demands it (ADR.260725).
        covered.add(lineno + 1)
        covered.add(lineno)
    return covered, bad


def analyze(path, marker=None, pattern=None):
    """Classify each line of `path` as citation or assertion.

    With `pattern`, a line is exempt only when *every* match of that pattern on
    it sits inside an inline code span — precision a whole-line rule cannot give.
    Without `pattern`, code-span content alone does not exempt a line, since
    there is nothing to locate.
    """
    try:
        with open(path, errors='replace') as fh:
            lines = fh.read().split('\n')
    except (IOError, OSError):
        return Result(set(), [])

    exempt = set()
    exempt |= fenced_lines(lines)

    covered, bad = marker_lines(lines, marker)
    exempt |= covered

    pat = re.compile(pattern) if pattern else None

    for lineno, line in enumerate(lines, 1):
        if lineno in exempt:
            continue
        if BLOCKQUOTE_RE.match(line):
            exempt.add(lineno)
            continue
        if pat is not None and pat.search(line):
            # Every match hidden by a code span means the line only cites.
            if not pat.search(strip_code_spans(line)):
                exempt.add(lineno)

    return Result(exempt, bad)


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('file')
    ap.add_argument('--marker', default=None,
                    help='inline marker name, e.g. praxis:allow-version-literal')
    ap.add_argument('--pattern', default=None,
                    help='regex the caller is scanning for; enables code-span precision')
    args = ap.parse_args(argv)

    result = analyze(args.file, marker=args.marker, pattern=args.pattern)
    for lineno in sorted(result.exempt):
        print('EXEMPT %d' % lineno)
    for lineno, msg in result.bad_markers:
        print('BADMARKER %d %s' % (lineno, msg))
    return 0


if __name__ == '__main__':
    sys.exit(main())
