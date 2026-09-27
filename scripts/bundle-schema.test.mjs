import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFile } from 'node:fs/promises';
import { compile } from 'json-schema-to-typescript';
import { bundleSchema } from './bundle-schema.mjs';

test('an imported Task keeps the scope of its local occurrence definition', async () => {
  const base = 'https://schemas.ubunow.net/phase1/';
  const task = { $id: `${base}core/task.schema.json`, title: 'Task', type: 'object',
    properties: { occurrence: { $ref: '#/$defs/occurrence' } }, required: ['occurrence'],
    $defs: { occurrence: { type: 'object', properties: { local_date: { type: 'string' } }, required: ['local_date'] } } };
  const response = { $id: `${base}api/response.schema.json`, type: 'object',
    properties: { task: { $ref: task.$id }, own: { $ref: '#/$defs/occurrence' } },
    $defs: { occurrence: { type: 'number' } } };
  const original = structuredClone([task, response]);
  const bundled = bundleSchema(response, new Map([[task.$id, task], [response.$id, response]]));
  const resolve = ref => ref.slice(2).split('/').reduce((value, key) => value[key], bundled);
  assert.equal(resolve(bundled.properties.own.$ref).type, 'number');
  const imported = resolve(bundled.properties.task.$ref);
  assert.equal(resolve(imported.properties.occurrence.$ref).properties.local_date.type, 'string');
  const ts = await compile(bundled, 'Response');
  assert.match(ts, /local_date: string/);
  assert.deepEqual([task, response], original);
});

test('real next-action response bundles nested Task references entirely offline', async () => {
  const { readdir } = await import('node:fs/promises');
  const root = new URL('../schemas/', import.meta.url);
  const registry = new Map();
  async function load(dir) {
    for (const entry of await readdir(dir, { withFileTypes: true })) {
      const url = new URL(entry.name + (entry.isDirectory() ? '/' : ''), dir);
      if (entry.isDirectory()) await load(url);
      else if (entry.name.endsWith('.schema.json')) {
        const schema = JSON.parse(await readFile(url, 'utf8'));
        registry.set(schema.$id, schema);
      }
    }
  }
  await load(root);
  const schema = registry.get('https://schemas.ubunow.net/phase1/api/next-action-response.schema.json');
  const ts = await compile(bundleSchema(schema, registry), 'NextActionResponse');
  assert.match(ts, /occurrence/);
  assert.match(ts, /local_date/);
});
