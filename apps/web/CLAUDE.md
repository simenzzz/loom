# web

React 19 + TS + Vite. Search UI and (from P2) the live crawl dashboard.

Conventions:
- Every response from loom-server passes through `src/lib/validate.ts`
  (AJV against the shared contracts) before rendering
- Generated types in `src/contracts/gen` — do not edit
- State stays local/simple until the dashboard demands more; no state
  library until it visibly hurts
- `npm run lint` is `tsc --noEmit`; vitest for logic (fixture corpus runs
  here too)
