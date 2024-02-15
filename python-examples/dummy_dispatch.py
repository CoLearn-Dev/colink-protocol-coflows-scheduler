import logging
from typing import List
import colink as CL
from colink import CoLink, ProtocolOperator
import json
import time

pop = ProtocolOperator(__name__)


@pop.handle("coflows_dispatch:local")
def run_initiator(cl: CoLink, param: bytes, participants: List[CL.Participant]):
    json_str = param.decode("utf-8")
    params = json.loads(json_str)
    print(params)
    print(cl.read_entry(params["message_ids"][0]))
    time.sleep(10)


if __name__ == "__main__":
    pop.run()
