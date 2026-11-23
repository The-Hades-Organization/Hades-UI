// Gera crates/hades-ui/assets/icons/*.svg e crates/hades-ui/src/icons/set.rs a partir do painel web do Hades.
// Uso: node scripts/gerar-icones.mjs [~/.claude/skills/hades/kit/panel/ui]
// Fonte: o lucide-react vendorizado do painel (ISC) e os desenhos próprios de app.tsx (ICONES, Cerberus, Argos, Caronte).
// Os nomes são os `I.<Nome>` que o painel usa; onde o painel troca o lucide por desenho próprio, vale o próprio.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import vm from 'node:vm';

const ui = process.argv[2] ?? path.join(os.homedir(), '.claude/skills/hades/kit/panel/ui');
const raiz = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const saida = path.join(raiz, 'crates/hades-ui/assets/icons');
const conjunto = path.join(raiz, 'crates/hades-ui/src/icons/set.rs');

const el = (type, props, ...children) => ({ type, props: props ?? {}, children: children.flat() });
const fakeReact = { createElement: el, forwardRef: (f) => f, Fragment: 'frag' };

// o UMD grava em globalThis.LucideReact e lê o React do global "react"
const ctx = { react: fakeReact };
ctx.globalThis = ctx;
vm.runInNewContext(fs.readFileSync(path.join(ui, 'vendor/lucide-react.js'), 'utf8'), ctx);
const lucide = ctx.LucideReact;

const app = fs.readFileSync(path.join(ui, 'app.tsx'), 'utf8');
const ini = app.indexOf('const Cerberus');
const fim = app.indexOf('// mesma API do lucide');
const proprios = vm.runInNewContext(`${app.slice(ini, fim)}; ({ Cerberus, Argos, Caronte, ICONES })`, { React: fakeReact });
const mapa = app.slice(app.indexOf('const I = {'), app.indexOf('const cn ='));
const trocas = Object.fromEntries([...mapa.matchAll(/(\w+): PROPRIOS\.(\w+)/g)].map((m) => [m[1], m[2]]));

const usados = new Set();
for (const arq of fs.readdirSync(ui, { recursive: true })) {
  if (!String(arq).endsWith('.tsx')) continue;
  for (const m of fs.readFileSync(path.join(ui, arq), 'utf8').matchAll(/\bI\.([A-Z][A-Za-z0-9]*)/g)) usados.add(m[1]);
}

const attrs = (p) => Object.entries(p)
  .filter(([k, v]) => !['key', 'aria-hidden', 'width', 'height', 'xmlns'].includes(k) && v !== undefined)
  .map(([k, v]) => `${k === 'viewBox' ? k : k.replace(/[A-Z]/g, (c) => '-' + c.toLowerCase())}="${v}"`).join(' ');
const tag = (n) => `<${n.type} ${attrs(n.props)}/>`;
const TRACO = 'fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="square" stroke-linejoin="miter"';
const svg24 = (corpo) => `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" ${TRACO}>${corpo}</svg>\n`;

function proprio(nome) {
  const corpo = proprios.ICONES[nome].map((d) => d[0] === '~'
    ? `<path d="${d.slice(1)}" fill="currentColor" fill-opacity="0.28"/>` : `<path d="${d}"/>`).join('');
  return svg24(corpo);
}
function figura(f) {
  const n = f({});
  return `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" ${attrs(n.props)}>${n.children.map(tag).join('')}</svg>\n`;
}
function doLucide(nome) {
  const comp = lucide[nome];
  if (!comp) throw new Error(`lucide sem ${nome}`);
  const no = comp({}).props.iconNode;
  return svg24(no.map(([t, p]) => tag({ type: t, props: p })).join(''));
}

const kebab = (n) => n.replace(/([a-z0-9])([A-Z])/g, '$1-$2').replace(/([A-Za-z])([0-9])/g, '$1-$2').toLowerCase();
const nomes = [...usados].sort();
fs.rmSync(saida, { recursive: true, force: true });
fs.mkdirSync(saida, { recursive: true });
const linhas = [];
for (const nome of nomes) {
  let conteudo;
  let origem;
  if (trocas[nome]) { conteudo = proprio(trocas[nome]); origem = 'hades'; }
  else if (proprios[nome]) { conteudo = figura(proprios[nome]); origem = 'hades'; }
  else { conteudo = doLucide(nome); origem = 'lucide'; }
  const arquivo = kebab(nome) + '.svg';
  fs.writeFileSync(path.join(saida, arquivo), conteudo);
  linhas.push(`    ${nome} => "${arquivo}", ${origem};`);
}
fs.mkdirSync(path.dirname(conjunto), { recursive: true });
fs.writeFileSync(conjunto, `// Gerado por scripts/gerar-icones.mjs; não editar à mão.\nicon_set! {\n${linhas.join('\n')}\n}\n`);
fs.mkdirSync(path.join(raiz, 'crates/hades-ui/assets/brand'), { recursive: true });
fs.copyFileSync(path.join(ui, 'brand/hades-mark.svg'), path.join(raiz, 'crates/hades-ui/assets/brand/hades-mark.svg'));
console.log(`${nomes.length} ícones (lucide ${lucide.Trash2 ? 'v0.460.0' : '?'})`);
