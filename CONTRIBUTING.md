# Contributing

Changes to schemas are contract changes. Open a contract-change issue before broad schema edits unless the change is a narrow bug fix.

Before opening a pull request:

```sh
scripts/validate-all.sh
```

Include fixture changes for every behavioral schema change. If a schema change affects TypeScript output, regenerate locally and describe the generated output impact in the PR.

