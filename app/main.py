import os

import py_school_match as psm
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

from constants import MAX_PROXIES_PER_HOLDER, PROXY_KEYS

import logging

logger = logging.getLogger(__name__)
logging.basicConfig(level=os.environ.get("LOG_LEVEL", logging.DEBUG).upper())


class AttendanceSnapshot(BaseModel):
    memberList: list
    presentList: list


class ProxyTarget(psm.Student):
    pass


class ProxyCandidate(psm.School):
    def __init__(self, id):
        super().__init__(MAX_PROXIES_PER_HOLDER, id)


app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


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
            logger.info(f"member={t.id} preferences={[p.id for p in t.preferences]}")
            targets[member["lastName"]] = t

    planner = psm.SocialPlanner(targets.values(), candidates.values(), psm.RuleSet())
    planner.run_matching(psm.SIC())

    assignments = {t: targets[t].assigned_school.id for t in targets}
    for t, p in assignments.items():
        logger.info(f"member={t} proxy={p}")

    return assignments