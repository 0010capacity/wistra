# wistra — Project Plan

> AI-powered personal wiki builder. Scans a knowledge graph, fills stub concepts, resolves disambiguation, and keeps everything connected — entirely from the command line.

---

## 1. Vision & Principles

wistra does exactly one thing well: **grow a knowledge graph**. It is not a renderer, not a search engine, not a note-taking app. It produces plain Markdown files that any tool can read, and its only opinion about format is that documents must be self-describing.

### Core Principles

| Principle | What it means |
|-----------|---------------|
| **Data over tools** | A document must be fully meaningful when opened in any text editor, printed, or read raw |
| **Obsidian Flavor compatibility** | `[[wikilinks]]`, YAML frontmatter, and standard Markdown syntax — nothing proprietary |
| **One responsibility** | wistra fills and connects. Rendering, querying, and dashboards are the user's problem |
| **Explicit over magic** | Every mutation is previewed and confirmed before writing to disk |
| **Language-first** | The user declares a language once; every generated document respects it |

---

## 2. Document Format Specification

### 2.1 File Conventions

- **Filename**: English title, spaces allowed, `.md` extension
  - `Artificial Intelligence.md`
  - `Apple (Fruit).md`
  - `Apple (Company).md`
- **Location**: all concept documents live in `concepts/`
- **Encoding**: UTF-8, LF line endings

### 2.2 YAML Frontmatter — Required Fields

Every document wistra generates **must** contain these fields. Documents without them are treated as foreign and left untouched.

```yaml
---
title: "Artificial Intelligence"
aliases:
  - AI
  - 인공지능           # added when user language is Korean
  - Artificial Intelligence
tags:
  - computer-science/ai
  - science/cognitive
status: published        # stub | published | disambiguation
language: ko             # mirrors user config
created: 2026-03-26
---
```

| Field | Required | Description |
|-------|----------|-------------|
| `title` | ✅ | Canonical English title |
| `aliases` | ✅ | Alternate names; includes native-language equivalents |
| `tags` | ✅ | Hierarchical, slash-delimited |
| `status` | ✅ | `stub` / `published` / `disambiguation` |
| `language` | ✅ | ISO 639-1 code of the document body |
| `created` | ✅ | ISO 8601 date |

Optional fields wistra may add:

```yaml
relates:
  - "[[Machine Learning]]"
  - "[[Alan Turing]]"
disambig: "[[AI]]"        # points to disambiguation doc, if applicable
```

### 2.3 Document Body Structure

wistra does not enforce a rigid template. The AI is instructed to write naturally. However, the following structural elements are used where appropriate:

**Sections** (H2 level, in order if applicable):

```markdown
## Overview
## Background / History
## Key Concepts
## See Also
```

`## See Also` always uses `[[wikilinks]]` for cross-references.

**Inline wikilinks** anywhere in the body:

```markdown
[[Machine Learning]] is a subfield of [[Artificial Intelligence]]...
```

**LaTeX** for mathematical notation (inline and block):

```markdown
The relationship is defined as $E = mc^2$.

$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$
```

**Code blocks** with language tags:

````markdown
```python
def fibonacci(n):
    return n if n <= 1 else fibonacci(n-1) + fibonacci(n-2)
```
````

**Blockquotes** for definitions or primary source excerpts:

```markdown
> "The question is not whether machines can think, but whether men do."
> — B.F. Skinner
```

**Callout blocks** (Obsidian-compatible):

```markdown
> [!note]
> This article covers the theoretical foundations only.

> [!warning]
> The term is used inconsistently across literature.
```

### 2.4 Disambiguation Documents

When two concepts share a title, wistra creates a disambiguation hub:

```yaml
---
title: "Apple"
aliases: []
tags: []
status: disambiguation
language: ko
created: 2026-03-26
---
```

```markdown
**Apple**은 여러 의미를 가집니다:

- [[Apple (Fruit)]] — 사과나무의 열매
- [[Apple (Company)]] — 미국의 IT 기업
```

### 2.5 Stub Documents

When a `[[wikilink]]` target does not yet exist, wistra creates a minimal stub:

```yaml
---
title: "Turing Machine"
aliases:
  - 튜링 머신
tags: []
status: stub
language: ko
created: 2026-03-26
---
```

```markdown
<!-- stub: created by wistra, pending generation -->
```

---

## 3. Tag System

### 3.1 Design Rules

- **Hierarchical**: slash-delimited, maximum 3 levels deep
- **Lowercase, hyphenated**: `computer-science/ai`, not `ComputerScience/AI`
- **Noun phrases only**: `programming/languages`, not `programming/using-languages`
- **Multiple axes**: a document should have tags across independent dimensions
- **Language-neutral**: tags are always in English regardless of document language

```yaml
# Good
tags:
  - mathematics/analysis          # domain axis
  - history/20th-century          # era axis
  - person/mathematician          # type axis

# Bad — too vague, mixed case, verb form
tags:
  - Math
  - doing-analysis
```

### 3.2 Canonical Tag Hierarchy (Seed)

```
science/
  physics/
  chemistry/
  biology/
  cognitive/
mathematics/
  analysis/
  algebra/
  discrete/
  statistics/
computer-science/
  ai/
  systems/
  networking/
  programming-languages/
  algorithms/
history/
  ancient/
  medieval/
  modern/
  20th-century/
culture/
  art/
  music/
  literature/
  film/
philosophy/
  ethics/
  epistemology/
  logic/
economics/
  macroeconomics/
  microeconomics/
person/
  scientist/
  mathematician/
  engineer/
  artist/
organization/
  company/
  institution/
  government/
technology/
  hardware/
  software/
  internet/
subculture/
  anime/
  gaming/
  internet-culture/
```

Users extend this hierarchy freely. wistra validates that new tags fit within the existing tree, or prompts to create a new branch.

### 3.3 Tag Index

`meta/tag-index.md` is regenerated on every run:

```markdown
---
title: Tag Index
status: meta
---

## computer-science/ai
- [[Artificial Intelligence]]
- [[Machine Learning]]
- [[Neural Network]]

## mathematics/analysis
- [[Fourier Transform]]
- [[Calculus]]
```

---

## 4. Alias & Multilingual Strategy

### 4.1 Title Language

Document **titles and filenames are always English**. This is the canonical identifier.

### 4.2 Alias Population

When `language = ko`, wistra instructs the AI to populate aliases with:

1. The English title itself (for wikilink compatibility)
2. The Korean equivalent, if well-established (`인공지능`, `컴퓨터 과학`)
3. Common abbreviations (`AI`, `CS`, `ML`)
4. Romanized Korean or alternate spellings if ambiguous

```yaml
# language: ko example
title: "Artificial Intelligence"
aliases:
  - AI
  - 인공지능
  - 인공 지능
```

```yaml
# language: en example
title: "Artificial Intelligence"
aliases:
  - AI
  - Artificial Intelligence
```

### 4.3 Body Language

The document body is written in the user's configured language. The title in the H1 or first sentence may render the native-language name naturally:

```markdown
# Artificial Intelligence

**인공지능**(Artificial Intelligence, AI)은 인간의 학습, 추론, 인식 능력을
컴퓨터 시스템으로 구현하는 컴퓨터 과학의 한 분야다.
```

---

## 5. CLI Design

### 5.1 Commands

```
wistra onboard               초기 설정 마법사
wistra run [path] [count]    위키 채우기 실행 (기본값: config 경로, 5개)
wistra scan [path]           상태 리포트 (파일 변경 없음)
wistra config                설정 수정
wistra interests             관심 분야 수정
wistra status                현재 위키 통계 요약
wistra help                  도움말
```

### 5.2 `wistra onboard` Flow

```
? Wiki path › ~/wiki
? Language › ● 한국어  ○ English
? Adapter › ● Claude API
? Claude API key › sk-ant-...
? Daily concept count › 5

? Interests (space to select)
  ● Science      ● Mathematics
  ● Programming  ● Computer Science
  ● History      ● Culture
  ○ Current Affairs  ● Subculture
  ○ Economics    ○ Philosophy

? Set up daily cron job? › ● Yes  ○ No
  → 0 9 * * * wistra run --quiet --no-confirm

✅ Config saved → ~/wiki/.wistra/config.toml
✅ Directory structure initialized
```

### 5.3 `wistra run` Flow

```
wistra run ~/wiki 5

🔍 Scanning...
   247 documents found
   34 stub links detected
   2 disambiguation candidates: Apple, Python

📋 Execution plan (5 slots):
   [1] disambig  → Apple            (Apple (Fruit) / Apple (Company))
   [2] stub      → Turing Machine   (mathematics, computer-science)
   [3] stub      → TCP/IP           (computer-science/networking)
   [4] random    → Renaissance      (history/modern, culture/art)
   [5] random    → Fourier Transform (mathematics/analysis, science/physics)

⚠️  Link updates required:
    [[Apple]] → 12 documents will be rewritten

? Proceed? › ● Yes  ○ No  ○ Save plan only

✍️  Generating...
   [1/5] Apple disambiguation... ✅
   [2/5] Turing Machine... ✅
   [3/5] TCP/IP... ✅
   [4/5] Renaissance... ✅
   [5/5] Fourier Transform... ✅

🔗 Updating links...
   12 files updated (Apple → Apple (Company) / Apple (Fruit))

✅ Done. 5 documents added, 12 links updated.
```

### 5.4 Slot Priority Logic

```
1st priority  Disambiguation resolution   (always first, blocks graph integrity)
2nd priority  Stub fill                   (sorted by inbound link count, descending)
3rd priority  Interest-based random       (fills remaining slots)
```

Random selection within interests:
- Each domain gets a weight; recently-generated domains get lower weight (rebalancing)
- Concepts that would link to existing documents are preferred over isolated ones
- AI is given the current tag index to suggest a concept that extends the graph naturally

### 5.5 `wistra scan` Output

```
📊 Wiki Status
   Total documents  : 247
   Published        : 231
   Stubs            : 16
   Disambiguation   : 3

🏷️  Tags
   Unique tags      : 89
   Most used        : computer-science/ai (34), mathematics/analysis (21)
   Orphan tags      : 2

🔗 Links
   Total links      : 1,204
   Broken links     : 7
   Stub targets     : 34

⚠️  Action required
   Disambiguation   : Apple, Python
   Broken links     : Lisp, Von Neumann Architecture, Chomsky Hierarchy...
```

---

## 6. File System Layout

```
~/wiki/
├── concepts/                      # all knowledge documents
│   ├── Artificial Intelligence.md
│   ├── Apple.md                   # disambiguation
│   ├── Apple (Fruit).md
│   ├── Apple (Company).md
│   └── ...
└── meta/
    ├── stubs.md                   # auto-managed stub list
    ├── disambig-queue.md          # pending disambiguation
    └── tag-index.md               # full tag → document index

~/wiki/.wistra/
├── config.toml                    # user configuration
├── state.json                     # last run snapshot
└── logs/
    ├── 2026-03.log
    └── ...
```

`meta/` and `.wistra/` are always present. They are maintained entirely by wistra and should not be edited by hand.

---

## 7. AI Adapter Design

### 7.1 Context Strategy

The AI receives a **compressed index** of the entire wiki on every call, not full document bodies. This keeps token usage predictable.

```json
[
  {
    "title": "Artificial Intelligence",
    "tags": ["computer-science/ai", "science/cognitive"],
    "aliases": ["AI", "인공지능"],
    "summary": "컴퓨터 시스템으로 인간의 인지 능력을 구현하는 분야",
    "status": "published"
  },
  ...
]
```

The summary is extracted as the first non-empty sentence of the body at scan time.

### 7.2 Generation Prompt (Concept)

```
당신은 지식 위키의 문서를 작성하는 에디터입니다.

[위키 현황]
- 언어: 한국어
- 기존 문서 수: 247개
- 태그 체계: <tag-index>
- 기존 문서 인덱스: <wiki-index>

[작성 규칙]
1. YAML frontmatter를 반드시 포함할 것 (title, aliases, tags, status, language, created)
2. 제목은 영문, 본문은 한국어로 작성
3. aliases에 한국어 동의어와 약어를 포함할 것
4. tags는 기존 태그 체계를 따를 것. 새 태그가 필요하면 기존 계층에 편입
5. 본문에서 기존 위키 문서를 [[wikilink]]로 참조할 것
6. 수식은 LaTeX, 코드는 코드블록, 인용은 blockquote 사용
7. status는 published로 설정
8. 길이는 300~800 단어 사이

[작성 대상]
개념명: Turing Machine
관련 기존 문서: [[Alan Turing]], [[Computability Theory]], [[Computer Science]]
```

### 7.3 Disambiguation Prompt

```
다음 두 문서가 같은 제목을 공유하고 있습니다.

[문서 A 컨텍스트]
title: Apple
링크 출처 문서들의 주변 문맥:
  - "Apple의 CEO Tim Cook은..." → 기업 의미
  - "Apple Watch 신제품..." → 기업 의미

[문서 B 컨텍스트]
title: Apple
링크 출처 문서들의 주변 문맥:
  - "Apple 한 알..." → 과일 의미
  - "Apple 나무 재배..." → 과일 의미

지시:
1. 두 개념에 적절한 qualifier를 결정하시오 (예: Apple (Fruit), Apple (Company))
2. 각 문서의 새 frontmatter를 작성하시오
3. 기존 [[Apple]] 링크들을 어느 쪽으로 재분류할지 목록으로 출력하시오
4. disambiguation 문서를 작성하시오

응답은 JSON으로:
{
  "concept_a": { "new_title": "...", "frontmatter": "...", "body": "..." },
  "concept_b": { "new_title": "...", "frontmatter": "...", "body": "..." },
  "disambig": { "frontmatter": "...", "body": "..." },
  "link_updates": [
    { "file": "Tim Cook.md", "from": "Apple", "to": "Apple (Company)" },
    ...
  ]
}
```

### 7.4 Adapter Interface (Rust trait)

```rust
pub trait WikiAdapter {
    async fn generate_concept(&self, ctx: GenerationContext) -> Result<Document>;
    async fn resolve_disambiguation(&self, ctx: DisambigContext) -> Result<DisambigResult>;
}
```

Only `ClaudeAdapter` is implemented initially. The trait exists to allow future adapters (GPT, Gemini, local LLM).

---

## 8. Rust Crate Structure

```
wistra/
├── Cargo.toml
├── src/
│   ├── main.rs              entry point, CLI routing
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── onboard.rs       onboard wizard
│   │   ├── run.rs           run command
│   │   ├── scan.rs          scan command
│   │   └── config.rs        config command
│   ├── scanner/
│   │   ├── mod.rs
│   │   ├── parser.rs        frontmatter + wikilink extraction
│   │   ├── graph.rs         link graph construction
│   │   └── report.rs        scan report generation
│   ├── planner/
│   │   ├── mod.rs
│   │   ├── priority.rs      slot assignment logic
│   │   └── interest.rs      weighted random selection
│   ├── adapter/
│   │   ├── mod.rs           WikiAdapter trait
│   │   └── claude.rs        Claude API implementation
│   ├── writer/
│   │   ├── mod.rs
│   │   ├── document.rs      document serialization
│   │   └── linker.rs        bulk link rewrite
│   ├── config/
│   │   ├── mod.rs
│   │   └── types.rs         Config, UserConfig structs
│   └── types/
│       └── mod.rs           Document, Tag, Link, WikiIndex structs
```

### Key Dependencies

```toml
[dependencies]
clap         = { version = "4", features = ["derive"] }
tokio        = { version = "1", features = ["full"] }
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
toml         = "0.8"
reqwest      = { version = "0.12", features = ["json"] }
gray_matter  = "0.2"   # YAML frontmatter parsing
walkdir      = "2"
regex        = "1"
indicatif    = "0.17"  # progress bars
dialoguer    = "0.11"  # interactive prompts
chrono       = "0.4"
anyhow       = "1"
```

---

## 9. State & Safety

### state.json

```json
{
  "last_run": "2026-03-26T09:00:00Z",
  "documents_total": 247,
  "last_added": ["Turing Machine", "TCP/IP", "Renaissance"],
  "pending_disambig": ["Python"],
  "broken_links": ["Lisp", "Von Neumann Architecture"]
}
```

### Mutation Safety Rules

1. wistra **never** modifies a document that does not have a valid `status` field in frontmatter
2. All file writes are batched; if any write fails, the entire run is rolled back
3. Before any rename, a dry-run diff is printed
4. `--dry-run` flag always available to preview without writing

---

## 10. Out of Scope

These are explicitly outside wistra's responsibility:

- Rendering or serving documents
- Search indexing
- Dataview or query evaluation
- Graph visualization
- Conflict resolution for simultaneous edits
- Version history (delegated to git)
