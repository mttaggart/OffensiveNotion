#!/usr/bin/env python3
import os
import sys
import argparse
import requests
from getpass import getpass
import socket
from time import sleep
import subprocess

URL_BASE = "https://api.notion.com/v1"

api_key = "secret_3IbYfPpuiFzyF8wFnKNq2e9Tgtj5I257QoGjg3mGJBz"
parent_page_id = "bf622c586fad48e58ba295e249d61d51"

#api_key = input("What is their API key?")
#parent_page_id = input("What is their parent page ID?")

def get_blocks(headers, page_id):
    """
    Retrieve blocks from the parent page
    """
    url = f"{URL_BASE}/blocks/{page_id}/children"
    r = requests.get(url, headers=headers)
    if r.status_code == 200:
        print(r.json()["results"])
        return r.json()["results"]
    return None

def inject_kill_command(headers, command_block_id):
    url = f"{URL_BASE}/blocks/{command_block_id}/children"
    body = {
        "children": [
            {
                "object": "block",
                "type": "todo",
                "quote": {
                    "text": [
                        {
                            "type": "text",
                            "text": {"content": "shutdown "},
                            "annotations": {"code": "false"}
                        }
                    ]
                }
            }
        ]
    }
    r = requests.patch(url, headers=headers, json=body)
    if r.status_code != 200:
        print(r.content)

def intercept(headers):
    url = f"{URL_BASE}/pages/"

    body = {
        "parent": {
            "type": "page_id",
            "page_id": parent_page_id
        },
        "properties": {
            "title": [{
                "text": {
                    "content": "INTERCEPTED"
                }
            }]
        }
    }
    r = requests.post(url, json=body, headers=headers)
    if r.status_code == 200:
        id = r.json()["id"]
        return id
    else:
        print(r.content)

def main():
    print("Interceptor!")

    headers = {
        "Notion-Version": "2021-08-16",
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}"
    }

    get_blocks(headers, parent_page_id)
    intercept(headers)
    inject_kill_command(headers, parent_page_id)

if __name__ == "__main__":
    main()