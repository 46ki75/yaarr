# TypeScript ESLint Standard

Org-wide ESLint baseline for TypeScript repositories. The checked-in flat
configuration in each repository is the executable specification; this
reference defines the minimum policy that configuration must preserve.

## Principles

- Use ESLint flat config with `@eslint/js` recommended and
  `typescript-eslint` recommended type-checked rules.
- Enable typed linting through `parserOptions.projectService: true` and set
  `tsconfigRootDir`. Do not point `project` at one tsconfig; multi-context
  repositories need all relevant configs to be discovered.
- Apply the strict baseline below to all maintained TypeScript source, test,
  script, and configuration files covered by a tsconfig.
- Add a framework's recommended flat config where applicable. Framework rules
  supplement this baseline; they do not replace or weaken it.
- Expose lint as a leaf command and include it in the repository's canonical
  quality gate. Editor and pre-commit integrations provide faster feedback but
  are not authoritative.

## Dependencies

Install the baseline tools as development dependencies in the package that
owns the ESLint config:

```sh
pnpm add --save-dev eslint @eslint/js typescript typescript-eslint
```

Vitest projects also install its ESLint plugin locally:

```sh
pnpm add --save-dev @vitest/eslint-plugin
```

In a workspace, do not rely on an undeclared dependency resolving from the
workspace root. Each independently linted package declares the plugins that
its config imports.

## Required baseline

Use these rules and severities as the shared baseline. A monorepo should export
them from one shared config module rather than copying them into every package.

```js
export const strictRules = {
  eqeqeq: ["error", "always", { null: "ignore" }],
  curly: ["error", "all"],
  "consistent-return": "error",
  "default-case-last": "error",
  "guard-for-in": "error",
  radix: "error",
  "@typescript-eslint/ban-ts-comment": [
    "error",
    { minimumDescriptionLength: 10 },
  ],
  "@typescript-eslint/no-meaningless-void-operator": "error",
  "@typescript-eslint/no-misused-spread": "error",
  "@typescript-eslint/no-mixed-enums": "error",
  "@typescript-eslint/no-useless-default-assignment": "error",
  "@typescript-eslint/related-getter-setter-pairs": "error",
  "@typescript-eslint/return-await": [
    "error",
    "error-handling-correctness-only",
  ],
  "@typescript-eslint/switch-exhaustiveness-check": "error",
  "@typescript-eslint/use-unknown-in-catch-callback-variable": "error",

  // Keep unsafe boundaries and incomplete migrations visible without making
  // adoption depend on fixing all existing debt at once.
  "@typescript-eslint/no-base-to-string": "warn",
  "@typescript-eslint/no-unsafe-argument": "warn",
  "@typescript-eslint/no-unsafe-assignment": "warn",
  "@typescript-eslint/no-unsafe-call": "warn",
  "@typescript-eslint/no-unsafe-member-access": "warn",
  "@typescript-eslint/no-unsafe-return": "warn",
  "@typescript-eslint/require-await": "warn",
  "@typescript-eslint/unbound-method": "warn",
};

export const strictLinterOptions = {
  reportUnusedDisableDirectives: "error",
  reportUnusedInlineConfigs: "error",
};

export const typedTestRules = {
  "@typescript-eslint/await-thenable": "warn",
};
```

The warning-level rules are migration debt, not permission to introduce new
findings. Keep warnings visible in local and CI output. A repository with a
clean baseline should promote them to errors rather than reduce coverage.

`eqeqeq` deliberately permits `value == null` as the concise check for both
`null` and `undefined`. `return-await` uses
`error-handling-correctness-only` so lint requires `await` where it changes
`try`/`catch` or `finally` behavior without imposing it on every return.

## Flat config shape

Adapt file globs to the repository, but preserve the baseline, typed parser
configuration, and linter options:

```js
// eslint.config.mjs
import js from "@eslint/js";
import tseslint from "typescript-eslint";

import {
  strictLinterOptions,
  strictRules,
  typedTestRules,
} from "./eslint.strict.mjs";

export const typeScriptConfig = {
  files: ["**/*.{ts,tsx,mts,cts}"],
  extends: [js.configs.recommended, tseslint.configs.recommendedTypeChecked],
  linterOptions: strictLinterOptions,
  languageOptions: {
    parserOptions: {
      projectService: true,
      tsconfigRootDir: import.meta.dirname,
    },
  },
  rules: strictRules,
};

export default tseslint.config(typeScriptConfig);
```

Include build scripts and TypeScript configuration files only when an
appropriate tsconfig covers them. For repositories with browser, Node,
library, or test contexts, create the corresponding tsconfigs as described in
[`general.md`](general.md); let the project service select among them.

## Vitest

Vitest projects append a test-specific object to the same flat config. Add
`import vitest from "@vitest/eslint-plugin";` to `eslint.config.mjs`, then
replace its final export with:

```js
export default tseslint.config(
  typeScriptConfig,
  {
    files: ["**/*.{spec,test}.{ts,tsx}"],
    extends: [vitest.configs.recommended],
    rules: typedTestRules,
  },
);
```

Do not copy the preset's individual rules or disable violations in bulk;
extending the preset keeps the repository aligned with the installed plugin
version.

Keep `vitest/no-disabled-tests` at the preset's warning severity. The rest of
the recommended preset remains error-level unless an upstream release changes
the preset itself.

## Exceptions

Fix the code before suppressing a rule. When an integration boundary makes a
suppression necessary:

1. Scope it to the smallest expression or line.
2. Explain the concrete external constraint in the directive or adjacent
   comment.
3. Do not disable a rule for an entire package merely to reduce migration
   work.
4. Use a file-scoped override only for a stable category such as generated
   files, framework declarations, or test doubles, and document why the base
   rule is unsuitable there.

`@ts-ignore` and `@ts-nocheck` are not substitutes for an ESLint exception.
When `@ts-expect-error` is unavoidable, its required description must identify
the type-system or upstream-library limitation, not merely restate that an
error is expected.

## Enforcement

Expose ESLint as a package-level leaf command:

```json
{
  "scripts": {
    "lint": "eslint ."
  }
}
```

[`../nodejs/general.md`](../nodejs/general.md) owns package-script naming and
composition. The repository-level gate defined by
[`../general/executable-quality.md`](../general/executable-quality.md) includes
the lint leaf and is what CI invokes. In a workspace, that gate must discover
every independently maintained package.

Ensure the lint globs cover the same maintained TypeScript contexts that the
gate builds or tests; source-only linting is insufficient when scripts and test
files contain maintained TypeScript.
