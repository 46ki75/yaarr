#!/usr/bin/env python3
"""Convert MCP MDX spec files to clean Markdown for skill reference files."""

import argparse
import re
from pathlib import Path

DOCS_BASE = Path("submodules/modelcontextprotocol/docs")
OUT_BASE = Path("skills/mcp-knowledge/references")

VERSIONS = ["2024-11-05", "2025-03-26", "2025-06-18", "2025-11-25", "2026-07-28"]

# Files to include per version (relative to version dir)
FILES_PER_VERSION = {
    "2024-11-05": [
        ("architecture/index.mdx", "architecture.md"),
        ("basic/lifecycle.mdx", "lifecycle.md"),
        ("basic/transports.mdx", "transports.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/utilities/cancellation.mdx", "cancellation.md"),
        ("basic/utilities/ping.mdx", "ping.md"),
        ("basic/utilities/progress.mdx", "progress.md"),
    ],
    "2025-03-26": [
        ("changelog.mdx", "changelog.md"),
        ("architecture/index.mdx", "architecture.md"),
        ("basic/lifecycle.mdx", "lifecycle.md"),
        ("basic/transports.mdx", "transports.md"),
        ("basic/authorization.mdx", "authorization.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/utilities/cancellation.mdx", "cancellation.md"),
        ("basic/utilities/ping.mdx", "ping.md"),
        ("basic/utilities/progress.mdx", "progress.md"),
    ],
    "2025-06-18": [
        ("changelog.mdx", "changelog.md"),
        ("architecture/index.mdx", "architecture.md"),
        ("basic/lifecycle.mdx", "lifecycle.md"),
        ("basic/transports.mdx", "transports.md"),
        ("basic/authorization.mdx", "authorization.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("client/elicitation.mdx", "elicitation.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/utilities/cancellation.mdx", "cancellation.md"),
        ("basic/utilities/ping.mdx", "ping.md"),
        ("basic/utilities/progress.mdx", "progress.md"),
    ],
    "2025-11-25": [
        ("changelog.mdx", "changelog.md"),
        ("architecture/index.mdx", "architecture.md"),
        ("basic/lifecycle.mdx", "lifecycle.md"),
        ("basic/transports.mdx", "transports.md"),
        ("basic/authorization.mdx", "authorization.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("client/elicitation.mdx", "elicitation.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/utilities/cancellation.mdx", "cancellation.md"),
        ("basic/utilities/ping.mdx", "ping.md"),
        ("basic/utilities/progress.mdx", "progress.md"),
        ("basic/utilities/tasks.mdx", "tasks.md"),
    ],
    "2026-07-28": [
        ("changelog.mdx", "changelog.md"),
        ("architecture/index.mdx", "architecture.md"),
        ("basic/index.mdx", "protocol.md"),
        ("basic/versioning.mdx", "versioning.md"),
        ("basic/transports/index.mdx", "transports.md"),
        ("basic/transports/stdio.mdx", "stdio.md"),
        ("basic/transports/streamable-http.mdx", "streamable-http.md"),
        ("basic/authorization/index.mdx", "authorization.md"),
        ("basic/authorization/authorization-server-discovery.mdx", "authorization-discovery.md"),
        ("basic/authorization/client-registration.mdx", "client-registration.md"),
        ("basic/authorization/security-considerations.mdx", "authorization-security.md"),
        ("basic/patterns/mrtr.mdx", "mrtr.md"),
        ("basic/patterns/subscriptions.mdx", "subscriptions.md"),
        ("server/discover.mdx", "discovery.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("client/elicitation.mdx", "elicitation.md"),
        ("server/utilities/caching.mdx", "caching.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/patterns/cancellation.mdx", "cancellation.md"),
        ("basic/patterns/progress.mdx", "progress.md"),
        ("deprecated.mdx", "deprecated.md"),
    ],
}

EXTENSION_FILES = [
    ("extensions/overview.mdx", "2026-07-28/extensions.md"),
    ("extensions/tasks/overview.mdx", "2026-07-28/tasks-extension.md"),
]

SCHEMA_FILES = [
    ("2026-07-28/schema.ts", "2026-07-28/schema.ts"),
]


def convert_mdx(content: str, title_from_frontmatter: str = "") -> str:
    """Convert MDX content to clean Markdown."""

    # Strip YAML frontmatter and extract title
    frontmatter_match = re.match(r"^---\n(.*?)\n---\n", content, re.DOTALL)
    fm_title = ""
    if frontmatter_match:
        fm_block = frontmatter_match.group(1)
        title_match = re.search(r"^title:\s*(.+)$", fm_block, re.MULTILINE)
        if title_match:
            fm_title = title_match.group(1).strip().strip('"')
        content = content[frontmatter_match.end():]

    title = fm_title or title_from_frontmatter

    # Prepend markdownlint-disable so all MDX-converted files are exempt
    header = "<!-- markdownlint-disable -->\n"
    if title:
        header += f"# {title}\n\n"

    # Strip <div ...> tags (self-closing or with content)
    content = re.sub(r"<div[^>]*/>\s*", "", content)
    content = re.sub(r"<div[^>]*>.*?</div>", "", content, flags=re.DOTALL)

    # Convert <Warning>, <Note>, <Info> blocks to blockquotes
    for tag in ("Warning", "Note", "Info"):
        def replace_block(m, tag=tag):
            inner = m.group(1).strip()
            lines = inner.split("\n")
            quoted = "\n".join(f"> {line}" if line.strip() else ">" for line in lines)
            return f"\n> **{tag}:**\n{quoted}\n"
        content = re.sub(
            rf"<{tag}>\s*(.*?)\s*</{tag}>",
            replace_block,
            content,
            flags=re.DOTALL,
        )

    # Strip grouped navigation cards, but preserve standalone source links.
    content = re.sub(r"<CardGroup[^>]*>.*?</CardGroup>", "", content, flags=re.DOTALL)

    def replace_card(m):
        attrs, inner = m.groups()
        title_match = re.search(r'title="([^"]+)"', attrs)
        href_match = re.search(r'href="([^"]+)"', attrs)
        if title_match and href_match:
            return f"\n[{title_match.group(1)}]({href_match.group(1)})\n\n{inner.strip()}\n"
        return inner.strip()

    content = re.sub(r"<Card\s+([^>]*)>\s*(.*?)\s*</Card>", replace_card, content, flags=re.DOTALL)
    content = re.sub(r"<Card[^/].*?/>", "", content, flags=re.DOTALL)

    # Strip any remaining self-closing JSX tags
    content = re.sub(r"<[A-Z][A-Za-z]*[^>]*/>\s*", "", content)

    # Strip any remaining JSX opening/closing tags that may remain
    content = re.sub(r"<[A-Z][A-Za-z]*[^>]*>", "", content)
    content = re.sub(r"</[A-Z][A-Za-z]*>", "", content)

    # Collapse 3+ blank lines to 2
    content = re.sub(r"\n{3,}", "\n\n", content)
    content = "\n".join(line.rstrip() for line in content.splitlines())

    return (header + content.lstrip()).rstrip() + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--docs-base", type=Path, default=DOCS_BASE)
    parser.add_argument("--out-base", type=Path, default=OUT_BASE)
    args = parser.parse_args()

    spec_base = args.docs_base / "specification"

    for version in VERSIONS:
        out_dir = args.out_base / version
        out_dir.mkdir(parents=True, exist_ok=True)

        for src_rel, dst_name in FILES_PER_VERSION[version]:
            src_path = spec_base / version / src_rel
            if not src_path.exists():
                raise FileNotFoundError(src_path)

            raw = src_path.read_text(encoding="utf-8")
            converted = convert_mdx(raw)

            dst_path = out_dir / dst_name
            dst_path.write_text(converted, encoding="utf-8")
            print(f"  wrote {dst_path}")

    for src_rel, dst_rel in EXTENSION_FILES:
        src_path = args.docs_base / src_rel
        if not src_path.exists():
            raise FileNotFoundError(src_path)

        dst_path = args.out_base / dst_rel
        dst_path.parent.mkdir(parents=True, exist_ok=True)
        dst_path.write_text(convert_mdx(src_path.read_text(encoding="utf-8")), encoding="utf-8")
        print(f"  wrote {dst_path}")

    schema_base = args.docs_base.parent / "schema"
    for src_rel, dst_rel in SCHEMA_FILES:
        src_path = schema_base / src_rel
        if not src_path.exists():
            raise FileNotFoundError(src_path)

        dst_path = args.out_base / dst_rel
        dst_path.parent.mkdir(parents=True, exist_ok=True)
        dst_path.write_text(src_path.read_text(encoding="utf-8"), encoding="utf-8")
        print(f"  wrote {dst_path}")

    print("Done.")


if __name__ == "__main__":
    main()
