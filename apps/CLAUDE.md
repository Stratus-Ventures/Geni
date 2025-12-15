# Memory

## Essentials
1. For our 'geni-site' & 'geni-web' apps we are using pnpm.
2. Use context7 mcp to access latest documentation
3. MCPs you have access to: polar, railway (for hosting and postgres), context7, better-auth, playwright.

## Style

[//]: # (## IOS)

## Web

**Scope:** SvelteKit + TypeScript.  
**Targets:** small functions (5–11 lines), single responsibility, strict typing (no `any`), clean module boundaries, minimal comments, auto-format.

### Principles
- One task per unit. Compose.
- One task per unit. Compose.
- Domain owns types + rules.
- Server code stays server.
- Formatter/linter decide style.

### Structure + Imports (hard rules)
- Every `src/lib/*` subfolder has `index.ts` re-exporting its public API.
- **Max import path:** `$lib/domain` / `$lib/server` / `$lib/components` / `$lib/adapters`. No deep imports.
- Barrels: **named exports only**, keep surface small, avoid cycles.

  src/lib/
  domain/ index.ts
  server/ index.ts
  adapters/ index.ts
  components/ index.ts

### Functions (appearance rules)
- 1 task per function. Prefer `verbNoun` names (`createUser`, `validateSignup`).
- Typical 5–11 lines. Split early.
- Early returns. Flat control flow. No nested pyramids.
- Pure by default. Side effects only in server/adapters.
- Exported functions: explicit return types. Internal helpers may infer.
- No “clever” one-liners. Readable > cute.

### TypeScript
- No `any`. No implicit `any`. `strict` on.
- No `unknown` in app flow. Parse/validate immediately → typed DTOs → domain types.
- Branching uses discriminated unions + exhaustive handling.
- Avoid `as` except inside parsing/validation helpers.

### SvelteKit
- Server-only: `+page.server.ts`, `+layout.server.ts`, `+server.ts`, `hooks.server.ts`, `$lib/server/**`.
- Client never imports `$lib/server/**` (types via `import type` only).
- `load`/actions orchestrate. Domain rules in `$lib/domain`.

### Components
- Routes/pages = containers (wire `data`, actions). `$lib/components` = presentational.
- Props typed. Pass view models, not raw payloads.
- Derived state stays derived. No god components.

### Formatting / Tooling
- Prettier owns formatting. ESLint owns correctness. Both enforced in CI.
- No quirky commas/styles. Defer to tools.

### Comments
- Default: **no comments**.
- Allowed: section banners only.
- If you need to “explain,” refactor into clearer code.

### Anti-patterns → fix
- Deep imports → export via `index.ts`.
- God handler → split parse/validate/execute/respond.
- Business logic in `load` → move to domain.
- Client imports server → move work server-side; return view model.

### Examples

**1) Barrels + max import path**

```typescript
    // src/lib/domain/index.ts
    export type { UserView } from "./user/types";
    export { validateSignup } from "./signup/validate";
    export type { SignupInput } from "./signup/types";
```

**2) Typed orchestration (small functions, clean flow)**

```typescript
    // routes/signup/+page.server.ts
    import { validateSignup, type SignupInput } from "$lib/domain";
    import { createUser, sendWelcomeEmail } from "$lib/server";

    export async function action(form: FormData): Promise<Response> {
      const input = parseSignup(form);
      validateSignup(input);
      const user = await createUser(input);
      await sendWelcomeEmail(user.email);
      return json({ id: user.id });
    }

    function parseSignup(form: FormData): SignupInput {
      return { email: req(form, "email"), password: req(form, "password") };
    }

    function req(form: FormData, k: string): string {
      const v = form.get(k);
      if (typeof v !== "string" || !v) throw new Error(`Missing ${k}`);
      return v;
    }
```

## Post feature implementation routine (WEB)
After implementing a feature fo the following: 
1. Check security with `/security-review`
2. run `pnpm check`, `pnpm lint`, 
3. Fix type error issues (or any others) 
4. Run `pnpm format`.