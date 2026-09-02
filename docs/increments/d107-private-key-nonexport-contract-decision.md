# D-107 private-key non-export contract decision

Status: Complete (`PASS WITH ADVISORIES`)

D-112 selects `nonexport_contract_not_accepted`: existing wrappers expose
external representation and no implementation inventory proves export, debug,
serialization, log, error, test, or DTO paths unreachable. No private-key or
system operation ran. D-097 through D-111 and Blocked readiness remain intact.
