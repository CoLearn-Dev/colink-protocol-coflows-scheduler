1. start colink server
2. set up colink users and update jwts in python-examples
3. run scheduler for two users
```cargo run -- --addr http://127.0.0.1:2021 --jwt <jwt> --vt-public-addr 127.0.0.1```

3.a when restarting the scheduler, run `python-examples/reset_scheduler.py` before `cargo run`

4. python-examples
```python python-examples/dummy_dispatch.py --addr http://127.0.0.1:2021 --jwt <jwt>```

```python python-examples/push_to_other_user.py```

```python python-examples/push.py```

```python python-examples/reset_scheduler.py```
