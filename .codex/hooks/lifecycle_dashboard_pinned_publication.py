"""One pinned dashboard publication profile; integrity evidence, not authority.

No staging, ref updates, commits, acquisition or ordinary lifecycle transitions.
Historical acceptance and current publication validity are deliberately separate.
"""
from __future__ import annotations

import hashlib
import math
import os
from pathlib import Path
import secrets

import lifecycle_closure as c
import lifecycle_dashboard_publication_transition as prior
import lifecycle_dashboard_verification_repair as repair
import post_increment_gate as gate

PROFILE = "dashboard_pinned_publication_20261021_91ebfe85_v1"
INCREMENT = "dashboard-pinned-publication"
ORIGINAL = "faa9f936b78a600c5d5f0dc2d587032fb256d5bf"
UPSTREAM = "91ebfe8562c8bee675fb032c4e5f7ef0ed39d909"
UPSTREAM_TREE = "dc5e026b57f94c3e23c2a99288b13e6ba6e85de1"
UPSTREAM_PARENT = "f08f4ac06c044fe5c17d10c09d4f65d3cf0fb520"
BRANCH = "codex/usage-dashboard-publication"
ROOT_SHA256 = prior.original.ROOT_SHA256
SNAPSHOT_SHA256 = prior.SNAPSHOT_SHA256
STATE_SHA256 = prior.STATE_SHA256
RAW_MANIFEST_SHA256 = prior.PUBLICATION_SHA256
CANONICAL_MANIFEST_SHA256 = prior.PUBLICATION_CANONICAL_SHA256
INDEX_SHA256 = repair.INDEX_SHA256
HISTORY_COUNT = 166
STORE = Path(".codex/state/dashboard-pinned-publication-20261021-91ebfe85")
BASE = STORE / "accepted-reference.json"
REQUEST = STORE / "request.json"
PLAN = prior.PLAN
ASSESSMENT = "docs/reviews/2026-10-21-dashboard-pinned-reconciliation-design-assessment.md"
REVIEW_ID = "dashboard-pinned-publication-readiness"
REVIEW = "docs/reviews/2026-10-21-" + REVIEW_ID + "-post-increment-review.md"
NEW = {".codex/hooks/lifecycle_dashboard_pinned_publication.py",
       ".codex/hooks/tests/test_lifecycle_dashboard_pinned_publication.py", ASSESSMENT, REVIEW}
OVERLAPS = {'CHANGELOG.md', 'TROUBLESHOOTING_LOG.md', 'src-tauri/src/lib.rs', 'ARCHITECTURE.md', 'DECISIONS.md', 'PLANS.md', '.codex/hooks/post_increment_gate.py', 'scripts/tests/test_repository_health.py', 'PROJECT_STATUS.md', 'NEXT_STEPS.md', 'scripts/repository_health.py', 'ENGINEERING_GUIDE.md', 'HANDOFF.md'}
CANDIDATE = {'.codex/hooks/lifecycle_dashboard_publication.py': {'exists': True, 'mode': 420, 'sha256': '2aae3e46378aa96c8d14824657ade5f0c814dba0c1af58fed9bc241dacf8d19a'}, '.codex/hooks/lifecycle_dashboard_publication_transition.py': {'exists': True, 'mode': 420, 'sha256': 'd41106f9e8295092574449abb885bb735da53d9597b46c26bb31a5390b34e51c'}, '.codex/hooks/lifecycle_dashboard_verification.py': {'exists': True, 'mode': 420, 'sha256': '6468f803395efd44b36d5aaf7ddd9119eb06cf7fa9c3408f072787d15d493273'}, '.codex/hooks/lifecycle_dashboard_verification_repair.py': {'exists': True, 'mode': 420, 'sha256': '3e85d2642e1b361421f2aa7c20895f82f363508b8346a0882926f5c69c40e2b9'}, '.codex/hooks/post_increment_gate.py': {'exists': True, 'mode': 420, 'sha256': '525f15b867b5f821c42c1d36baf86440c32f8d7807a0c3a4bafb6b2b71204de4'}, '.codex/hooks/tests/test_lifecycle_dashboard_publication.py': {'exists': True, 'mode': 420, 'sha256': '72ada8de83f184334cd752283c095a046fa33ee1054379a64249c5823036e993'}, '.codex/hooks/tests/test_lifecycle_dashboard_publication_transition.py': {'exists': True, 'mode': 420, 'sha256': 'a8de553c04d1b2cfe26e4dd954a483c6ad9956470fe34dae64d79172234aaac5'}, '.codex/hooks/tests/test_lifecycle_dashboard_verification.py': {'exists': True, 'mode': 420, 'sha256': 'f9704d2638b3dd51809e93695fbd0cd6bccac22a3f67c987a350997c5fd9be57'}, '.codex/hooks/tests/test_lifecycle_dashboard_verification_repair.py': {'exists': True, 'mode': 420, 'sha256': '635ec49236ac296dee3bd658dfdf4e7b5a2fb17b0a51c4f393d705752106ed1f'}, '.prettierignore': {'exists': True, 'mode': 420, 'sha256': '7704d643f58a7511c15b328f3ef5a32d4b443f7ae960c36459fb71868e70ee1c'}, 'AGENTS.md': {'exists': True, 'mode': 420, 'sha256': '82ce73475587e8cdf374f4fa7c5710ee76eb095bf51872c421b567e576b065f6'}, 'ARCHITECTURE.md': {'exists': True, 'mode': 420, 'sha256': 'fd70049f29a9fd5fc68378f4e6b318539c363e58bfbd6011ca592c7b47561458'}, 'CHANGELOG.md': {'exists': True, 'mode': 420, 'sha256': '12ca15a311c93009755f179f63acf843d1c5513c5b93e0fed465815569cf5098'}, 'DECISIONS.md': {'exists': True, 'mode': 420, 'sha256': 'c9e3db6e34f27095a0383b52b029fa55d1ae21e79b5b2bcc530bb7b51606ee15'}, 'ENGINEERING_GUIDE.md': {'exists': True, 'mode': 420, 'sha256': '6698a81ec78761dbcebae7c1ab5ce8d3012817fe2900d3ed3c80b1cc76c52f97'}, 'HANDOFF.md': {'exists': True, 'mode': 420, 'sha256': 'a07be07e2dffb9ab752208b1dc5181e348f7384b4fd09c68aecef7b835e7815d'}, 'NEXT_STEPS.md': {'exists': True, 'mode': 420, 'sha256': '29dff706a3eae0accd7b3424a8c081d647ecb0f05fb4b4fa8ffc408e1653c700'}, 'PLANS.md': {'exists': True, 'mode': 420, 'sha256': '896a9ef7524795e14f8ffce54bbf69d33f173266699c4a5e7541a6918affe869'}, 'PROJECT_STATUS.md': {'exists': True, 'mode': 420, 'sha256': '1f25753248ca1a5b8907802a6bd23cab2b9946cb12acdc8a47a5853df2dfe84a'}, 'TROUBLESHOOTING_LOG.md': {'exists': True, 'mode': 420, 'sha256': '1ef44a8b8ae31d55f495316f55322e08d77b8af87633a2b0894bb407101b9484'}, 'docs/USAGE_EFFICIENCY_DEMO.md': {'exists': True, 'mode': 420, 'sha256': 'd8407d96c96b0863f31f47f945f7ff1c17254c14f37275c40bd1ae5302899556'}, 'docs/plans/2026-10-20-account-capacity-preview.md': {'exists': True, 'mode': 420, 'sha256': 'd2b02c9b45cf26783bf5b81b5da6e47c6221112dba5cc33300a3632e841e9467'}, 'docs/plans/2026-10-20-capacity-disclosure-label-repair.md': {'exists': True, 'mode': 420, 'sha256': '04ecc51b526078523cc6da96547b36f2d19246000808b029e6564509b41e16ff'}, 'docs/plans/2026-10-20-dashboard-dependent-recovery-maintenance.md': {'exists': True, 'mode': 420, 'sha256': '93241bfcd5e4044494a6ad1c62dd17a7c98ab9a565d4b5378657857e4f2c47fa'}, 'docs/plans/2026-10-20-publication-documentation-correction.md': {'exists': True, 'mode': 420, 'sha256': '427a067807b70a5a30f148f2cae475a53ee424a097497fd756e433f77a8793ad'}, 'docs/plans/2026-10-20-usage-dashboard-hierarchy.md': {'exists': True, 'mode': 420, 'sha256': '4ec5edc558e7bd1f87c997041dbd2e11beb38a8f75fd90caa813994199543018'}, 'docs/plans/2026-10-20-usage-dashboard-publication-preparation.md': {'exists': True, 'mode': 420, 'sha256': 'f64378cf6e5ff2f28c4bdf0ab2fe6e6ee0e7c942c84881497252fe0671b53fa3'}, 'docs/plans/2026-10-20-usage-dashboard-publication-recovery.md': {'exists': True, 'mode': 420, 'sha256': '1bc5bb85d1616e6f658871f38b75f47aac746c626a362dde5edb8561eaf6cf83'}, 'docs/plans/2026-10-20-usage-efficiency-demo.md': {'exists': True, 'mode': 420, 'sha256': 'aa888d6999a42ee3aea077fcae411c90a29f178f19c297d5c4c719b2f440ac95'}, 'docs/plans/2026-10-20-usage-efficiency-presentation.md': {'exists': True, 'mode': 420, 'sha256': '0f5240cdb86415bd646421dae23073dae0e89d4479d906249d1bb61d7d0c50c2'}, 'docs/plans/2026-10-20-usage-visual-polish.md': {'exists': True, 'mode': 420, 'sha256': 'e0d7fa9037f58535c3cdcd3dc2ab272a3062ed7feea9e1ff79d4d00ad22ab86c'}, 'docs/plans/2026-10-21-dashboard-publication-transition.md': {'exists': True, 'mode': 420, 'sha256': 'a05b3525b4cf0e4724c5a11dea0a74b5d84f69df45ace9174f4bc54312c1b3c0'}, 'docs/plans/2026-10-21-dashboard-verification-admission-maintenance.md': {'exists': True, 'mode': 420, 'sha256': 'ea9babd5a0aa3653e41d239e757cc891861739ccbb7b27b44b9d8c4ce716251f'}, 'docs/plans/2026-10-21-dashboard-verification-binding-continuation.md': {'exists': True, 'mode': 420, 'sha256': '7830d3c1c45adc6e8dcba45fabd5d2070b378a2adc0d1e1d133884be450c73fa'}, 'docs/plans/2026-10-21-dashboard-verification-repair-maintenance.md': {'exists': True, 'mode': 420, 'sha256': 'f045940012071c8de86e0c94b18a6055c63a5fc17b545ed75b9076d9350edaf9'}, 'docs/plans/2026-10-21-usage-dashboard-publication-verification-repair.md': {'exists': True, 'mode': 420, 'sha256': '7585633547f9ed2bf2eb058f562f63dd95fdad1b86ca0a7858b555474043573b'}, 'docs/plans/2026-10-21-usage-dashboard-publication-verification.md': {'exists': True, 'mode': 420, 'sha256': '2baa382b96c922eb6814724daec0d9aa621a8d8be766c460bb075c325abf3f01'}, 'docs/reviews/2026-10-20-account-capacity-preview-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '14af9c2c5bcf466261f5e0f0e4e05625684a51ad62315ce2f5d8d3e5c8b1cf91'}, 'docs/reviews/2026-10-20-account-capacity-preview-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '37cb51bc63c57cc343b5f2b0f2f96ffb729607077b69d4645f1de3ac0cc4f0f7'}, 'docs/reviews/2026-10-20-capacity-disclosure-label-repair-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '212fab19378caf01fe8a1b372c23ec13a7e1524f4cd71c178ee0d7abdc807db3'}, 'docs/reviews/2026-10-20-capacity-disclosure-label-repair-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'b4e6ede0095ad4c853ea5cc638a3c2f43f0a585d58167588c56d43ecf0d37a5f'}, 'docs/reviews/2026-10-20-dashboard-dependent-recovery-maintenance-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '75fd8bc7307df0a4cf637616b79abfa870689c5a47a362015dd8b3e1f25cfb37'}, 'docs/reviews/2026-10-20-publication-documentation-correction-assessment.md': {'exists': True, 'mode': 420, 'sha256': 'a6651256fc8b8c2b67e5739b9b56795b57235a500fc932613db29912774461c5'}, 'docs/reviews/2026-10-20-usage-dashboard-hierarchy-approved-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '3ce714c19672a853bd5f983c1dedf389d397acbdff3c82bb55069763d81fd340'}, 'docs/reviews/2026-10-20-usage-dashboard-hierarchy-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'e6fd539f8e53872a5628d17cf796231e53e1f2df7a5556e5ae1556d1d1a851f6'}, 'docs/reviews/2026-10-20-usage-dashboard-hierarchy-readiness-assessment.md': {'exists': True, 'mode': 420, 'sha256': 'c1da049e5a8913c1dd64819949fd1f4ffedc5ee7f82f8377c79a93fd736a255a'}, 'docs/reviews/2026-10-20-usage-dashboard-hierarchy-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '386e98306fd4720b52359c668f15e80e89cc0e5e9a0fb820f6712314a0503e65'}, 'docs/reviews/2026-10-20-usage-dashboard-publication-preparation-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'a17ba967b0800691004ab7ac4787a42652f3e0db2c2d71be52331f6ea5a72453'}, 'docs/reviews/2026-10-20-usage-dashboard-publication-preparation-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'd76ad99e617a13fbddfbb526948e320597b787526e1f76d11cf1183e0d9e4177'}, 'docs/reviews/2026-10-20-usage-dashboard-publication-recovery-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '2868aba02274028fb675e72b44e80106de4396e1bfc873843d231fb66d0cadcb'}, 'docs/reviews/2026-10-20-usage-dashboard-publication-recovery-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '3791db44d181645eb4d372007f5082a9d42aa0ecc768f5cbdfa70b3c326bc397'}, 'docs/reviews/2026-10-20-usage-efficiency-demo-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'f71183d89fca5d49cdcf12d4507d0d43e251c4446f9521c20c1d19df5a405962'}, 'docs/reviews/2026-10-20-usage-efficiency-demo-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '9998f0f74def6c36c3969e456ead91fd2bd46d02f7381943c62c83d3c616fc1a'}, 'docs/reviews/2026-10-20-usage-efficiency-presentation-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '61ed479a90096b7df55505da2366f9671d0441517b01cc631dc4116b617f054d'}, 'docs/reviews/2026-10-20-usage-efficiency-presentation-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '631c37afd7d0252227776acbb8d9bca677a034df950b07baf20860dc00af0b43'}, 'docs/reviews/2026-10-20-usage-visual-polish-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '038247ffb469e5a974755594b9468c8727db21b1119909aaab377956209ca2c6'}, 'docs/reviews/2026-10-20-usage-visual-polish-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'a7becb9c853769935e35c9339c07f279cafeb8fffe50ad8f9ec149d2868a6e90'}, 'docs/reviews/2026-10-21-dashboard-historical-formatting-recovery-assessment.md': {'exists': True, 'mode': 420, 'sha256': '27c56f58da85a812200518a7cfbc55b33183cd66179ece657a7552bbe8ed9769'}, 'docs/reviews/2026-10-21-dashboard-publication-boundary-repair-assessment.md': {'exists': True, 'mode': 420, 'sha256': 'dff0752716d6a4ec56b7f655f0fa300486aaf651ef4f6a99738a3240857ec258'}, 'docs/reviews/2026-10-21-dashboard-publication-transition-maintenance-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'a5f0e9eedcdb2c4b1e93acb70efc27a271b45ddb131ec1f6c0c49b4a6cb4d504'}, 'docs/reviews/2026-10-21-dashboard-transition-binding-repair-assessment.md': {'exists': True, 'mode': 420, 'sha256': '2c98c881447f3dd4616790351d88936a995f423816b5d061baa95593b4cc9a0a'}, 'docs/reviews/2026-10-21-dashboard-transition-custody-accounting-assessment.md': {'exists': True, 'mode': 420, 'sha256': '2889da58e51b64561963d083d930c9b6c5830d10d2d897ca4754cfdd53bbe78b'}, 'docs/reviews/2026-10-21-dashboard-transition-diagnostic-recovery-assessment.md': {'exists': True, 'mode': 420, 'sha256': '7b968685f97d13ac1529434b5df3aaa14fd9fa4680d25b144f6eefdafe86c503'}, 'docs/reviews/2026-10-21-dashboard-transition-fixture-repair-assessment.md': {'exists': True, 'mode': 420, 'sha256': '110af773a5b5c706338ff7f72b0b79bedc44b7d1565ba2987bd8cea198b5f617'}, 'docs/reviews/2026-10-21-dashboard-transition-tracked-fixture-repair-assessment.md': {'exists': True, 'mode': 420, 'sha256': 'ca1cf75b20dee6ee30881144f69a8903adadaf2bf02aa1e08dd0ecbf13117c82'}, 'docs/reviews/2026-10-21-dashboard-verification-admission-maintenance-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '71e735698235fa8751eed015edea262c005b2498eddf30589122cb86dc1a17c0'}, 'docs/reviews/2026-10-21-dashboard-verification-binding-continuation-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'a5001005940d56572a8eb114b995bd96420e2f1c2c312bf7517ef9f2b5842424'}, 'docs/reviews/2026-10-21-dashboard-verification-binding-preparation-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '4458d1e45b9e66ee3f9170ac3e5374702b5f032424dd05e381f2e17089539467'}, 'docs/reviews/2026-10-21-dashboard-verification-repair-maintenance-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'ee0f2a442b303c780f081c3f8c217193f2d091b0dcd8d122123b0adbcaf26401'}, 'docs/reviews/2026-10-21-usage-dashboard-publication-verification-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '05fc126d56e37393a58ea2bc424d9317277e25056af93a1816270e246b21daae'}, 'docs/reviews/2026-10-21-usage-dashboard-publication-verification-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': 'cf9c93cbdc1717b9f3b1ee652babbc3c29e93f794f41e18eabf5ee84ed524b29'}, 'docs/reviews/2026-10-21-usage-dashboard-publication-verification-repair-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '000831e8e85db794963ee139a126e7bd34c70d6f0f59d023b91cc5d4ac8ebff4'}, 'docs/reviews/2026-10-21-usage-dashboard-publication-verification-repair-readiness-post-increment-review.md': {'exists': True, 'mode': 420, 'sha256': '1751b24dd35cee2995bad533e724460d95ecd9b0fa908ed68661655a1e18c027'}, 'scripts/repository_health.py': {'exists': True, 'mode': 420, 'sha256': '04ac52b230499b41b168b6ec255749fbd3ff4ca2871be332d498cdc7acf85620'}, 'scripts/tests/test_repository_health.py': {'exists': True, 'mode': 420, 'sha256': '210607a57ac16c0b4e2c046bd8656230e101e0a24d422342eebf325a020f1dff'}, 'src-tauri/src/lib.rs': {'exists': True, 'mode': 420, 'sha256': '4bc613fac53c3b16e210e4628c445e366c3ad6ea74c9c6c4813cdee570750874'}, 'src-tauri/src/usage_efficiency.rs': {'exists': True, 'mode': 420, 'sha256': 'c41210356e264f8cccf3bd0ae4ba7502a6eda99c94325866c0b32337c7f7a01e'}, 'src-tauri/tests/usage_efficiency_contract.rs': {'exists': True, 'mode': 420, 'sha256': '150a9a7429e364b9a2b7f37700d122b548942614892ba95d73f6a4e0a91fe928'}, 'src/features/activity/ActivityPage.tsx': {'exists': True, 'mode': 420, 'sha256': 'c10ebc70c29dd5d3f04859a41b9e93895fb77ff4da3dbe0b88c0565180f13924'}, 'src/features/usage-efficiency/AccountCapacityPanel.test.tsx': {'exists': True, 'mode': 420, 'sha256': '325797183d1d1f7c2422d07541089116c28aa93308f3d3740b6267368a47d772'}, 'src/features/usage-efficiency/AccountCapacityPanel.tsx': {'exists': True, 'mode': 420, 'sha256': '5a3b094d7fa8992a108c9c92dabd6326bccba7b4825bb243f0c0cf0c06264db5'}, 'src/features/usage-efficiency/UsageCharts.tsx': {'exists': True, 'mode': 420, 'sha256': '4f72bc3e9f7d984b420bc9fc055af6a3b9390a1a99f76a89165c15bf3f10a73b'}, 'src/features/usage-efficiency/UsageEfficiencyPanel.test.tsx': {'exists': True, 'mode': 420, 'sha256': 'ec847472f244778a77ab694e9fa469edc7a0253444e396d9a42e923a69b546d0'}, 'src/features/usage-efficiency/UsageEfficiencyPanel.tsx': {'exists': True, 'mode': 420, 'sha256': '380c40997d34950d1fd675fa19c97a3219e8677eb93aca93fdcb1edffc01b0f5'}, 'src/features/usage-efficiency/UsageSummary.test.tsx': {'exists': True, 'mode': 420, 'sha256': 'a063ed6ef3765c96d9485ff8b8e2d89f04af4e31ff8dc7f2366305c91cbe2d41'}, 'src/features/usage-efficiency/UsageSummary.tsx': {'exists': True, 'mode': 420, 'sha256': 'f4fce6d8739149161ff7620816e67a655083ceebd15046ddcf6862afc93f440c'}, 'src/features/usage-efficiency/account-capacity.css': {'exists': True, 'mode': 420, 'sha256': '7b21c9ed44fe3f2c70feaa46ca7267b91be59ee04ce50c806af015754c6954de'}, 'src/features/usage-efficiency/usage-disclosure.css': {'exists': True, 'mode': 420, 'sha256': '1cebccbcbf801bb5a745d1cacb2d16ed94d6bbe164415b2b259bd1cc2936ab25'}, 'src/features/usage-efficiency/usage-efficiency.css': {'exists': True, 'mode': 420, 'sha256': '443da9fac221ee00632f594f9f5861d908f9cddfdbbfc0979fcd88eef607b774'}, 'src/features/usage-efficiency/usage-presentation.test.tsx': {'exists': True, 'mode': 420, 'sha256': 'a2ad009bdc68d7b9216347690ebe4bdb56b66f2b5f07d779337bf187c98cf5e0'}, 'src/features/usage-efficiency/usage-presentation.ts': {'exists': True, 'mode': 420, 'sha256': '834c3a4a81072ec2b0a19307e8c47dd0b906545e9f279c7e8e580d71967149a8'}, 'src/infrastructure/tauri/usage-efficiency-client.test.ts': {'exists': True, 'mode': 420, 'sha256': '4b10b4bf7c949472f09ff217102d717f9a3f4bd0bb34fd92d42b67f8aca0a6aa'}, 'src/infrastructure/tauri/usage-efficiency-client.ts': {'exists': True, 'mode': 420, 'sha256': '7ef8d26e8f26570356fddbfa0a84c0f74266a5d5041880fa121516ccc242cf09'}, 'src/test/usage-efficiency-fixture.ts': {'exists': True, 'mode': 420, 'sha256': '08b6cf2d73f9a9520875baefb7bc9e27df357a0818c9a27a3814e52920191ccd'}}
EDITABLE = OVERLAPS | {"AGENTS.md", PLAN} | NEW
# Exact current test maintenance; original CANDIDATE remains historical evidence.
FIXTURE_MAINTENANCE_PATH = '.codex/hooks/tests/test_lifecycle_dashboard_publication.py'
FIXTURE_MAINTENANCE_ORIGINAL_SHA256 = '72ada8de83f184334cd752283c095a046fa33ee1054379a64249c5823036e993'
FIXTURE_MAINTENANCE_SHA256 = '9891e86133bf5ae7bb1ef58f1f97fcbe7960a0f7abf3f482e68f58c88f7ebf2f'
# One owner-approved upstream test fixture correction; published identity is immutable.
STORAGE_TEST_PATH = 'scripts/tests/test_storage_inventory.py'
STORAGE_TEST_UPSTREAM_SHA256 = '76a997a124a80f795e50d11a7fba4eda9f882a2d78b3ddc5ab09fd92b9445979'
STORAGE_TEST_UPSTREAM_OID = 'f8aa796b53779e0b91898742c6067e0eec5b639c'
STORAGE_TEST_CURRENT_SHA256 = 'efdc65badd126765ec4251582ece01b0ef203f4788650582380a6ef7084ee094'
STORAGE_TEST_CURRENT_OID = '0f310ef2677ac05a6e3d595edaf054559cbd7035'
CHECKS = prior.CHECKS
PHASES = ("prepared", "staged", "parent-aligned", "committed")
require = c.require
read = prior.read
parse = prior.parse


def bounded(path):
    raw = read(path)
    require(len(raw) <= 1048576, "pinned binding bound")
    value = parse(raw)
    require(c.encoded(value) == raw, "pinned canonical artifact")
    return value


def path(root, name):
    return c.relative(root, str(name))


def present(root):
    return os.path.lexists(root / STORE)


@c.guarded
def accepted(root, state):
    """Own fixed reader: the old route's accepted.json is never consulted."""
    require(c.digest(str(root.resolve()).encode()) == ROOT_SHA256
            and gate._run_git(root, "branch", "--show-current").decode().strip() == BRANCH,
            "pinned checkout identity")
    raw = read(path(root, BASE))
    require(c.digest(raw) == SNAPSHOT_SHA256, "pinned snapshot drift")
    base = parse(raw)
    c.keys(base, {"schema", "root", "branch", "head", "raw_state", "index_hex", "manifest",
                  "protected_records", "publication_manifest", "publication_sha256", "retained_acceptance"})
    require(type(base["schema"]) is int and base["schema"] == 1
            and base["root"] == str(root.resolve()) and base["branch"] == BRANCH
            and base["head"] == ORIGINAL, "pinned snapshot identity")
    require(c.digest(base["raw_state"].encode()) == STATE_SHA256
            and read(gate.state_path(root)) == base["raw_state"].encode()
            and parse(base["raw_state"].encode()) == state
            and state["schema_version"] == 8 and state["status"] == "complete"
            and state["increment_id"] == repair.SUCCESSOR, "pinned historical state drift")
    gate.validate_state(state)
    require(c.fingerprint(c.normalize_manifest(base["manifest"])) == state["workspace_fingerprint"]
            and c.digest(bytes.fromhex(base["index_hex"])) == INDEX_SHA256,
            "pinned historical index/inventory")
    require(c.digest(read(path(root, state["report_path"]))) == state["report_sha256"],
            "pinned historical report drift")
    require(base["publication_sha256"] == RAW_MANIFEST_SHA256
            and c.digest(c.encoded(base["publication_manifest"])) == CANONICAL_MANIFEST_SHA256,
            "pinned raw/canonical provenance")
    require(len(base["protected_records"]) == HISTORY_COUNT, "pinned historical record count")
    for name, item in base["protected_records"].items():
        require(name.startswith(".codex/state/"), "pinned historical namespace")
        raw_record = read(path(root, name))
        require(c.digest(raw_record) == item["sha256"]
                and prior.mode(path(root, name)) == item["mode"], "pinned historical record drift")
    return base


def object_identity(root):
    actual = gate._run_git(root, "show", "-s", "--format=%T%n%P", UPSTREAM).decode().splitlines()
    require(actual == [UPSTREAM_TREE, UPSTREAM_PARENT], "pinned upstream commit/tree/parent")


def candidate_binding(name):
    value = CANDIDATE[name]
    if name == FIXTURE_MAINTENANCE_PATH:
        require(type(value) is dict and set(value) == {'exists', 'mode', 'sha256'}
                and type(value['exists']) is bool and type(value['mode']) is int
                and value == {'exists': True, 'mode': 0o644,
                              'sha256': FIXTURE_MAINTENANCE_ORIGINAL_SHA256},
                'pinned original fixture maintenance binding')
        return {'exists': True, 'mode': 0o644, 'sha256': FIXTURE_MAINTENANCE_SHA256}
    return value


def storage_test_maintenance(upstream, manifest, leaves):
    require(upstream.get(STORAGE_TEST_PATH) == ('100644', STORAGE_TEST_UPSTREAM_OID),
            'pinned upstream test maintenance origin')
    require(manifest.get(STORAGE_TEST_PATH) == {'exists': True, 'mode': 0o644,
                                              'sha256': STORAGE_TEST_CURRENT_SHA256}
            and leaves.get(STORAGE_TEST_PATH) == ('100644', STORAGE_TEST_CURRENT_OID),
            'pinned upstream test maintenance current')
    return {'upstream': {'mode': '100644', 'oid': STORAGE_TEST_UPSTREAM_OID,
                         'sha256': STORAGE_TEST_UPSTREAM_SHA256},
            'current': {'mode': '100644', 'oid': STORAGE_TEST_CURRENT_OID,
                        'sha256': STORAGE_TEST_CURRENT_SHA256}, 'group': 'governance'}


def inventories(root, request, base):
    manifest = c.normalize_manifest(request["manifest"])
    require(manifest == request["manifest"] and manifest == c.manifest(root), "pinned current inventory")
    upstream = {n: (m, oid) for n, m, oid in prior.tree(root, UPSTREAM)}
    original = {n: (m, oid) for n, m, oid in prior.tree(root, ORIGINAL)}
    expected = prior.expected_tree(root, manifest)
    leaves = {n: (m, oid) for n, m, oid in expected}
    require(set(leaves) == set(upstream) | set(CANDIDATE) | NEW, "pinned complete combined scope")
    maintenance = storage_test_maintenance(upstream, manifest, leaves)
    for name, value in upstream.items():
        if name not in CANDIDATE and name not in NEW and name != STORAGE_TEST_PATH:
            require(leaves[name] == value, "pinned upstream inheritance drift")
    for name, value in CANDIDATE.items():
        if name not in EDITABLE:
            require(manifest.get(name) == candidate_binding(name), "pinned protected candidate drift")
        else:
            require(manifest[name]["mode"] == value["mode"], "pinned candidate mode drift")
    for name in NEW:
        require(manifest[name]["mode"] == 0o644, "pinned new mode drift")
    prior.maintenance_history(root, base, manifest)
    publication = request["publication_manifest"]
    c.keys(publication, {"original_sha256", "groups", "upstream_inheritance", "upstream_test_maintenance"})
    require(publication["original_sha256"] == RAW_MANIFEST_SHA256
            and set(publication["groups"]) == prior.GROUPS, "pinned group identities")
    old_groups = base["publication_manifest"]["groups"]
    seen = set()
    for group, values in publication["groups"].items():
        require(isinstance(values, dict), "pinned group shape")
        names = {item["path"] for item in old_groups[group]}
        if group == "governance":
            names |= (set(CANDIDATE) - {row["path"] for rows in old_groups.values() for row in rows}) | NEW | {STORAGE_TEST_PATH}
        require(set(values) == names and not seen & names, "pinned omitted/duplicate/misgrouped path")
        require(all(values[n] == manifest[n]["sha256"] for n in names), "pinned group content")
        seen |= names
    require(seen == set(CANDIDATE) | NEW | {STORAGE_TEST_PATH}, "pinned attribution coverage")
    inherited = {n: {"mode": m, "oid": oid} for n, (m, oid) in upstream.items()
                 if n not in CANDIDATE and n != STORAGE_TEST_PATH and original.get(n) != (m, oid)}
    require(publication["upstream_inheritance"] == inherited, "pinned upstream attribution")
    require(publication['upstream_test_maintenance'] == {STORAGE_TEST_PATH: maintenance},
            'pinned upstream test maintenance provenance')
    return manifest, expected


def proof(root, ref, decode=True):
    c.keys(ref, {"path", "sha256"})
    require(isinstance(ref["path"], str)
            and ref["path"].startswith((STORE / "evidence").as_posix() + "/"), "pinned proof namespace")
    raw = read(path(root, ref["path"]))
    require(c.digest(raw) == c.sha(ref["sha256"]), "pinned proof drift")
    return bounded(path(root, ref["path"])) if decode else raw


def proofs(root, request, inputs):
    c.keys(request["checks"], set(CHECKS))
    for label, ref in request["checks"].items():
        require(ref.get("path") == (STORE / "evidence" / (label + ".json")).as_posix(),
                "pinned fixed proof role")
        item = proof(root, ref)
        c.keys(item, {"label", "argv", "cwd", "input_bindings", "exit_status", "timed_out",
                      "complete_capture", "cleanup", "elapsed_seconds", "log"})
        require(item["label"] == label and item["argv"] == CHECKS[label]
                and item["cwd"] == str(root.resolve()) and item["input_bindings"] == inputs
                and type(item["exit_status"]) is int and item["exit_status"] == 0
                and item["timed_out"] is False and item["complete_capture"] is True
                and type(item["cleanup"]) is dict and set(item["cleanup"]) == {"root_reaped", "group_absent"}
                and all(value is True for value in item["cleanup"].values())
                and type(item["elapsed_seconds"]) in {int, float}
                and math.isfinite(item["elapsed_seconds"])
                and 0 <= item["elapsed_seconds"] <= (1800 if label == "full" else 600),
                "pinned incomplete/stale/failed proof")
        require(item["log"].get("path") == (STORE / "evidence" / (label + ".log")).as_posix(),
                "pinned fixed capture role")
        require(len(proof(root, item["log"], False)) <= 2097152, "pinned capture bound")


@c.guarded
def contract(root, request, base):
    c.keys(request, {"schema", "profile", "increment_id", "original_parent", "upstream_parent", "upstream_tree",
                     "accepted_sha256", "original_raw_manifest_sha256", "original_canonical_manifest_sha256",
                     "baseline_bindings", "manifest", "publication_manifest", "expected_tree", "owner_authorization",
                     "plan_sha256", "review_sha256", "checks"})
    require(type(request["schema"]) is int and request["schema"] == 1 and request["profile"] == PROFILE
            and request["increment_id"] == INCREMENT and request["original_parent"] == ORIGINAL
            and request["upstream_parent"] == UPSTREAM and request["upstream_tree"] == UPSTREAM_TREE
            and request["accepted_sha256"] == SNAPSHOT_SHA256
            and request["original_raw_manifest_sha256"] == RAW_MANIFEST_SHA256
            and request["original_canonical_manifest_sha256"] == CANONICAL_MANIFEST_SHA256,
            "pinned request identity")
    c.authorization(root, request["owner_authorization"])
    require(c.digest(read(path(root, PLAN))) == c.sha(request["plan_sha256"]), "pinned frozen plan drift")
    object_identity(root)
    manifest, expected = inventories(root, request, base)
    inputs = {n: value for n, value in manifest.items() if n != REVIEW}
    require(request["baseline_bindings"] == inputs, "pinned proof baseline")
    require(request["expected_tree"] == [list(row) for row in expected], "pinned reviewed tree")
    raw = read(path(root, REVIEW))
    require(c.digest(raw) == c.sha(request["review_sha256"]), "pinned readiness drift")
    report = gate._extract_manifest(raw.decode())
    require(report["increment_id"] == REVIEW_ID
            and report["quality_gate"] in {"PASS", "PASS WITH ADVISORIES"}, "pinned nonpassing readiness")
    for name in ("Preservation and scope review", "Architecture, security, code-health and readiness review"):
        rows = [row for row in report["manual_verification"] if row["check"] == name]
        require(len(rows) == 1 and rows[0]["required"] is True and rows[0]["status"] == "Passed",
                "pinned substantive review missing")
    require(not any(row["blocks_completion"] for row in report["findings"]), "pinned blocking finding")
    proofs(root, request, inputs)
    return expected


def phase_path(phase, kind):
    require(phase in PHASES and kind in {"intent", "receipt"}, "pinned phase identity")
    return STORE / (phase + "." + kind + ".json")


def phase_now(root, base, expected):
    head = gate.current_head_commit(root)
    raw = read(c.git_index_path(root)); index = prior.index_semantics(raw)
    if head == ORIGINAL:
        if raw == bytes.fromhex(base["index_hex"]):
            phase = "prepared"
        else:
            require(index == expected, "pinned partial/hidden/unrelated index")
            phase = "staged"
    elif head == UPSTREAM:
        require(index == expected, "pinned parent-aligned index")
        phase = "parent-aligned"
    else:
        parents = gate._run_git(root, "show", "-s", "--format=%P", head).decode().strip().split()
        require(parents == [UPSTREAM] and index == expected and prior.tree(root, head) == expected,
                "pinned committed ancestry/tree/index")
        phase = "committed"
    require(read(c.git_index_path(root)) == raw and gate.current_head_commit(root) == head,
            "pinned index/HEAD race")
    return {"phase": phase, "head": head, "index_sha256": c.digest(raw)}


def chain(root, request_raw, upto):
    previous = c.digest(request_raw)
    for phase in PHASES[:PHASES.index(upto) + 1]:
        intent = bounded(path(root, phase_path(phase, "intent")))
        receipt = bounded(path(root, phase_path(phase, "receipt")))
        c.keys(intent, {"profile", "phase", "request_sha256", "previous_sha256", "token"})
        c.keys(receipt, {"intent_sha256", "observation"})
        require(intent["profile"] == PROFILE and intent["phase"] == phase
                and intent["request_sha256"] == c.digest(request_raw)
                and intent["previous_sha256"] == previous
                and type(intent["token"]) is str and len(intent["token"]) == 64,
                "pinned phase lineage")
        require(receipt["intent_sha256"] == c.digest(c.encoded(intent))
                and receipt["observation"]["phase"] == phase, "pinned phase receipt")
        previous = c.digest(c.encoded(receipt))
    for phase in PHASES[PHASES.index(upto) + 1:]:
        require(not os.path.lexists(root / phase_path(phase, "intent"))
                and not os.path.lexists(root / phase_path(phase, "receipt")), "pinned interrupted/unexpected effect")
    return previous


@c.guarded
def validate(root, state):
    base = accepted(root, state)
    request = bounded(path(root, REQUEST)); expected = contract(root, request, base)
    observation = phase_now(root, base, expected)
    chain(root, c.encoded(request), observation["phase"])
    last = bounded(path(root, phase_path(observation["phase"], "receipt")))
    require(last["observation"] == observation, "pinned phase observation drift")
    return {"historical_acceptance_valid": True, "current_publication_valid": True,
            **observation, "profile": PROFILE, "upstream_reconciliation_supported": True}


@c.guarded
def prepare_effect(root, phase):
    """Journal only. A later owner-authorized caller performs the Git effect."""
    require(phase in PHASES, "pinned phase identity")
    request = bounded(path(root, REQUEST)); raw = c.encoded(request)
    state = gate.read_state(root); base = accepted(root, state)
    expected = contract(root, request, base)
    current = phase_now(root, base, expected)
    i = PHASES.index(phase)
    if i == 0:
        require(current["phase"] == "prepared", "pinned seal requires original index/HEAD")
        for name in PHASES:
            require(not os.path.lexists(root / phase_path(name, "intent"))
                    and not os.path.lexists(root / phase_path(name, "receipt")), "pinned replay")
        previous = c.digest(raw)
    else:
        require(current["phase"] == PHASES[i-1], "pinned unsupported phase transition")
        previous = chain(root, raw, current["phase"])
    token = secrets.token_hex(32)
    prior.original.exclusive(root, phase_path(phase, "intent"), c.encoded({
        "profile": PROFILE, "phase": phase, "request_sha256": c.digest(raw),
        "previous_sha256": previous, "token": token}))
    return token


@c.guarded
def finish_effect(root, phase, token):
    intent = bounded(path(root, phase_path(phase, "intent")))
    require(intent["token"] == token and not os.path.lexists(root / phase_path(phase, "receipt")),
            "pinned consumed/foreign effect")
    request = bounded(path(root, REQUEST)); state = gate.read_state(root)
    base = accepted(root, state); expected = contract(root, request, base)
    observation = phase_now(root, base, expected)
    require(observation["phase"] == phase and intent["request_sha256"] == c.digest(c.encoded(request)),
            "pinned post-effect drift")
    prior.original.exclusive(root, phase_path(phase, "receipt"), c.encoded({
        "intent_sha256": c.digest(c.encoded(intent)), "observation": observation}))
    validate(root, state)  # Leave issued intent/receipt intact on post-effect failure.


@c.guarded
def seal(root, request_path):
    require(request_path == REQUEST.as_posix(), "pinned fixed request path")
    token = prepare_effect(root, "prepared")
    finish_effect(root, "prepared", token)
