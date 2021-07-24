import os
from typing import Dict, Iterable, List, Tuple

import py_school_match as psm
import rollbar
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from rollbar.contrib.fastapi import add_to as rollbar_add_to

from constants import ID_KEY, LABEL_KEY, MAX_PROXIES_PER_HOLDER, PROXY_KEYS

import logging

logger = logging.getLogger(__name__)
logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO").upper())

try:
    rollbar.init(
        os.environ["ROLLBAR_SERVER_TOKEN"],
        environment=os.environ.get("HEROKU_APP_NAME"),
        include_request_body=True,
    )
except KeyError as exc:
    logger.error(f"Failed to initialize Rollbar: missing {exc}")


class AttendanceSnapshot(BaseModel):
    memberList: list
    presentList: list


class ProxySpace:
    candidates: dict
    targets: dict

    def __init__(self) -> None:
        self.candidates = {}
        self.targets = {}

    @property
    def solution(self) -> Dict:
        return dict(self._solution)

    @property
    def _solution(self) -> Iterable[Tuple]:
        for target in self.targets.values():
            if target.assigned_school:
                yield (target.external_id, target.assigned_school.external_id)

    def solve(self) -> None:
        planner = psm.SocialPlanner(
            self.targets.values(), self.candidates.values(), psm.RuleSet()
        )
        planner.run_matching(psm.SIC())
        logger.info([str(t) for t in self.targets.values()])


class ProxyCandidate(psm.School):
    def __init__(self, external_id, label=None):
        super().__init__(MAX_PROXIES_PER_HOLDER)
        self.external_id = external_id
        self.label = label

    def __str__(self) -> str:
        if self.label:
            return f"""<ProxyCandidate label="{self.label}" external_id="{self.external_id}" id={self.id} assignments={self.assignments}>"""

        return f"""<ProxyCandidate external_id={self.external_id} id={self.id} assignments={self.assignments}>"""

    @property
    def assignments(self) -> int:
        return len(self.assignation.get_all_students())


class ProxyTarget(psm.Student):
    def __init__(self, external_id, label=None) -> None:
        super().__init__()
        self.external_id = external_id
        self.label = label

    def __str__(self) -> str:
        if self.label:
            return f"""<ProxyTarget label="{self.label}" external_id="{self.external_id}" id={self.id} proxy={self.assigned_school}>"""

        return f"""<ProxyTarget external_id={self.external_id} id={self.id} proxy={self.assigned_school}>"""

    def set_preferences(
        self, preferences: List[str], candidates: Dict[str, ProxyCandidate]
    ) -> None:
        self.preferences = list(
            self._filter_present_preferences(preferences, candidates)
        )

    def _filter_present_preferences(
        self, preferences: List[str], candidates: Dict[str, ProxyCandidate]
    ) -> Iterable[ProxyCandidate]:
        for pref in preferences:
            if pref == "":
                continue
            if pref not in candidates:
                logger.debug(
                    f"""{self} preference="{pref}" does not exist in list of present candidates"""
                )
                continue

            yield candidates[pref]


app = FastAPI()
rollbar_add_to(app)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health/ready/", status_code=204)
def health_ready() -> None:
    return


@app.post("/solve/")
def solve(snapshot: AttendanceSnapshot):
    space = ProxySpace()

    for member in snapshot.memberList:
        if member[ID_KEY] in snapshot.presentList:
            space.candidates[member[ID_KEY]] = ProxyCandidate(
                member[ID_KEY], label=member[LABEL_KEY]
            )

    for member in snapshot.memberList:
        if member[ID_KEY] not in snapshot.presentList:
            target = ProxyTarget(member[ID_KEY], label=member[LABEL_KEY])

            target.set_preferences([member[k] for k in PROXY_KEYS], space.candidates)
            logger.debug(
                f"{target} preferences={[p.external_id for p in target.preferences]}"
            )
            if len(target.preferences) == 0:
                logger.warning(f"{target} has no viable preferences")
                continue

            space.targets[member[ID_KEY]] = target

    space.solve()
    return space.solution