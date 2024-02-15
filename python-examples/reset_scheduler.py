import sys
import logging
import colink as CL
from colink import CoLink, decode_jwt_without_validation
import json
import secrets

if __name__ == "__main__":
    addr = "http://127.0.0.1:2021"
    jwt_a = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJwcml2aWxlZ2UiOiJ1c2VyIiwidXNlcl9pZCI6IjAzNTY2M2EyNDgzYWYwZDlkNmRjNWRhYWNjNmRiOWVjYjA5ZTUzOGY1NjU2OWRlOGYzYzdiNzVmYzJjYTJjZmZkYiIsImV4cCI6MTcxMDY1MTM0N30.ho0uUVjSwWfFz1XGqTf0kH7NvhyPWzgvE3nvUM8Win0"
    jwt_b = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJwcml2aWxlZ2UiOiJ1c2VyIiwidXNlcl9pZCI6IjAyMjkyNDlkZTE1ODA4OGMzMDM2ZDE1MWUxZjRmNDU4NTA3YTdkMWE0NTg3MWUwNWU2YzkxNzI2ZmIxZGEzMGE3OCIsImV4cCI6MTcxMDY1MTM0N30.58tASkzsAQZcoc534HPK-Ua3Wi7k1uZ5Ba4v3TvgNnY"
    cl = CoLink(addr, jwt_a)
    cl.delete_entry("_internal:protocols:coflows_scheduler:_is_initialized")
    cl = CoLink(addr, jwt_b)
    cl.delete_entry("_internal:protocols:coflows_scheduler:_is_initialized")
