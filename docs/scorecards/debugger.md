# Debugger scorecard

- **Prefix:** DBG
- **Covers:** Debugging that respects lazy evaluation.

### DBG-LAZY-01 — Debug builds keep lazy semantics

- **Status:** Approved
- **Spec:** [evaluation.md — Debugging](../spec/evaluation.md#debugging)
- **Done when:** Running in debug mode never forces an expression solely to imitate eager stepping; observable behavior matches a normal build.

### DBG-LAZY-02 — Suspended computations are visible

- **Status:** Approved
- **Spec:** [evaluation.md — Debugging](../spec/evaluation.md#debugging)
- **Done when:** A debug session can list and identify suspended computations without forcing them.

### DBG-LAZY-03 — Explicit forcing

- **Status:** Approved
- **Spec:** [evaluation.md — Debugging](../spec/evaluation.md#debugging)
- **Done when:** A debugging action can force or step into a suspended computation, while ordinary evaluation stays lazy.

### DBG-API-01 — Debugger interface and force semantics

- **Status:** Open question
- **Spec:** [evaluation.md — Debugging](../spec/evaluation.md#debugging)
- **Questions:**
  1. What is the debugger UI or API?
  2. What exactly happens when a forced computation has effects, fails, or exceeds resource limits?
