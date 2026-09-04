# Personal Assistant V0 fixed local engine and artifact evidence post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git rev-parse HEAD main origin/main",
    "git rev-list --left-right --count main...origin/main",
    "git diff --name-only",
    "git ls-files --others --exclude-standard",
    "git switch -c codex/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan",
    "python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-fixed-local-engine-artifact-evidence-plan",
    "node --version",
    "npm --version",
    "rustc --version",
    "cargo --version",
    "sw_vers",
    "uname -m",
    "./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PRODUCT_REQUIREMENTS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md docs/PROJECT_DIRECTION.md docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md docs/reviews/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan-post-increment-review.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "npm run verify",
    "git diff --check",
    "python3 -c 'from pathlib import Path; import subprocess; baseline=subprocess.check_output([\"git\",\"show\",\"355d42ac8bb9a5ed663f57895675df688507ce9f:DECISIONS.md\"]); current=Path(\"DECISIONS.md\").read_bytes(); plan=Path(\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\").read_text(); block=plan.split(\"```markdown\\n\",1)[1].split(\"\\n```\",1)[0].encode(); suffix=current[len(baseline):].strip(); print(f\"prefix={current.startswith(baseline)} exact={suffix==block} d121={suffix.count(b\\\"## D-121 -\\\")}\")'",
    "python3 -c 'from pathlib import Path; paths=(\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\"); ok=True; summary=[]; exec(\"for path in paths:\\n text=Path(path).read_text()\\n tuple_part=text.split(\\\"Evidence-resolved tuple\\\",1)[1].split(\\\"Conjunctive static evidence matrix\\\",1)[0]\\n matrix_part=text.split(\\\"Conjunctive static evidence matrix\\\",1)[1].split(\\\"Matrix totals\\\",1)[0]\\n values=(tuple_part.count(\\\"\\`documented\\`\\\"),tuple_part.count(\\\"\\`contract_unproven\\`\\\"),tuple_part.count(\\\"\\`not_run\\`\\\"),matrix_part.count(\\\"\\`documented\\`\\\"),matrix_part.count(\\\"\\`contract_unproven\\`\\\"),matrix_part.count(\\\"\\`not_run\\`\\\"),matrix_part.count(\\\"\\`boundary_failed\\`\\\"))\\n summary.append((path,values))\\n ok = ok and values==(1,8,1,0,14,1,0)\"); print(summary); raise SystemExit(0 if ok else 1)'",
    "python3 -c 'from pathlib import Path; import subprocess; baseline=subprocess.check_output([\"git\",\"show\",\"355d42ac8bb9a5ed663f57895675df688507ce9f:DECISIONS.md\"]); current=Path(\"DECISIONS.md\").read_bytes(); profiles=(b\"local_no_auth\",b\"google_gemini_oauth\",b\"google_gemini_api_key\",b\"direct_openai_api_key\",b\"direct_openai_workload_identity\",b\"azure_openai_entra\",b\"azure_openai_api_key\",b\"anthropic_api_key\",b\"mistral_api_key\",b\"aws_bedrock_identity\"); blockers=(b\"opaque_prebound_identity_contract\",b\"exact_signer_binding_contract\",b\"account_keychain_scope_contract\",b\"private_key_nonexport_contract\",b\"fixed_algorithm_contract\",b\"interaction_denial_contract\",b\"hard_deadline_cancellation_contract\",b\"late_result_rejection_contract\",b\"cleanup_quarantine_contract\",b\"platform_effect_contract\"); live=(\"HANDOFF.md\",\"NEXT_STEPS.md\",\"PLANS.md\",\"PROJECT_STATUS.md\",\"ROADMAP.md\",\"SECURITY.md\"); ok=current.startswith(baseline) and all(current.count(x)==baseline.count(x) for x in profiles+blockers) and all(b in baseline for b in (b\"D-113\",b\"D-114\",b\"D-115\",b\"D-116\",b\"D-117\",b\"candidate_blocked\",b\"no_eligible_client\")) and all(\"candidate_not_eligible_or_unproven\" in Path(p).read_text() and \"Blocked\" in Path(p).read_text() for p in live); print(\"profiles=10 blockers=10 proposed=5 blocked_live=6\"); raise SystemExit(0 if ok else 1)'",
    "python3 -c 'from pathlib import Path; import sys; sys.path.insert(0,\".codex/hooks\"); from common import changed_paths; expected=frozenset((\"ARCHITECTURE.md\",\"CHANGELOG.md\",\"DECISIONS.md\",\"HANDOFF.md\",\"NEXT_STEPS.md\",\"PLANS.md\",\"PRODUCT_REQUIREMENTS.md\",\"PROJECT_STATUS.md\",\"ROADMAP.md\",\"SECURITY.md\",\"SECURITY_CHECKLIST.md\",\"TESTING_GUIDE.md\",\"docs/PROJECT_DIRECTION.md\",\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"docs/reviews/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan-post-increment-review.md\")); actual=frozenset(changed_paths(Path.cwd())); print(\"\\n\".join(sorted(actual))); raise SystemExit(0 if actual == expected else 1)'",
    "python3 -c 'from pathlib import Path; import subprocess; baseline=subprocess.check_output([\"git\",\"show\",\"355d42ac8bb9a5ed663f57895675df688507ce9f:DECISIONS.md\"]); current=Path(\"DECISIONS.md\").read_bytes(); plan=Path(\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\").read_text(); block=plan.split(\"```markdown\\n\",1)[1].split(\"\\n```\",1)[0].encode(); suffix=current[len(baseline):].strip(); prefix=current.startswith(baseline); exact=suffix==block; count=suffix.count(b\"## D-121 -\"); print(\"prefix={} exact={} d121={}\".format(prefix,exact,count)); raise SystemExit(0 if prefix and exact and count==1 and b\"## D-122 -\" not in suffix else 1)'",
    "python3 -c 'from pathlib import Path; specs=((\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"#### Matching evidence-resolved tuple\",\"#### Completed conjunctive static evidence matrix\"),(\"docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"### Evidence-resolved tuple\",\"### Conjunctive static evidence matrix\")); sections=[(text.split(ts,1)[1].split(ms,1)[0],text.split(ms,1)[1].split(\"Matrix totals\",1)[0]) for path,ts,ms in specs for text in [Path(path).read_text()]]; rows=[([line for line in t.splitlines() if line.startswith(\"|\") and any(s in line for s in (\"`documented`\",\"`contract_unproven`\",\"`not_run`\",\"`boundary_failed`\"))],[line for line in m.splitlines() if line.startswith(\"|\") and any(s in line for s in (\"`documented`\",\"`contract_unproven`\",\"`not_run`\",\"`boundary_failed`\"))]) for t,m in sections]; values=[(sum(\"`documented`\" in line for line in tr),sum(\"`contract_unproven`\" in line for line in tr),sum(\"`not_run`\" in line for line in tr),sum(\"`documented`\" in line for line in mr),sum(\"`contract_unproven`\" in line for line in mr),sum(\"`not_run`\" in line for line in mr),sum(\"`boundary_failed`\" in line for line in mr)) for tr,mr in rows]; print(values); raise SystemExit(0 if values==[(1,8,1,0,14,1,0),(1,8,1,0,14,1,0)] else 1)'",
    "git diff --exit-code 355d42ac8bb9a5ed663f57895675df688507ce9f -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json AGENTS.md CODE_REVIEW.md ENGINEERING_GUIDE.md TROUBLESHOOTING_LOG.md",
    "python3 -c 'from pathlib import Path; import hashlib,subprocess; paths=(\"docs/plans/2026-08-28-personal-assistant-v0-program.md\",\"docs/plans/2026-08-28-personal-assistant-v0-real-prompt-activation.md\",\"docs/plans/2026-08-28-personal-assistant-v0-https-dependency-decision.md\",\"docs/increments/personal-assistant-v0-https-dependency-decision.md\",\"docs/reviews/2026-09-03-personal-assistant-v0-https-dependency-decision-post-increment-review.md\",\"docs/plans/2026-09-03-personal-assistant-v0-selectable-connection-profile-architecture-decision.md\",\"docs/increments/pa-v0-selectable-connection-profile-decision.md\",\"docs/reviews/2026-09-03-pa-v0-selectable-connection-profile-decision-post-increment-review.md\",\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-private-lane-decision.md\",\"docs/increments/pa-v0-fixed-local-private-lane-decision.md\",\"docs/reviews/2026-09-03-pa-v0-fixed-local-private-lane-decision-post-increment-review.md\",\"docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md\",\"docs/increments/personal-assistant-v0-key-use-containment-classification.md\",\"docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md\",\"docs/plans/2026-09-03-personal-assistant-v0-pr112-publication-closeout.md\",\"docs/increments/personal-assistant-v0-pr112-publication-closeout.md\",\"docs/reviews/2026-09-03-personal-assistant-v0-pr112-publication-closeout-post-increment-review.md\"); pairs=[(p,hashlib.sha256(subprocess.check_output([\"git\",\"show\",\"355d42ac8bb9a5ed663f57895675df688507ce9f:\"+p])).hexdigest(),hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in paths]; print(\"\\n\".join(\"{} {} {}\".format(*row) for row in pairs)); raise SystemExit(0 if all(a==b for _,a,b in pairs) else 1)'",
    "python3 -c 'from pathlib import Path; import subprocess; baseline=subprocess.check_output([\"git\",\"show\",\"355d42ac8bb9a5ed663f57895675df688507ce9f:DECISIONS.md\"]); current=Path(\"DECISIONS.md\").read_bytes(); d119=baseline.split(b\"## D-119 -\",1)[1].split(b\"## D-120 -\",1)[0]; profiles=(b\"local_no_auth\",b\"google_gemini_oauth\",b\"google_gemini_api_key\",b\"direct_openai_api_key\",b\"direct_openai_workload_identity\",b\"azure_openai_entra\",b\"azure_openai_api_key\",b\"anthropic_api_key\",b\"mistral_api_key\",b\"aws_bedrock_identity\"); blockers=(b\"opaque_prebound_identity_contract\",b\"exact_signer_binding_contract\",b\"account_keychain_scope_contract\",b\"private_key_nonexport_contract\",b\"fixed_algorithm_contract\",b\"interaction_denial_contract\",b\"hard_deadline_cancellation_contract\",b\"late_result_rejection_contract\",b\"cleanup_quarantine_contract\",b\"platform_effect_contract\"); live=(\"HANDOFF.md\",\"NEXT_STEPS.md\",\"PLANS.md\",\"PROJECT_STATUS.md\",\"ROADMAP.md\",\"SECURITY.md\"); ok=current.startswith(baseline) and all(d119.count(x)==1 for x in profiles) and b\"Every entry is `candidate_blocked`\" in d119 and b\"D-113 through D-117 remain Proposed and non-controlling\" in d119 and all(x in baseline for x in blockers) and all(\"candidate_not_eligible_or_unproven\" in Path(p).read_text() and \"Blocked\" in Path(p).read_text() for p in live); print(\"d119_profiles=10 candidate_blocked=true blockers=10 proposed_noncontrolling=true blocked_live=6\"); raise SystemExit(0 if ok else 1)'",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 -c 'from pathlib import Path; import sys; sys.path.insert(0,\".codex/hooks\"); import post_increment_gate as gate; manifest, report, digest = gate.validate_report(Path.cwd(), \"docs/reviews/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan-post-increment-review.md\", \"personal-assistant-v0-fixed-local-engine-artifact-evidence-plan\"); print(manifest[\"quality_gate\"]); print(manifest[\"next_increment_readiness\"]); print(len(manifest[\"files_changed\"])); print(digest)'",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment personal-assistant-v0-fixed-local-engine-artifact-evidence-plan --report docs/reviews/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan-post-increment-review.md",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "docs/PROJECT_DIRECTION.md",
    "docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md",
    "docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md",
    "docs/reviews/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Large",
      "milestone": "Before any replacement candidate, artifact acquisition, target-Mac execution, source/dependency change, or operational local-v2 successor",
      "risk": "Pinned core retains reachable public dynamic loading; fourteen mandatory rows remain unproved and target-Mac evidence is not run. Treating this documentation result as admission would bypass artifact, no-egress/native-TCB, lifecycle, authentication, and target-platform gates.",
      "severity": "Advisory",
      "summary": "The frozen llama.cpp/Qwen candidate is candidate_not_eligible_or_unproven; it remains unavailable and every operational successor remains Blocked."
    }
  ],
  "increment_id": "personal-assistant-v0-fixed-local-engine-artifact-evidence-plan",
  "manual_verification": [
    {
      "check": "Owner approved the exact Ready plan, branch, gate, sixteen-file ceiling, and candidate-nomination checkpoint; the gate began exactly once",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Owner accepted the exact immutable candidate envelope and separately authorized the frozen-allowlist static assessment without a second begin",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Owner accepted the matching tuple, completed fifteen-row matrix, sole candidate_not_eligible_or_unproven disposition, and exact D-121 wording before final reconciliation",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Tuple totals are exactly one documented, eight contract_unproven, and one not_run; matrix totals are exactly zero documented, fourteen contract_unproven, one target-Mac not_run, and zero boundary_failed",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "D-121 is one append-only byte-exact copy of the owner-accepted plan block and all preceding DECISIONS.md bytes are preserved",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Independent architecture, security, documentation-sync, code-health, technical-debt, quality, and readiness re-review accepted the corrected exact sixteen-file result",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Repository-pinned toolchains were observed as Node 26.3.0, npm 11.16.0, rustc 1.90.0, and cargo 1.90.0 on macOS 26.6 build 25G72 arm64",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "One frozen Qwen README refresh returned HTTP 429; no workaround or alternate source was used and affected claims remained contract_unproven",
      "required": false,
      "status": "Failed"
    },
    {
      "check": "The first ad hoc exact-D-121 assertion had invalid Python f-string escaping; the corrected equivalent assertion passed",
      "required": false,
      "status": "Failed"
    },
    {
      "check": "The first tuple/matrix assertion split on a prose occurrence instead of the exact table headings; the corrected exact-heading assertion passed",
      "required": false,
      "status": "Failed"
    },
    {
      "check": "The first catalog preservation assertion incorrectly counted D-121's new local_no_auth preservation reference as D-119 mutation; the corrected D-119-section assertion passed",
      "required": false,
      "status": "Failed"
    },
    {
      "check": "Initial independent closeout review found pending header chronology and ambiguous generic Not-run wording; bounded same-increment corrections resolved both findings",
      "required": false,
      "status": "Failed"
    },
    {
      "check": "The first sandboxed finalizer attempt returned exit 4 because the ignored gate state was not writable; it changed no gate state, and the exact command succeeded with the required local filesystem permission",
      "required": false,
      "status": "Failed"
    },
    {
      "check": "The prescribed Hermes real-executable version probe remained intentionally ignored because it requires explicit opt-in and an operator-supplied pinned executable",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Source archives, packages, binaries, model bytes, artifact redirects, dependency resolution, candidate build/load/inference, product/model-artifact filesystem operations, target-Mac inspection, credentials, providers, signing, product systems, and operational external systems",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Owner acceptance review, commit, push, merge, replacement-candidate selection, and successor start",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -c 'from pathlib import Path; import sys; sys.path.insert(0,\".codex/hooks\"); from common import changed_paths; expected=frozenset((\"ARCHITECTURE.md\",\"CHANGELOG.md\",\"DECISIONS.md\",\"HANDOFF.md\",\"NEXT_STEPS.md\",\"PLANS.md\",\"PRODUCT_REQUIREMENTS.md\",\"PROJECT_STATUS.md\",\"ROADMAP.md\",\"SECURITY.md\",\"SECURITY_CHECKLIST.md\",\"TESTING_GUIDE.md\",\"docs/PROJECT_DIRECTION.md\",\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"docs/reviews/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan-post-increment-review.md\")); actual=frozenset(changed_paths(Path.cwd())); print(\"\\n\".join(sorted(actual))); raise SystemExit(0 if actual == expected else 1)'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -c 'from pathlib import Path; import subprocess; baseline=subprocess.check_output([\"git\",\"show\",\"355d42ac8bb9a5ed663f57895675df688507ce9f:DECISIONS.md\"]); current=Path(\"DECISIONS.md\").read_bytes(); plan=Path(\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\").read_text(); block=plan.split(\"```markdown\\n\",1)[1].split(\"\\n```\",1)[0].encode(); suffix=current[len(baseline):].strip(); prefix=current.startswith(baseline); exact=suffix==block; count=suffix.count(b\"## D-121 -\"); print(\"prefix={} exact={} d121={}\".format(prefix,exact,count)); raise SystemExit(0 if prefix and exact and count==1 and b\"## D-122 -\" not in suffix else 1)'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -c 'from pathlib import Path; specs=((\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"#### Matching evidence-resolved tuple\",\"#### Completed conjunctive static evidence matrix\"),(\"docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md\",\"### Evidence-resolved tuple\",\"### Conjunctive static evidence matrix\")); sections=[(text.split(ts,1)[1].split(ms,1)[0],text.split(ms,1)[1].split(\"Matrix totals\",1)[0]) for path,ts,ms in specs for text in [Path(path).read_text()]]; rows=[([line for line in t.splitlines() if line.startswith(\"|\") and any(s in line for s in (\"`documented`\",\"`contract_unproven`\",\"`not_run`\",\"`boundary_failed`\"))],[line for line in m.splitlines() if line.startswith(\"|\") and any(s in line for s in (\"`documented`\",\"`contract_unproven`\",\"`not_run`\",\"`boundary_failed`\"))]) for t,m in sections]; values=[(sum(\"`documented`\" in line for line in tr),sum(\"`contract_unproven`\" in line for line in tr),sum(\"`not_run`\" in line for line in tr),sum(\"`documented`\" in line for line in mr),sum(\"`contract_unproven`\" in line for line in mr),sum(\"`not_run`\" in line for line in mr),sum(\"`boundary_failed`\" in line for line in mr)) for tr,mr in rows]; print(values); raise SystemExit(0 if values==[(1,8,1,0,14,1,0),(1,8,1,0,14,1,0)] else 1)'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code 355d42ac8bb9a5ed663f57895675df688507ce9f -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json AGENTS.md CODE_REVIEW.md ENGINEERING_GUIDE.md TROUBLESHOOTING_LOG.md",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -c 'from pathlib import Path; import hashlib,subprocess; paths=(\"docs/plans/2026-08-28-personal-assistant-v0-program.md\",\"docs/plans/2026-08-28-personal-assistant-v0-real-prompt-activation.md\",\"docs/plans/2026-08-28-personal-assistant-v0-https-dependency-decision.md\",\"docs/increments/personal-assistant-v0-https-dependency-decision.md\",\"docs/reviews/2026-09-03-personal-assistant-v0-https-dependency-decision-post-increment-review.md\",\"docs/plans/2026-09-03-personal-assistant-v0-selectable-connection-profile-architecture-decision.md\",\"docs/increments/pa-v0-selectable-connection-profile-decision.md\",\"docs/reviews/2026-09-03-pa-v0-selectable-connection-profile-decision-post-increment-review.md\",\"docs/plans/2026-09-03-personal-assistant-v0-fixed-local-private-lane-decision.md\",\"docs/increments/pa-v0-fixed-local-private-lane-decision.md\",\"docs/reviews/2026-09-03-pa-v0-fixed-local-private-lane-decision-post-increment-review.md\",\"docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md\",\"docs/increments/personal-assistant-v0-key-use-containment-classification.md\",\"docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md\",\"docs/plans/2026-09-03-personal-assistant-v0-pr112-publication-closeout.md\",\"docs/increments/personal-assistant-v0-pr112-publication-closeout.md\",\"docs/reviews/2026-09-03-personal-assistant-v0-pr112-publication-closeout-post-increment-review.md\"); pairs=[(p,hashlib.sha256(subprocess.check_output([\"git\",\"show\",\"355d42ac8bb9a5ed663f57895675df688507ce9f:\"+p])).hexdigest(),hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in paths]; print(\"\\n\".join(\"{} {} {}\".format(*row) for row in pairs)); raise SystemExit(0 if all(a==b for _,a,b in pairs) else 1)'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -c 'from pathlib import Path; import subprocess; baseline=subprocess.check_output([\"git\",\"show\",\"355d42ac8bb9a5ed663f57895675df688507ce9f:DECISIONS.md\"]); current=Path(\"DECISIONS.md\").read_bytes(); d119=baseline.split(b\"## D-119 -\",1)[1].split(b\"## D-120 -\",1)[0]; profiles=(b\"local_no_auth\",b\"google_gemini_oauth\",b\"google_gemini_api_key\",b\"direct_openai_api_key\",b\"direct_openai_workload_identity\",b\"azure_openai_entra\",b\"azure_openai_api_key\",b\"anthropic_api_key\",b\"mistral_api_key\",b\"aws_bedrock_identity\"); blockers=(b\"opaque_prebound_identity_contract\",b\"exact_signer_binding_contract\",b\"account_keychain_scope_contract\",b\"private_key_nonexport_contract\",b\"fixed_algorithm_contract\",b\"interaction_denial_contract\",b\"hard_deadline_cancellation_contract\",b\"late_result_rejection_contract\",b\"cleanup_quarantine_contract\",b\"platform_effect_contract\"); live=(\"HANDOFF.md\",\"NEXT_STEPS.md\",\"PLANS.md\",\"PROJECT_STATUS.md\",\"ROADMAP.md\",\"SECURITY.md\"); ok=current.startswith(baseline) and all(d119.count(x)==1 for x in profiles) and b\"Every entry is `candidate_blocked`\" in d119 and b\"D-113 through D-117 remain Proposed and non-controlling\" in d119 and all(x in baseline for x in blockers) and all(\"candidate_not_eligible_or_unproven\" in Path(p).read_text() and \"Blocked\" in Path(p).read_text() for p in live); print(\"d119_profiles=10 candidate_blocked=true blockers=10 proposed_noncontrolling=true blocked_live=6\"); raise SystemExit(0 if ok else 1)'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/post_increment_gate.py status",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-09-03
Increment: `personal-assistant-v0-fixed-local-engine-artifact-evidence-plan`
Branch: `codex/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan`
Baseline: `355d42ac8bb9a5ed663f57895675df688507ce9f`

## Executive summary

The exact sixteen-file documentation-only increment is complete with **PASS
WITH ADVISORIES**. The owner accepted the matching evidence-resolved tuple,
fifteen-row matrix, closed `candidate_not_eligible_or_unproven` disposition,
and exact D-121 wording. D-121 rejects the frozen
`fixed_local_v2_llamacpp_0_3_0_qwen2_5_1_5b_q4km_cpu_v1` candidate from the
current evidence; it does not select a replacement or authorize operational
work.

The tuple contains one `documented`, eight `contract_unproven`, and one
`not_run` result. The matrix contains zero `documented`, fourteen
`contract_unproven`, one target-Mac `not_run`, and zero `boundary_failed` rows.
The candidate remains unavailable, and next-increment readiness is `Blocked`.

## Scope and boundaries

The diff changes thirteen existing architecture, governance, product-memory,
security, and testing records and adds the approved plan, increment record, and
this review. These are exactly the sixteen authorized documentation paths. No
product/test source, dependency, manifest, lockfile, workflow, hook, skill,
configuration, capability, CSP, permission, entitlement, model, artifact, or
operational-system path changed.

D-094 synthetic-v1, historical V0-14, D-118 `no_eligible_client`, D-119's
exactly ten `candidate_blocked` entries and no handle, D-120, V0-3/V0-7, all
ten D-107 blockers, D-113 through D-117 Proposed/non-controlling status, and
every operational `Blocked` boundary remain unchanged.

## Verification results

| Check                                          | Status  | Evidence                                                                                                                                                                                                                   |
| ---------------------------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Baseline, branch, and gate                     | Passed  | The branch starts at `355d42ac8bb9a5ed663f57895675df688507ce9f`; the approved plan was the sole pre-branch delta; the predecessor was preserved; this gate began exactly once.                                             |
| Owner checkpoints                              | Passed  | The owner separately accepted the immutable envelope, full static-assessment authority, resolved tuple/matrix/disposition/D-121, and final reconciliation.                                                                 |
| Toolchains                                     | Passed  | Node `26.3.0`, npm `11.16.0`, rustc `1.90.0`, and cargo `1.90.0` ran on macOS `26.6` build `25G72`, arm64.                                                                                                                 |
| Documentation, repository, and security checks | Passed  | Formatting/link validation, repository policy, secret-pattern scanning, and diff hygiene passed.                                                                                                                           |
| Complete verification                          | Passed  | Hook tests: 74; repository-policy tests: 80; frontend tests: 370; Rust library tests: 302; Rust integration tests: 247 passed and one ignored. Lint, typecheck, frontend builds, and Tauri no-bundle release build passed. |
| Ignored test                                   | Not run | `real_hermes_version_probe_is_opt_in_and_version_only` remained intentionally ignored because it requires explicit opt-in and an operator-supplied pinned executable; the prescribed enclosing suite passed.               |
| Exact scope and protected paths                | Passed  | Gate-visible paths equal the exact sixteen-file allowlist; product/test source, dependencies, configuration, workflows, hooks, skills, and gate implementation are unchanged.                                              |
| D-121 identity and lineage                     | Passed  | Baseline `DECISIONS.md` is an exact byte prefix followed by one byte-exact copy of the owner-accepted D-121 block.                                                                                                         |
| Historical preservation                        | Passed  | SHA-256 comparisons preserve the seventeen required synthetic-v1, V0-14, D-107, D-118, D-119, D-120, and PR #113 closeout artifacts.                                                                                       |
| Candidate evidence                             | Passed  | Tuple and matrix totals are exact; the frozen coordinates and evidence boundary match; no positive admission criterion is claimed.                                                                                         |
| Public-source refresh                          | Failed  | One frozen Qwen README request returned HTTP 429. No workaround or wider source was used; affected claims remain `contract_unproven`.                                                                                      |
| Initial custom assertions                      | Failed  | One assertion split on a prose heading, and one counted D-121's new preservation reference as D-119 mutation. Narrow exact-heading and D-119-section replacements passed.                                                  |
| Initial closeout wording                       | Failed  | Independent review found pending header chronology and ambiguous generic Not-run wording. Bounded documentation-only corrections resolved the findings.                                                                    |
| Initial finalizer write                        | Failed  | The sandboxed attempt returned exit 4 because ignored gate state was not writable and made no state change. The exact command then succeeded with the required local filesystem permission.                                |
| Corrected independent re-review                | Passed  | Architecture, security, documentation-sync, code-health, technical-debt, quality, and readiness reviewers accepted the corrected sixteen-file result.                                                                      |
| Operational evidence                           | Not run | No artifact bytes, candidate build/load/inference, target-Mac inspection, product/model-artifact filesystem operation, credential, provider, signing, product system, or operational external system was accessed.         |
| Completion marker                              | Passed  | Report validation and the exact finalizer bound a complete valid marker to this report and workspace.                                                                                                                      |

## Architecture findings

Accepted. The result adds no runtime, source, dependency, filesystem, network,
provider, Tauri/UI, or product edge. Fixed local-v2 remains separate from
synthetic-v1 and D-119's future selectable catalog. `NativeAgentRuntime`
remains sole/default, and existing deterministic fixtures remain
non-transferable to local inference.

## Security findings

Accepted with one blocking-next advisory. The pinned core retains reachable
public dynamic-loading behavior, while fourteen mandatory rows remain unproved
and target-Mac evidence was not run. Treating the record as candidate admission
would bypass artifact identity, no-egress/native-TCB, lifecycle, owner-
authentication, and target-platform gates. No secret or operational boundary
was accessed or widened.

## Code-health findings

Accepted. The change is documentation-only, terminology is consistent, D-121
is append-only, and the exact scope and preservation assertions pass. The
initial ambiguous Not-run wording was corrected to distinguish passing
repository verification builds from unrun candidate-specific operations.

## Technical debt

No code or dependency debt was introduced. One security advisory remains:

- Category: Security
- Severity: Advisory
- Risk: reachable dynamic loading and unproved artifact, containment,
  lifecycle, authentication, and target-platform contracts make the frozen
  candidate ineligible or unproved.
- Effort: Large
- Milestone: before any replacement candidate, artifact acquisition,
  target-Mac execution, source/dependency change, or operational local-v2
  successor.
- Blocks completion: No
- Blocks next increment: Yes

## Roadmap findings

The documentation assessment is complete, but no candidate or successor is
Ready. D-121 closes only the exact frozen assessment. Any replacement would
require a new separately approved plan; this increment supplies no reusable
candidate-selection or operational authority.

## Completion decision

**PASS WITH ADVISORIES.** All required completion checks passed. The sole
advisory blocks any operational successor, not completion of this
documentation-only increment.

## Next-increment readiness

**Blocked.** No replacement candidate, artifact plan, target-Mac plan, source
increment, or operational local-v2 successor is selected or Ready. Owner review
of this completed increment is the only current action.

## Exact files changed

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PRODUCT_REQUIREMENTS.md`
8. `PROJECT_STATUS.md`
9. `ROADMAP.md`
10. `SECURITY.md`
11. `SECURITY_CHECKLIST.md`
12. `TESTING_GUIDE.md`
13. `docs/PROJECT_DIRECTION.md`
14. `docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md`
15. `docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md`
16. `docs/reviews/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan-post-increment-review.md`

## Exact commands executed

The machine-readable manifest above records the exact formatting,
documentation, repository, security, complete verification, diff, scope,
decision, tuple/matrix, preservation, session, report-validation, finalizer,
and status commands. All required commands ultimately passed. The earlier
malformed ad hoc D-121 assertion, two overly broad custom assertions, and first
sandboxed finalizer write are recorded truthfully as Failed; their corrected
or permission-scoped equivalents passed.
