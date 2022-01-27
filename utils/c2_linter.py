import socket
import requests
from utils.colors import *


def create_page(NOTION_API_KEY, PARENT_PAGE_ID) -> bool:
    """
    One stop function ripped from offnote.py to do a C2 lint check, uses the passed in vars to perform a request and create a page.
    """

    URL_BASE = "https://api.notion.com/v1"
    url = f"{URL_BASE}/pages/"

    headers = {
        "Notion-Version": "2021-08-16",
        "Content-Type": "application/json",
        "Authorization": f"Bearer {NOTION_API_KEY}"
    }

    body = {
        "parent": {
            "type": "page_id",
            "page_id": PARENT_PAGE_ID
        },
        "properties": {
            "title": [{
                "text": {
                    "content": "C2_LINT_TEST"
                }
            }]
        }
    }
    try:
        print(info + "POSTing to the Notion API...")
        r = requests.post(url, json=body, headers=headers)
        print(info + "Status code: {}".format(str(r.status_code)))
        if r.status_code == 200:
            return True
    except Exception as e:
        print(printError + "C2 Lint Error: {}".format(str(e)))
        return False
