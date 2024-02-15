import sys
import logging
import colink as CL
from colink import CoLink, decode_jwt_without_validation
import json
import secrets

if __name__ == "__main__":
    addr = "http://127.0.0.1:2021"
    jwt_a = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJwcml2aWxlZ2UiOiJ1c2VyIiwidXNlcl9pZCI6IjAzNTY2M2EyNDgzYWYwZDlkNmRjNWRhYWNjNmRiOWVjYjA5ZTUzOGY1NjU2OWRlOGYzYzdiNzVmYzJjYTJjZmZkYiIsImV4cCI6MTcxMDY1MTM0N30.ho0uUVjSwWfFz1XGqTf0kH7NvhyPWzgvE3nvUM8Win0"
    flow_id = "flowA"
    msg = "hello"
    message_key = "random_id" + secrets.token_hex(16)
    user_id_a = decode_jwt_without_validation(jwt_a).user_id
    participants = [
        CL.Participant(
            user_id=user_id_a,
            role="receiver",
        ),
    ]
    cl = CoLink(addr, jwt_a)
    core_pub_key = cl.request_info().core_public_key
    cl.create_entry(message_key, msg)
    task_id = cl.run_task("coflows_push", json.dumps({"flow_id": flow_id, "message_id": message_key}), participants, True)
