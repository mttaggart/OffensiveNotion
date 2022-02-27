<h1 align="center">
OffensiveNotion
</h1>
<h3 align="center"> Notion (yes, the notetaking app) as a C2.</h3>
<div align="center">

---
A collaboration by:

mttaggart | HuskyHacks

---

[Documentation][wiki]&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;[Pull Requests][pr]&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;[Issues][issues]

![GitHub last commit][lastcommit] [![Pull Requests][img-pr-badge]][pr] [![License][img-license-badge]][license]

</div>

---

![on](https://user-images.githubusercontent.com/57866415/155594981-1ae9212e-a0f9-4ff3-8a81-8946546dc0a3.gif)


### Wait, What?
Yes.

### But Why?
What started as a meme grew into a full project. Just roll with it.

## Features
* 📡 A full-featured C2 platform built on the Notion notetaking app.
* 🚧 Easy setup: set up your Notion developer API account, drop the Agent to the target, run and enjoy!
* 🖥️ Cross-platform agent built in Rust that compiles for Linux and Windows with the same code base.
* ☢️ A range of capabilities including port-scanning, privilege escalation, asynchronous command execution, file download, and shellcode injection, all controlled from the comfort of a Notion page!
* 📜 Document as you go! The agent identifies special syntax to run commands, so feel free to use the rest of the Notion page to document your operation.
* 🕵️‍♀️ Stealth! C2 comms ride over the Notion API natively. Your C2 traffic looks like someone is using Notion for its intended purpose.

## Quickstart
See the [Quickstart guide](https://github.com/HuskyHacks/OffensiveNotion/wiki/2.-Quickstart) on how to get going right away!

## Documentation
Please see the [Wiki][wiki] for setup, usage, commands, and more!

## v1.0.0  - "Iron Age"
### MUST

<details>
<summary> Done </summary>

  ### Documentation
- [x] Quickstart
- [x] Install
- [x] Agent interaction
  - [x] Commands
  - [x] Linux commands
  - [x] Windows commands

#### Misc
- [x] YARA Rules
#### Setup
- [x] Python Setup Script for config options
- [x] Dynamic Docker container spin up/tear down for agent generation
- [x] Parse args for Docker build options

#### Agent
- Commands:
  - [x] `shell`
  - [x] `cd`
  - [x] `download`
  - [x] `ps`
  - [x] `pwd`
  - [x] `save`
  - [x] `shutdown`
  - [x] `sleep [#]` to adjust callback

</details>

### SHOULD

<details>
<summary> Done </summary>

#### Agent
- [x] Jitter interval for callback time
- Commands:
  - [x] `getprivs`
  - [x] `sleep [#][%]` to adjust callback and jitter
  - [x] `portscan`
- [x] Linux `elevate sudo`
- [x] Windows `elevate fodhelper`
- [x] Linux `persist bashrc`
- [x] Linux `persist cron`
- [x] Linux `persist service`
- [x] Windows `inject`
- [x] Windows `persist startup`
- [x] Windows `persist registry`

- Persist:
  - [x] Windows `persist schtasks`
  - [x] (Bonus) `wmic`
  
</details>

### COULD

<details>
<summary> Done </summary>

- [x] Compiles with Notion icon
- [x] Mirror the notion.ico file 😈 (slightly red tint to logo)
- [x] "Web delivery" via Flask and one-liner for remote download/exec (https://www.offensive-security.com/metasploit-unleashed/web-delivery/)
- [x] Agent checks in by POSTing hostname and username to page title with asterisk if in an admin context (getprivs at checkin)
- [x] Agent can spawn in kiosk mode Notion.so page at startup

</details>

<details>
<summary> For Next Release </summary>
  
  - [ ] Linux `persist rc.local`
  - [ ] Linux `inject` (more of a shellcode runner than injection)
  - [ ] Windows `runas` (SCshell)
  - [ ] Windows `inject-assembly` (⚠️ large lift ⚠️)
  - [ ] (Bonus) Windows `persist comhijack`
  - [ ] (Bonus) Windows `persist xll`

</details>
  
## Thanks & Acknowledgements
> This project has been a blast for me! I learned a ton about Rust and how the mechanics of a C2 work. So thank you to my co-creator @mttaggart for helping me along the way. None of this would have been possible without your technical acumen and creativity.
>
>Thank you to Joe Helle (@joehelle) for the POC steps for the fodhelper UAC bypass.
>
>Thank you to all of the great red team devs who came before me, too numerous to list them all, who have created some of my favorite tools. I’m continually inspired by the red dev innovation in our field.
>
>-Husky
>



## Disclaimer
There is no way to make an offensive security relevant research tool and release it open source without the possibility of it falling into the wrong hands. This tool is only to be used for legal, ethical purposes including, but not limited to, research, security assessment, education. The dev team is not responsible for the misuse of this tool by anyone if used for illegal/unethical purposes. No animals were harmed in the making of this code base (although Cosmo keeps climbing on my keyboard and I have to put him over on the couch, which I'm sure must feel like torture to him).

See the LICENSE for more details.

<!--
Links
-->

[issues]:https://github.com/HuskyHacks/OffensiveNotion/issues "OffensiveNotion Issues ➶"
[wiki]:https://github.com/HuskyHacks/OffensiveNotion/wiki "OffensiveNotion Documentation ➶"
[repo]:https://github.com/HuskyHacks/OffensiveNotion "OffensiveNotion Repository ➶"
[pr]:https://github.com/HuskyHacks/OffensiveNotion/pulls "OffensiveNotion Pull Requests ➶"
[license]:https://github.com/HuskyHacks/OffensiveNotion/blob/main/LICENSE "OffensiveNotion License File ➶"

<!--
Badges
-->
[lastcommit]:https://img.shields.io/github/last-commit/HuskyHacks/OffensiveNotion?style=for-the-badge
[img-pr-badge]:https://img.shields.io/badge/PRs-welcome-orange.svg?style=for-the-badge&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz48c3ZnIGlkPSJzdmcyIiB3aWR0aD0iNjQ1IiBoZWlnaHQ9IjU4NSIgdmVyc2lvbj0iMS4wIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPiA8ZyBpZD0ibGF5ZXIxIj4gIDxwYXRoIGlkPSJwYXRoMjQxNyIgZD0ibTI5Ny4zIDU1MC44N2MtMTMuNzc1LTE1LjQzNi00OC4xNzEtNDUuNTMtNzYuNDM1LTY2Ljg3NC04My43NDQtNjMuMjQyLTk1LjE0Mi03Mi4zOTQtMTI5LjE0LTEwMy43LTYyLjY4NS01Ny43Mi04OS4zMDYtMTE1LjcxLTg5LjIxNC0xOTQuMzQgMC4wNDQ1MTItMzguMzg0IDIuNjYwOC01My4xNzIgMTMuNDEtNzUuNzk3IDE4LjIzNy0zOC4zODYgNDUuMS02Ni45MDkgNzkuNDQ1LTg0LjM1NSAyNC4zMjUtMTIuMzU2IDM2LjMyMy0xNy44NDUgNzYuOTQ0LTE4LjA3IDQyLjQ5My0wLjIzNDgzIDUxLjQzOSA0LjcxOTcgNzYuNDM1IDE4LjQ1MiAzMC40MjUgMTYuNzE0IDYxLjc0IDUyLjQzNiA2OC4yMTMgNzcuODExbDMuOTk4MSAxNS42NzIgOS44NTk2LTIxLjU4NWM1NS43MTYtMTIxLjk3IDIzMy42LTEyMC4xNSAyOTUuNSAzLjAzMTYgMTkuNjM4IDM5LjA3NiAyMS43OTQgMTIyLjUxIDQuMzgwMSAxNjkuNTEtMjIuNzE1IDYxLjMwOS02NS4zOCAxMDguMDUtMTY0LjAxIDE3OS42OC02NC42ODEgNDYuOTc0LTEzNy44OCAxMTguMDUtMTQyLjk4IDEyOC4wMy01LjkxNTUgMTEuNTg4LTAuMjgyMTYgMS44MTU5LTI2LjQwOC0yNy40NjF6IiBmaWxsPSIjZGQ1MDRmIi8%2BIDwvZz48L3N2Zz4%3D
[img-license-badge]:https://img.shields.io/badge/license-mit-367588.svg?style=for-the-badge
