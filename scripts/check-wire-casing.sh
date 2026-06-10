#!/usr/bin/env bash
set -euo pipefail

node --input-type=module <<'NODE'
import { promises as fs } from 'node:fs';
import path from 'node:path';

const repoRoot = process.cwd();
const camelCasePattern = /[a-z][A-Z]/;
const failures = [];

async function walk(dir, predicate) {
  const entries = await fs.readdir(dir, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...await walk(fullPath, predicate));
    } else if (entry.isFile() && predicate(fullPath)) {
      files.push(fullPath);
    }
  }

  return files.sort();
}

function pointerSegment(segment) {
  return String(segment).replace(/~/g, '~0').replace(/\//g, '~1');
}

function formatLocation(file, pointer) {
  const relative = path.relative(repoRoot, file);
  return pointer.length === 0
    ? relative
    : `${relative}#/${pointer.map(pointerSegment).join('/')}`;
}

async function readJson(file) {
  const content = await fs.readFile(file, 'utf8');
  return JSON.parse(content);
}

function checkName(kind, file, pointer, name) {
  if (camelCasePattern.test(name)) {
    failures.push(`${kind}: ${formatLocation(file, pointer)} -> ${name}`);
  }
}

function inspectSchemaProperties(node, file, pointer = []) {
  if (Array.isArray(node)) {
    node.forEach((item, index) => inspectSchemaProperties(item, file, [...pointer, index]));
    return;
  }

  if (!node || typeof node !== 'object') {
    return;
  }

  if (node.properties && typeof node.properties === 'object' && !Array.isArray(node.properties)) {
    for (const name of Object.keys(node.properties)) {
      checkName('schema property', file, [...pointer, 'properties', name], name);
    }
  }

  for (const [key, value] of Object.entries(node)) {
    inspectSchemaProperties(value, file, [...pointer, key]);
  }
}

function inspectFixtureKeys(node, file, pointer = []) {
  if (Array.isArray(node)) {
    node.forEach((item, index) => inspectFixtureKeys(item, file, [...pointer, index]));
    return;
  }

  if (!node || typeof node !== 'object') {
    return;
  }

  for (const [key, value] of Object.entries(node)) {
    checkName('fixture key', file, [...pointer, key], key);
    inspectFixtureKeys(value, file, [...pointer, key]);
  }
}

async function inspectGeneratedTypeScript(file) {
  let content;
  try {
    content = await fs.readFile(file, 'utf8');
  } catch (error) {
    if (error && error.code === 'ENOENT') {
      failures.push(`generated TypeScript: ${path.relative(repoRoot, file)} is missing; run scripts/generate-typescript.sh first`);
      return;
    }
    throw error;
  }

  const propertyPattern = /^\s{2,}(?:([A-Za-z_$][A-Za-z0-9_$]*)|["']([^"']+)["'])\??:/;
  content.split(/\r?\n/).forEach((line, index) => {
    const match = line.match(propertyPattern);
    if (!match) {
      return;
    }

    const name = match[1] ?? match[2];
    checkName('generated TypeScript property', file, [index + 1], name);
  });
}

const schemaFiles = await walk(path.join(repoRoot, 'schemas'), (file) => file.endsWith('.schema.json'));
for (const file of schemaFiles) {
  inspectSchemaProperties(await readJson(file), file);
}

const fixtureFiles = await walk(path.join(repoRoot, 'fixtures'), (file) => file.endsWith('.json'));
for (const file of fixtureFiles) {
  inspectFixtureKeys(await readJson(file), file);
}

await inspectGeneratedTypeScript(path.join(repoRoot, 'generated', 'typescript', 'index.d.ts'));

if (failures.length > 0) {
  console.error('CamelCase UbU wire fields found:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log(`wire casing ok: ${schemaFiles.length} schemas, ${fixtureFiles.length} fixtures, generated TypeScript`);
NODE
