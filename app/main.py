import os
from typing import Dict, List

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


class ProxyCandidate(psm.School):
    def __init__(self, name):
        super().__init__(MAX_PROXIES_PER_HOLDER)
        self.name = name


class ProxyTarget(psm.Student):
    def __init__(self, name):
        super().__init__()
        self.name = name

    def set_preferences(
        self, preferences: List[str], candidates: Dict[str, ProxyCandidate]
    ):
        self.preferences = list(
            self._filter_present_preferences(preferences, candidates)
        )

    def _filter_present_preferences(
        self, preferences: List[str], candidates: Dict[str, ProxyCandidate]
    ):
        for pref in preferences:
            if pref == "":
                continue
            if pref not in candidates:
                logger.debug(
                    f"target={self.id} preference={pref} does not exist in list of present candidates"
                )
                continue

            yield candidates[pref]


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

            t.set_preferences([member[k] for k in PROXY_KEYS], candidates)
            logger.info(f"member={t.id} preferences={[p.id for p in t.preferences]}")
            if len(t.preferences) == 0:
                logger.warning(f"member={t.id} has no viable preferences")
                continue

            targets[member["lastName"]] = t

    planner = psm.SocialPlanner(targets.values(), candidates.values(), psm.RuleSet())
    planner.run_matching(psm.SIC())

    assignments = {t: targets[t].assigned_school.id for t in targets}
    for t, p in assignments.items():
        logger.info(f"member={t} proxy={p}")

    return assignments