"""Check archived evidence and optionally refresh it from public Horizon."""
import argparse, json, pathlib, subprocess
root = pathlib.Path(__file__).resolve().parents[1]
manifest = json.loads((root / "docs/evidence/testnet-proof-2026-10-27.json").read_text())
p = argparse.ArgumentParser(); p.add_argument("--live", action="store_true"); args = p.parse_args()
for kind, item in manifest["transactions"].items():
    record = (json.loads(subprocess.check_output(["curl", "-fsS", "--max-time", "30", item["horizon"]]))
              if args.live else json.loads((root / f"docs/evidence/testnet-{kind}-transaction.json").read_text()))
    assert record["successful"] is True
    assert record["hash"] == item["hash"] and record["ledger"] == item["ledger"]
    assert record["source_account"] == manifest["test_account"]
    print(kind, "SUCCESS", item["hash"], "ledger", item["ledger"])
print("Verification covers recorded transaction success; not a contract audit.")
