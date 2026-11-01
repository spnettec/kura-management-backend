#!/usr/bin/env python3
"""Live additive takeover contracts; never mutates host network sources."""
import importlib.util
import json
from pathlib import Path
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("network_contract", ROOT / "scripts/phase2-contract-smoke.py")
n = importlib.util.module_from_spec(spec)
spec.loader.exec_module(n)


def main():
    c = n.m.Client()
    c.login("admin")
    target = {"interface": "test-absent"}
    n.check("takeover plan requires typed payload", n.submit(c, "network.takeover.plan")[0] == 400)
    n.check("inspect rejects takeover payload", n.submit(c, "network.inspect", networkTakeover=target)[0] == 400)
    n.check("unknown takeover field is rejected", n.submit(c, "network.takeover.plan", networkTakeover={**target, "path": "/etc/netplan/arbitrary"})[0] == 422)
    key = str(uuid.uuid4())
    body = {"operation": "network.takeover.plan", "idempotencyKey": key, "networkTakeover": target}
    status, receipt = c.request("/api/v1/gateways/dev-gateway/tasks", "POST", body)
    n.check("typed plan admitted", status == 202)
    same = c.request("/api/v1/gateways/dev-gateway/tasks", "POST", body)
    n.check("exact plan replay is idempotent", same[0] == 202 and same[1]["id"] == receipt["id"])
    changed = c.request("/api/v1/gateways/dev-gateway/tasks", "POST", {**body, "networkTakeover": {"interface": "different"}})
    n.check("plan key cannot change target", changed[0] == 409)
    result = n.m.wait_task(c, receipt["id"])
    n.check("missing source returns an explicit noneligible plan", result["state"] == "SUCCEEDED" and result["result"]["eligible"] is False and bool(result["result"]["diagnostic"]))
    payload = {**target, "expectedRevision": 0, "sourceDigest": "0" * 64, "planDigest": "0" * 64, "confirmSeconds": 120}
    status, receipt = n.submit(c, "network.takeover.prepare", networkTakeover=payload)
    n.check("takeover prepare admitted as typed task", status == 202)
    result = n.m.wait_task(c, receipt["id"])
    n.check("unreviewed missing source cannot be transferred", result["state"] == "FAILED" and result["error"] == "CONFLICT")
    operator = n.m.Client()
    operator.login("other")
    n.check("operator cannot prepare takeover", n.submit(operator, "network.takeover.prepare", networkTakeover=payload)[0] == 403)
    n.check("other-tenant operator cannot preview this gateway", n.submit(operator, "network.takeover.plan", networkTakeover=target)[0] == 404)
    n.check("logout", c.request("/api/v1/auth/logout", "POST")[0] == 204)
    output = {"passed": len(n.checks), "checks": n.checks, "writes": "none; development host has no delegated takeover source"}
    (ROOT / "target/takeover-contract.json").write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(output, indent=2))


if __name__ == "__main__":
    main()
