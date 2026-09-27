// Bundle registered documents without losing the document owning a local ref.
function pointerGet(value, pointer) {
  if (!pointer || pointer === '#') return value;
  if (!pointer.startsWith('#/')) throw new Error(`Unsupported JSON pointer fragment: ${pointer}`);
  return pointer.slice(2).split('/').reduce((current, token) => {
    const key = token.replace(/~1/g, '/').replace(/~0/g, '~');
    if (current == null || !(key in current)) throw new Error(`Unresolvable JSON pointer fragment: ${pointer}`);
    return current[key];
  }, value);
}

export function bundleSchema(rootSchema, registry) {
  const root = structuredClone(rootSchema);
  const definitions = {};
  const keys = new Map();
  const used = new Set(Object.keys(root.$defs ?? {}));
  function rewrite(node, owner) {
    if (Array.isArray(node)) {
      node.forEach(value => rewrite(value, owner));
      return;
    }
    if (!node || typeof node !== 'object') return;
    if (typeof node.$ref === 'string') {
      const ref = node.$ref;
      const [base, fragment = ''] = ref.startsWith('#')
        ? [owner.$id, ref.slice(1)] : ref.split('#');
      const document = base === rootSchema.$id ? rootSchema : registry.get(base);
      if (!document) throw new Error(`Unregistered schema $id in $ref: ${ref}`);
      const uri = `${base}#${fragment}`;
      let key = keys.get(uri);
      if (!key) {
        const stem = uri.replace(/^https:\/\/schemas\.ubunow\.net\/phase1\//, '')
          .replace(/\.schema\.json/g, '').replace(/[^A-Za-z0-9]+/g, '_').replace(/^_+|_+$/g, '') || 'root';
        key = stem;
        for (let suffix = 1; used.has(key); suffix++) key = `${stem}_${suffix}`;
        keys.set(uri, key);
        used.add(key);
        // Reserve the key before descending, so recursive references terminate.
        definitions[key] = structuredClone(pointerGet(document, `#${fragment}`));
        rewrite(definitions[key], document);
      }
      node.$ref = `#/$defs/${key}`;
    }
    for (const [key, value] of Object.entries(node)) {
      if (key !== '$ref') rewrite(value, owner);
    }
  }
  // Rewrite the original definitions too; do not replace them with stale clones.
  rewrite(root, rootSchema);
  if (Object.keys(definitions).length) root.$defs = { ...root.$defs, ...definitions };
  return root;
}
