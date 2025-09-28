```markdown
# Feature Specification: Coding CLI (cprcodr-coding-cli)

**Feature Branch**: `001-create-a-coding`
**Created**: 2025-09-28
**Status**: Draft
**Input**: User description: "create a coding cli simular to claudecode, opencode and nanocoder. allow the LLM to be used to be selected, and support models hosted on ollama and lmstudio"

## Execution Flow (main)
```

1. Parse user description from Input
   → If empty: ERROR "No feature description provided"
2. Extract key concepts from description
   → Identify: actors, actions, data, constraints
3. For each unclear aspect:
   → Mark with [NEEDS CLARIFICATION: specific question]
4. Fill User Scenarios & Testing section
   → If no clear user flow: ERROR "Cannot determine user scenarios"
5. Generate Functional Requirements
   → Each requirement must be testable
   → Mark ambiguous requirements
6. Identify Key Entities (if data involved)
7. Run Review Checklist
   → If any [NEEDS CLARIFICATION]: WARN "Spec has uncertainties"
   → If implementation details found: ERROR "Remove tech details"
8. Return: SUCCESS (spec ready for planning)

````

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (except constraints like supported model hosts)
- 👥 Written for stakeholders and developers preparing the implementation plan

### Rust-specific guidance
- This project will be implemented in Rust. The technical context in the plan
  MUST include a `rust-toolchain` file and a Cargo workspace layout.

### For AI Generation
1. **Mark all ambiguities**: Use [NEEDS CLARIFICATION: specific question]
2. **Don't guess**: If the prompt doesn't specify something, mark it
3. **Think like a tester**: Every vague requirement should fail the "testable"
   checklist item

---

## User Scenarios & Testing *(mandatory)*

### Primary User Story
As a developer, I want a command-line tool that helps me generate, run, and
refine code using LLM-based assistants. I can select which LLM backend to use
and choose models hosted on Ollama or LMStudio. I can interactively iterate on
prompts, run generated code locally, and produce artifacts (files, patches)
ready to commit.

### Acceptance Scenarios
1. Given a user has installed the CLI, when they run `cprcodr init`, then a
   workspace and config file are created with defaults.
2. Given a user asks the CLI to generate code for a specified task and selects
   an Ollama model, when the request completes, then generated source files are
   written to the specified directory and tests (if requested) are scaffolded.
3. Given a user wants to switch LLM backend, when they update the config or
   pass `--backend lmstudio`/`--backend ollama`, then subsequent requests use
   the selected provider.

### Edge Cases
- What happens if both Ollama and LMStudio are unavailable? (offline/timeout)
- How are long-running operations (code generation/compilation) reported?
- How are secrets/credentials for model hosts stored and rotated?

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: The CLI MUST initialize a project workspace (`cprcodr init`) with
  a `cprcodr.toml` config and optionally a Cargo workspace skeleton.
- **FR-002**: The CLI MUST allow the user to select an LLM backend per request
  or via configuration: supported backends at launch MUST include `ollama` and
  `lmstudio` and an extensible plugin for additional hosts.
- **FR-003**: The CLI MUST provide commands to generate, preview, and apply
  code patches or files (e.g., `cprcodr gen`, `cprcodr preview`, `cprcodr apply`).
- **FR-004**: The CLI MUST offer an interactive mode for prompt refinement and
  local execution support (e.g., `cprcodr session`).
- **FR-005**: The CLI MUST support running tests and capturing outputs
  (`cargo test`) for Rust projects where applicable.
- **FR-006**: The CLI MUST securely store credentials or configuration for
  model hosts (support env vars and encrypted local credential stores).
- **FR-007**: The CLI MUST provide machine-readable outputs (JSON) and
  human-readable logs for integration in pipelines.

*Unclear / design choices to confirm*
- **FR-008**: Authentication and credential flow for Ollama and LMStudio: are
  users expected to self-host Ollama/LMStudio or connect to a managed service?
  (NEEDS CLARIFICATION)
- **FR-009**: Scope of execution support (sandboxing, Docker, direct local run)
  (NEEDS CLARIFICATION)

### Key Entities
- **ProjectConfig**: stores project-level configuration (`cprcodr.toml`):
  selected default backend, default model, prompt templates, execution options.
- **Session**: interactive session metadata (history of prompts, selected model,
  temporary files, logs).

---

## Review & Acceptance Checklist

### Content Quality
- [ ] No implementation details leaked into requirements (implementation details
  belong in the plan)
- [ ] Focused on user value and measurable outcomes
- [ ] All mandatory sections completed

### Requirement Completeness
- [ ] No [NEEDS CLARIFICATION] markers remain (see unresolved items above)
- [ ] Requirements are testable and unambiguous where possible
- [ ] Success criteria are measurable (files written, tests passing, outputs)

---

## Execution Status

- [ ] User description parsed
- [ ] Key concepts extracted
- [ ] Ambiguities marked
- [ ] User scenarios defined
- [ ] Requirements generated
- [ ] Entities identified
- [ ] Review checklist passed

---

```# Feature Specification: [FEATURE NAME]

**Feature Branch**: `[###-feature-name]`
**Created**: [DATE]
**Status**: Draft
**Input**: User description: "$ARGUMENTS"

## Execution Flow (main)
````

1. Parse user description from Input
   → If empty: ERROR "No feature description provided"
2. Extract key concepts from description
   → Identify: actors, actions, data, constraints
3. For each unclear aspect:
   → Mark with [NEEDS CLARIFICATION: specific question]
4. Fill User Scenarios & Testing section
   → If no clear user flow: ERROR "Cannot determine user scenarios"
5. Generate Functional Requirements
   → Each requirement must be testable
   → Mark ambiguous requirements
6. Identify Key Entities (if data involved)
7. Run Review Checklist
   → If any [NEEDS CLARIFICATION]: WARN "Spec has uncertainties"
   → If implementation details found: ERROR "Remove tech details"
8. Return: SUCCESS (spec ready for planning)

```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

### Section Requirements
- **Mandatory sections**: Must be completed for every feature
- **Optional sections**: Include only when relevant to the feature
- When a section doesn't apply, remove it entirely (don't leave as "N/A")

### Rust-specific guidance
- If the project uses Rust, include the toolchain (`rust-toolchain`) and primary
  crates in the Technical Context. Do not bake implementation details into the
  spec; specify constraints (e.g., need for a persistent cache) and leave backend
  choice to design unless explicitly requested.

### For AI Generation
When creating this spec from a user prompt:
1. **Mark all ambiguities**: Use [NEEDS CLARIFICATION: specific question] for any assumption you'd need to make
2. **Don't guess**: If the prompt doesn't specify something (e.g., "login system" without auth method), mark it
3. **Think like a tester**: Every vague requirement should fail the "testable and unambiguous" checklist item
4. **Common underspecified areas**:
   - User types and permissions
   - Data retention/deletion policies
   - Performance targets and scale
   - Error handling behaviors
   - Integration requirements
   - Security/compliance needs

---

## User Scenarios & Testing *(mandatory)*

### Primary User Story
[Describe the main user journey in plain language]

### Acceptance Scenarios
1. **Given** [initial state], **When** [action], **Then** [expected outcome]
2. **Given** [initial state], **When** [action], **Then** [expected outcome]

### Edge Cases
- What happens when [boundary condition]?
- How does system handle [error scenario]?

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST [specific capability, e.g., "allow users to create accounts"]
- **FR-002**: System MUST [specific capability, e.g., "validate email addresses"]
- **FR-003**: Users MUST be able to [key interaction, e.g., "reset their password"]
- **FR-004**: System MUST [data requirement, e.g., "persist user preferences"]
- **FR-005**: System MUST [behavior, e.g., "log all security events"]

*Example of marking unclear requirements:*
- **FR-006**: System MUST authenticate users via [NEEDS CLARIFICATION: auth method not specified - email/password, SSO, OAuth?]
- **FR-007**: System MUST retain user data for [NEEDS CLARIFICATION: retention period not specified]

### Key Entities *(include if feature involves data)*
- **[Entity 1]**: [What it represents, key attributes without implementation]
- **[Entity 2]**: [What it represents, relationships to other entities]

---

## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [ ] No implementation details (languages, frameworks, APIs)
- [ ] Focused on user value and business needs
- [ ] Written for non-technical stakeholders
- [ ] All mandatory sections completed

### Requirement Completeness
- [ ] No [NEEDS CLARIFICATION] markers remain
- [ ] Requirements are testable and unambiguous
- [ ] Success criteria are measurable
- [ ] Scope is clearly bounded
- [ ] Dependencies and assumptions identified

---

## Execution Status
*Updated by main() during processing*

- [ ] User description parsed
- [ ] Key concepts extracted
- [ ] Ambiguities marked
- [ ] User scenarios defined
- [ ] Requirements generated
- [ ] Entities identified
- [ ] Review checklist passed

---
```
