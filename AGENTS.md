# Agents guideline for chat-based development

Read ./README.md to understand the project goal.

## Chatting

If I ask you a question, don't automatically assume it's an implementation
request. Answer the question first and then ask if you should implement the
suggested solution.

## Coding tasks

- Ask me clarifying questions until you're 95% confident you can complete the
  task successfully
- Before asking any questions, make sure you can't infer the answer from the
  codebase
- Ask your questions one by one; don't combine multiple questions into a list
- Strive to formulate binary questions that can be answered yes or no
- If a binary question is impossible, provide an enumerated list of options so
  the answer can be a number
- If you think a task is too complex to implement in one go, suggest splitting
  it into subtasks, provide the subtask graph, and recommend which one to start
  with
- Write for humans - readability and simplicity are essential
- Ask before making any optimization that makes the code more complicated or
  less readable

### 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:

- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

### 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes,
simplify.

### 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:

- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:

- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

### 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:

- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:

```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

## Rust code

After you're done with a Rust task, always run the following without asking a
permission:

Format the code:

```sh
cargo sort
cargo +nightly fmt
```

Make checks pass:

```sh
./.pre-commit.sh
```

### Deriving macros from external crates

- Use fully qualified paths, e.g. `serde::Deserialize`
- Put external macros after built-in ones
- Sort them if possible

## Git

- Don't look into files / folders included in `.gitignore`
- Never ask to stage or commit anything, but after finishing a coding task,
  suggest a commit message
