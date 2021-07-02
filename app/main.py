import py_school_match as psm
from fastapi import FastAPI
from pydantic import BaseModel

from .constants import MAX_PROXIES_PER_HOLDER, PROXY_KEYS


class AttendanceSnapshot(BaseModel):
    memberList: list
    presentList: list


class ProxyTarget(psm.Student):
    pass


class ProxyCandidate(psm.School):
    def __init__(self, id):
        super().__init__(MAX_PROXIES_PER_HOLDER, id)


app = FastAPI()


@app.post("/solve/")
def solve(snapshot: AttendanceSnapshot):
    targets = {}
    candidates = {}

    for member in snapshot.memberList:
        if member["lastName"] in snapshot.presentList:
            candidates[member["lastName"]] = ProxyCandidate(member["lastName"])

    for member in snapshot.memberList:
        if member["lastName"] not in snapshot.presentList:
            t = ProxyTarget(member["lastName"])
            t.preferences = [
                candidates[c]
                for c in filter(lambda v: v is not "", [member[k] for k in PROXY_KEYS])
            ]
            print(f"{t.id} prefers {[p.id for p in t.preferences]}")
            targets[member["lastName"]] = t

    planner = psm.SocialPlanner(targets.values(), candidates.values(), psm.RuleSet())
    planner.run_matching(psm.SIC())

    assignments = {t: targets[t].assigned_school.id for t in targets}
    for t, p in assignments.items():
        print(f"{t} will be represented by {p}")

    return assignments