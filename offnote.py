import os
import sys
import argparse
import requests
from getpass import getpass
import socket

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

def create_page(api_key):
    hostname = socket.gethostname()
    url = "https://api.notion.com/v1/pages/"
    headers = {
        "Notion-Version": "2021-08-16",
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}"
    }
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
        print("Ya done screwed up.")

def main():
    print("I'm sorry if this Notion is Offensive to you.")
    try:
        NOTION_API_KEY = os.environ["NOTION_API_KEY"]
    except:
        NOTION_API_KEY = getpass("Enter API Key: ")

    page_id = create_page(NOTION_API_KEY)



if __name__ == "__main__":
    main()