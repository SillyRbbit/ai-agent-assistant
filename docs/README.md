# Documentation

Start with the [project overview](../README.md). Repository work follows
[root instructions](../AGENTS.md) and the
[master prompt](governance/MASTER_PROMPT.md); this index changes neither their
precedence nor approval boundaries.

## Current authority and state

- [Project direction](PROJECT_DIRECTION.md): owner-approved present scope and
  planned direction, not implementation evidence.
- [Handoff](../HANDOFF.md), [project status](../PROJECT_STATUS.md), and
  [next steps](../NEXT_STEPS.md): current work and evidence; read leading
  superseding entries before older sections.
- [Architecture](../ARCHITECTURE.md) and
  [product requirements](../PRODUCT_REQUIREMENTS.md): current boundaries and
  requirements; planned behavior is not implemented capability.
- [Decision log](../DECISIONS.md), [roadmap](../ROADMAP.md), and
  [plan register](../PLANS.md): decisions, sequencing, and active plan links.
- [Changelog](../CHANGELOG.md) and [troubleshooting log](../TROUBLESHOOTING_LOG.md):
  retained change and failure history.

## Browse by purpose

| Purpose                             | Entry points                                                                                                                                                             |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Development and assistant workflow  | [Workflow index](workflows/README.md), [assistant usage](workflows/ASSISTANT_USAGE.md), [engineering guide](../ENGINEERING_GUIDE.md), [contributing](../CONTRIBUTING.md) |
| Verification and review             | [Testing guide](../TESTING_GUIDE.md), [code review](../CODE_REVIEW.md), [review reports](reviews/README.md)                                                              |
| Security and release                | [Security policy](../SECURITY.md), [security checklist](../SECURITY_CHECKLIST.md), [release checklist](../RELEASE_CHECKLIST.md), [security references](security/)        |
| Architecture and decisions          | [Architecture assessments](architecture/), [ADRs](adr/), [design records](design/)                                                                                       |
| Plans and roadmap detail            | [Plans](plans/README.md), [increment records](increments/), [roadmap detail](roadmap/)                                                                                   |
| Product and demos                   | [Product source material](product/), [demo guides](demos/)                                                                                                               |
| Branding and repository setup       | [Brand guidelines](branding/BRAND_GUIDELINES.md), [GitHub setup](github/), [licensing record](github/LICENSING.md)                                                       |
| Templates and assistant prompts     | [Document templates](templates/), [prompt library](../prompts/README.md)                                                                                                 |
| Historical and exploratory evidence | [Retained backups](backups/), [spikes](spikes/)                                                                                                                          |

## Locations and history

Root authority documents remain in place because instructions, checks, or
repository conventions consume those locations. Tool-specific instructions,
skills, hooks, and GitHub files retain their scopes. The existing licensing
record remains at `docs/github/LICENSING.md`; no license is added or moved.

The assistant guide moved from root `ASSISTANT_USAGE.md` to
[`docs/workflows/ASSISTANT_USAGE.md`](workflows/ASSISTANT_USAGE.md). Older plans,
review manifests, commands, and backup snapshots retain that former path as
historical evidence; they are not current navigation or tooling instructions.
There is one current copy. Dated records are not obsolete solely because of
age, and a design, plan, or successful check does not grant new authority.
