#!/usr/bin/env python3
"""Render documentation-only diagram JSON using the standard library.

Run from any directory. Existing generated SVG/HTML/README are refreshed; source
JSON/Markdown and all application files are read-only. The HTML build uses the
repository's already-installed marked and Prettier packages via Node, without acquisition.
"""
import base64
import html
import json
import subprocess
import textwrap
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
W = 1560
PALETTE = {
    'native': ('#eef1fb', '#4758A2'), 'ui': ('#e7f3f5', '#326672'),
    'owner': ('#fff4db', '#94642c'), 'remote': ('#f8eaf3', '#934873'),
    'store': ('#e9f4eb', '#42784e'), 'foundation': ('#f1eff7', '#79628d'),
    'boundary': ('#f2f2f4', '#686b77'), 'deferred': ('#ffffff', '#79628d')}

def esc(value):
    return html.escape(str(value), quote=True)

def lines(value, width):
    return textwrap.wrap(value, width=width, break_long_words=False, break_on_hyphens=False)

def txt(x, y, value, size=16, weight=400, color='#182033', anchor='start'):
    return f'<text x="{x}" y="{y}" font-size="{size}" font-weight="{weight}" fill="{color}" text-anchor="{anchor}">{esc(value)}</text>'

def render(d):
    height = 188 + len(d['rows']) * 282 + 140
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {height}" role="img" aria-labelledby="title desc">',
           f'<title id="title">{esc(d["title"])}</title><desc id="desc">{esc(d["subtitle"] + ". " + d["note"])}</desc>',
           '<defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8 Z" fill="#4758A2"/></marker></defs>',
           f'<rect width="{W}" height="{height}" fill="#FBFBFB"/>',
           '<g font-family="Arial, Helvetica, sans-serif">',
           '<rect x="0" y="0" width="1560" height="112" fill="#252B71"/>',
           txt(40,47,d['title'],30,700,'#ffffff'),txt(40,81,d['subtitle'],19,400,'#ffffff')]
    legend = [('native','Native'),('ui','Interface'),('owner','Owner'),('remote','Remote'),('store','Stored'),('foundation','Separate foundation'),('deferred','Deferred / unverified')]
    x=40
    for kind, label in legend:
        fill,stroke=PALETTE[kind]
        dash=' stroke-dasharray="6 4"' if kind in ('foundation','deferred') else ''
        out.append(f'<rect x="{x}" y="132" width="18" height="18" fill="{fill}" stroke="{stroke}"{dash}/>')
        out.append(txt(x+27,147,label,15))
        x += 43+len(label)*8
    for ri,r in enumerate(d['rows']):
        y=190+ri*282
        out.append(txt(40,y,r['label'],20,700))
        count=len(r['nodes']); gap=100; bw=(W-80-gap*(count-1))/count; by=y+24; bh=204
        assert len(r['arrows'])==count-1
        for i,node in enumerate(r['nodes']):
            x=40+i*(bw+gap); fill,stroke=PALETTE[node['kind']]
            dash=' stroke-dasharray="7 4"' if node['kind'] in ('foundation','deferred') else ''
            out.append(f'<g data-component="{esc(node["id"])}"><rect x="{x}" y="{by}" width="{bw}" height="{bh}" rx="12" fill="{fill}" stroke="{stroke}" stroke-width="2"{dash}/>')
            out.append(txt(x+16,by+25,node['id'],14,700,stroke))
            yy=by+54
            for v in lines(node['title'],int((bw-32)/10.6)):
                out.append(txt(x+16,yy,v,19,700)); yy+=23
            yy+=10
            for v in lines(node['body'],int((bw-32)/8.3)):
                out.append(txt(x+16,yy,v,16)); yy+=21
            if yy > by+bh+3: raise ValueError(f'Clipping risk: {d["slug"]} {node["id"]} {yy-by}')
            out.append('</g>')
            if i<count-1:
                ax=x+bw; ay=by+104; label=r['arrows'][i]
                comparison=any(w in label for w in ('separate','distinct','no ','not ')) or d['slug']=='09-future'
                dashed=' stroke-dasharray="5 4"' if comparison else ''
                marker='' if comparison else ' marker-end="url(#arrow)"'
                out.append(f'<line x1="{ax+6}" y1="{ay}" x2="{ax+gap-9}" y2="{ay}" stroke="#4758A2" stroke-width="2"{marker}{dashed}/>')
                for j,v in enumerate(lines(label,12)):
                    out.append(txt(ax+gap/2,ay-43+j*17,v,13,600,'#252B71','middle'))
        out.append(txt(43,by+bh+23,'Evidence: '+', '.join(r['refs']),14,400,'#49516a'))
    yy=height-104
    for v in lines(d['note'],164):
        out.append(txt(40,yy,v,16)); yy+=22
    out.append(txt(40,height-22,'CORTEXA  /  Source bc12776412c7  /  2026-10-07  /  Solid arrows: stated flow; dashed lines: comparison or unwired boundary',14,400,'#49516a'))
    out.extend(['</g>','</svg>'])
    return '\n'.join(out)+'\n'

def evidence_md(d,sources):
    out=['| Components / connections | Source symbol and location | Evidence / limit |','| --- | --- | --- |']
    for row in d['rows']:
        refs=[]
        for key in row['refs']:
            s=sources[key];refs.append(f'[{key}: `{s["symbol_or_anchor"]}`]({s["url"]})')
        out.append('| '+row['label']+' | '+'; '.join(refs)+' | Code trace; '+('separate foundation / deferred' if d['slug']=='09-future' else 'see retained evidence below')+' |')
    return '\n'.join(out)+'\n\n'+d['evidence']+'\n'

def main():
    data=json.loads((HERE/'diagrams.json').read_text())
    source_data=json.loads((HERE/'sources.json').read_text()); sources=source_data['sources']
    for d in data['diagrams']:
        for row in d['rows']:
            for key in row['refs']: assert key in sources,key
        (HERE/(d['slug']+'.svg')).write_text(render(d))
    intro='''# Cortexa — current architecture and execution atlas

Snapshot: **2026-10-07**. Checkout `/Users/hdang/.codex/worktrees/live-provider-qa/ai-agent-assistant`,
branch `codex/live-provider-qa`, source commit `bc12776412c717613de1fdc42c38bf3312493726`.
Its product tree equals merged main `bbe7546281fec3c8b4b608d68a52bc9732d9ecd9` (PR137).
The checkout began clean; this increment adds documentation only. No application source,
profile, artifact or historical QA result was changed. This is an evidence snapshot,
not a claim that every runtime, account, model or workflow variant has been tested.

## Open the artifacts

- [Standalone viewer](viewer.html): all nine diagrams, full inventories, evidence, navigation and zoom; no network required.
- [PDF overview](overview.pdf): print overview of all nine views.
- [Capability inventory](capabilities.md): nine bots, six connections, settings, Knowledge, tools, graphs and lifecycle.
- [Security evidence table](security.md): enforcer, resource, source, verification and limitations.
- [External-project adoption matrix](external-projects.md): actual adoption versus evaluation and development guidance.
- [Concrete execution walkthrough](walkthrough.md): owner-approved Research collaboration and simpler single-bot route.
- [Editable diagram source](diagrams.json), [renderer](render_diagrams.py), [source/hash register](sources.json).

## Read the atlas

**Implemented and verified** means code plus the stated retained tests or native evidence,
not universal acceptance. **Implemented, not natively verified** has source/automated evidence
but no accepted live variant. **Partial** includes fixture-only UI or unwired foundation
contracts. **Deferred** describes separately approved future direction. **Unclear** means the
available evidence cannot establish the claim. Colors distinguish component boundaries;
status text and evidence qualify acceptance. Solid arrows carry the stated data. Dashed
lines compare separate capabilities or unwired boundaries and do not claim live flow.
Rows are independent paths unless explicitly numbered as a continuing sequence.

Stable IDs: **UI/RPC** are the frontend/native boundary; **CHAT/CTX/HOST** own single-bot
execution/context/events; **PROFILE** is saved bot configuration; **ADAPTER/API/CODEX/LOCAL/CLAUDE**
are connections; **COORD/ROOM_DB/SNAP/GRAPH** own collaboration and projections; **KNOW/DOCS/DB**
own documents and local persistence; **DIAG** owns sanitized operational evidence;
**FOUNDATION/POLICY/APPROVAL/TOOLS/MEMORY** are separate typed foundations. `BOT.*` IDs name
canonical roles, not additional provider implementations. Conductor is application
coordination/presentation, not a tenth model agent.

## Evidence register and limits

- [Retained live-provider review](../../reviews/2026-10-06-live-provider-e2e-post-increment-review.md):
  recorded full verification (105 hook, 94 repository, 601 frontend, 459 Rust-library,
  255 integration tests; inherited opt-in Hermes test ignored), focused repair tests,
  native API/Codex matrix and explicit limitations. Not rerun for documentation.
- [Inherited UI/UX review](../../reviews/2026-10-05-ui-ux-redesign-post-increment-review.md):
  layout/keyboard/typed native evidence. Later six-screen aesthetics are owner-reported,
  not a claim of complete ordinary workflow or all profile/provider coverage.
- Corrected Codex credential-store and saved-low A/B recheck:
  `/private/tmp/cortexa-codex-effort-recheck-3nvc11hq/FINAL-HANDOFF.md`.
  Native minimal request, discovery without Save, restart persistence and identity-bound
  cleanup passed. Backend timing-order claims remain automated, not proven by UI snapshots.
- [Post-merge Documentation](https://github.com/SillyRbbit/ai-agent-assistant/actions/runs/37649785081)
  and [CI](https://github.com/SillyRbbit/ai-agent-assistant/actions/runs/37649785034): six
  successful attempt-1 jobs at merged main, including Linux runner23 and macOS runner22.
  Earlier prerequisite failures remain historical failures.
- ECC installation/trial evidence remains external: installation handoff in
  `/Users/hdang/Developer/Cortexa-archive/2026-10-06_120320-ecc-xzfwc0cg/HANDOFF.md` and
  `/private/tmp/cortexa-ecc-review-confirmation-md8kmo8f/TRIAL.md`. Development-only,
  hooks/MCP disabled; no product runtime integration.
- New documentation evidence: `/private/tmp/cortexa-architecture-walkthrough-8sim6hio`.
  Source/hash checks, rendering and document validation are new; retained product tests,
  builds and QA are inherited. No credentials, personal databases or raw provider payloads
  inspected. Ledger33 unchanged; D-125/M1/M2 parked; all advisories/native workaround retained.

## Most important boundaries

Live bots generate text and structured handoffs; they do not execute arbitrary tools,
apply code, administer systems or browse by role name. Live routes are sequential. Native
policy/approval/tool/memory foundations are real tested code but not a generic live executor.
Simulation UI is separate from provider verification. Database storage is not proven encrypted;
`store=false` is not a retention guarantee. Codex restrictions are validated configuration,
not a complete OS sandbox audit. This atlas is not a complete security audit.

## Diagram guide

'''
    md=intro
    for d in data['diagrams']:
        md+=f'### {d["title"]}\n\n{d["subtitle"]}.\n\n![{d["title"]}]({d["slug"]}.svg)\n\n{d["note"]}\n\n'+evidence_md(d,sources)+'\n'
    md+='''## Regeneration

Use installed tooling only, from this checkout:

```bash
python3 -B docs/architecture/current/render_diagrams.py
```

The standard-library renderer refreshes only generated SVG/HTML/index files in this directory.
It uses the existing installed Node `marked` package for the offline HTML text. No acquisition,
application launch or provider call occurs. PDF is a separate Chromium print artifact; its
creation command and visual receipts are retained with this increment's external evidence.
JSON is the editable graph source. Change the factual source register and inventory together
with any future approved implementation; this frozen revision is not a moving architecture claim.
'''
    (HERE/'README.md').write_text(md)
    markdowns=[md]+[(HERE/name).read_text() for name in ('capabilities.md','security.md','external-projects.md','walkthrough.md')]
    js="const fs=require('fs');const {marked}=require('marked');const x=JSON.parse(fs.readFileSync(0,'utf8'));process.stdout.write(JSON.stringify(x.map(s=>marked.parse(s))));"
    rendered=json.loads(subprocess.run(['node','-e',js],input=json.dumps(markdowns),text=True,capture_output=True,check=True,cwd=ROOT).stdout)
    logo=base64.b64encode((ROOT/'assets/branding/logo-primary.png').read_bytes()).decode()
    css='''*{box-sizing:border-box}html{scroll-behavior:smooth}body{margin:0;background:#FBFBFB;color:#182033;font:17px/1.65 system-ui,sans-serif}a{color:#354b97}header{background:#252B71;color:white;padding:28px 36px;display:flex;align-items:center;gap:24px}header img{width:86px;height:86px;object-fit:contain}header h1{margin:0;font-size:30px}header p{margin:3px 0 0;color:#e2e7ff}nav{position:sticky;top:0;z-index:9;background:#fff;border-bottom:1px solid #cbd1e2;padding:12px 24px;display:flex;gap:12px;flex-wrap:wrap}nav select,button{font:inherit;border:1px solid #8994b1;border-radius:6px;background:white;padding:7px 12px;color:#182033}nav label{display:flex;align-items:center;gap:8px}button{cursor:pointer}button:focus-visible,select:focus-visible,a:focus-visible{outline:3px solid #4758A2;outline-offset:3px}main{max-width:1580px;margin:auto;padding:24px}section{scroll-margin-top:90px;margin:0 0 40px;padding:28px;border:1px solid #d7dce7;border-radius:12px;background:white}h2{color:#252B71;margin-top:0}h3{color:#354475}p,li{max-width:110ch}table{border-collapse:collapse;width:100%;font-size:15px;margin:18px 0;table-layout:fixed}th,td{padding:10px;border:1px solid #cbd1e2;text-align:left;vertical-align:top;overflow-wrap:anywhere}th{background:#eef1fb}code{font-size:.9em;overflow-wrap:anywhere}pre{overflow:auto;background:#f1f3f8;padding:18px;border-radius:8px;white-space:pre-wrap}.viewport{overflow:auto;background:#FBFBFB;border:1px solid #d6dbe6;border-radius:8px}.viewport svg{display:block;width:100%;min-width:1000px;height:auto}.note{border-left:5px solid #4758A2;background:#f0f3fa;padding:14px 18px}.evidence{font-size:15px}.cover{display:none}.text-doc img{max-width:100%}.status{color:#334b59;font-weight:600}footer{padding:25px;text-align:center;color:#4e5870}@media(max-width:850px){main{padding:10px}section{padding:15px}header{padding:20px}header h1{font-size:23px}table{font-size:13px}nav{position:relative}section{scroll-margin-top:10px}}@media print{@page{size:A3 portrait;margin:14mm}body{font-size:13px}header,nav,footer,.text-doc,.diagram .evidence,.diagram>h2,.diagram>.subtitle,.diagram>.note{display:none!important}.cover{display:block;page-break-after:always;padding:60px 20px}.cover h1{font-size:44px;color:#252B71}.cover img{width:150px}.cover p{font-size:20px;line-height:1.7}main{padding:0;max-width:none}section.diagram{border:0;padding:0;margin:0;page-break-after:always;break-inside:avoid}.viewport{border:0;overflow:visible}.viewport svg{min-width:0;width:100%!important;max-height:1430px}section.diagram:last-of-type{page-break-after:auto}}'''
    nav=''.join(f'<option value="{d["slug"]}">{esc(d["title"])}</option>' for d in data['diagrams'])
    docs=[('guide','Evidence and guide',rendered[0]),('capabilities','Capability inventory',rendered[1]),('security','Security evidence',rendered[2]),('external','External projects',rendered[3]),('walkthrough','Concrete walkthrough',rendered[4])]
    nav+=''.join(f'<option value="{id}">{title}</option>' for id,title,_ in docs)
    contents=''
    for d in data['diagrams']:
        rows=''
        for row in d['rows']:
            refs='<br>'.join(f'<a href="{esc(sources[k]["url"])}">{esc(k+": "+sources[k]["path"]+":"+str(sources[k]["line"]))}</a> · <code>{esc(sources[k]["symbol_or_anchor"])}</code>' for k in row['refs'])
            rows+=f'<tr><td>{esc(row["label"])}</td><td>{refs}</td></tr>'
        contents+=f'<section class="diagram" id="{d["slug"]}"><h2>{esc(d["title"])}</h2><p class="subtitle">{esc(d["subtitle"])}</p><div class="viewport">{render(d)}</div><p class="note">{esc(d["note"])}</p><div class="evidence"><h3>Evidence for this view</h3><table><thead><tr><th>Connection / components</th><th>Source and symbol</th></tr></thead><tbody>{rows}</tbody></table><p>{esc(d["evidence"])}</p></div></section>'
    # The guide's image references become the same inline SVGs, keeping the viewer standalone.
    for id,title,body in docs:
        for d in data['diagrams']:
            body=body.replace(f'<img src="{d["slug"]}.svg" alt="{esc(d["title"])}">','<a href="#'+d['slug']+'">Open diagram above</a>')
        # Local document references navigate to embedded content; explicit external references remain links.
        for filename,target in [('viewer.html','01-system'),('capabilities.md','capabilities'),('security.md','security'),('external-projects.md','external'),('walkthrough.md','walkthrough')]:
            body=body.replace(f'href="{filename}"',f'href="#{target}"')
        contents+=f'<section class="text-doc" id="{id}"><h2>{title}</h2>{body}</section>'
    script='''const choice=document.querySelector('#view');let zoom=1;choice.addEventListener('change',()=>document.getElementById(choice.value).scrollIntoView());function setZoom(n){zoom=Math.max(.75,Math.min(2,n));document.querySelectorAll('.viewport svg').forEach(s=>s.style.width=(zoom*100)+'%');document.querySelector('#zoom').textContent=Math.round(zoom*100)+'%'}document.querySelector('#plus').onclick=()=>setZoom(zoom+.25);document.querySelector('#minus').onclick=()=>setZoom(zoom-.25);document.querySelector('#reset').onclick=()=>setZoom(1);document.querySelector('#print').onclick=()=>window.print();'''
    page=f'''<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="description" content="Source-bound Cortexa architecture atlas, 2026-10-07"><title>Cortexa — Architecture Atlas</title><style>{css}</style></head><body><header><img alt="Approved Cortexa logo" src="data:image/png;base64,{logo}"><div><h1>Cortexa · Architecture Atlas</h1><p>Implemented paths, trust boundaries and retained evidence · 7 October 2026</p><p>Source bc12776412c7 · equivalent merged main bbe7546281fe</p></div></header><nav aria-label="Atlas controls"><label>View <select id="view">{nav}</select></label><button id="minus" aria-label="Zoom out">−</button><output id="zoom" aria-live="polite">100%</output><button id="plus" aria-label="Zoom in">+</button><button id="reset">Reset zoom</button><button id="print">Print overview</button><a href="#guide">Evidence guide</a></nav><div class="cover"><img alt="Cortexa" src="data:image/png;base64,{logo}"><h1>Cortexa<br>Architecture and execution</h1><p>Current implementation · 7 October 2026<br>Source bc12776412c717613de1fdc42c38bf3312493726<br>Merged main bbe7546281fec3c8b4b608d68a52bc9732d9ecd9</p><p>Nine connected views. Solid arrows carry labeled data; dashed lines mark separate foundations or unimplemented directions. Read the companion HTML for full capability, security and adoption evidence.</p><p>Native acceptance covers the recorded API/Codex variants. Other implementations, fixtures and future work are explicitly distinguished. No product tests, builds or provider calls were repeated for this atlas.</p><p>Ledger33 unchanged · ECC hooks/MCP disabled · D-125/M1/M2 parked.<br>This documentation is not a complete security audit.</p></div><main>{contents}</main><footer>Documentation only · preserved source and historical evidence · offline standalone viewer</footer><script>{script}</script></body></html>'''
    (HERE/'viewer.html').write_text(page)
    format_paths = ['README.md', 'viewer.html', 'diagrams.json', 'sources.json', 'capabilities.md', 'security.md', 'external-projects.md', 'walkthrough.md']
    subprocess.run(['node', str(ROOT/'node_modules/prettier/bin/prettier.cjs'), '--write'] + [str(HERE/name) for name in format_paths], check=True, cwd=ROOT, stdout=subprocess.DEVNULL)
    print(json.dumps({'diagrams':len(data['diagrams']),'source_entries':len(sources),'standalone_viewer':True}))

if __name__=='__main__':
    main()
