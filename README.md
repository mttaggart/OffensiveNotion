# OffensiveNotion
Notion (yes, the notetaking app) as a C2.

## Wait, What?
Yes.

## But Why?
What started as a meme grew into a full project. Just roll with it.

### Setup
To use Notion as a platform for offensive operations, you need a few things:

- A Notion account, available [here](https://www.notion.so/signup)
- A Notion Developers API account, available [here](https://developers.notion.com/)
- (if installing from source) 🦀 **Rust** 🦀. Head to [rustup](https://rustup.rs/) and run:
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Setting Up A Listener Page
The "listener" is just a page in a Notion notebook. But you can set it up to catch the callbacks for your agents:

1) Create your listener page. Simply add a new page to Notion, preferably in a notebook that's not being used for anything else:

![img_1.png](assets/img_1.png)

2) In the upper right corner, click "Share" and "Invite". Add your Notion Developer API account to this page:

![img_2.png](assets/img_2.png)

![img_3.png](assets/img_3.png)

3) Copy the URL of your page down. If you're in the web browser Notion client, this can be taken from the URL of the page. In the desktop app, enter `ctl-l` to copy it to your clipboard.
4) If your listener URL is:
```
https://www.notion.so/LISTENER-11223344556677889900112233445566                     
```
... then your **parent page ID** is the number after the name of the listener, split with hyphens into the following schema: 8-4-4-4-12.
Meaning, your parent page ID would be: `11223344-5566-7788-9900-112233445566`. This value is used to connect your agent to your listener, so keep track of it!

## Python Installer Script (QuickStart)
The `main.py` script handles all setup and agent compilation.

Simply install dependencies:
```
$ pip3 install poetry
$ poetry shell
$ poetry install
```
Then run the main script:
```
$ sudo python3 main.py
```
...and follow the prompts to perform the installation. It creates a Docker container and creates the agent inside, then copies it to your physical host and deletes the container.

## Agent Control
When your agent is built and dispatched to the target environment, it uses your secret key and the parent page ID to check in. Once this happens, a new page with the hostname of where the agent landed will appear as a child page of your listener:

![img.png](assets/img_4.png)

When you click on this page, you are now in the agent's "session". Which is really just a notebook page for the most part, but has some special functions.

### Running commands
Make a `To Do` block and add any shell command you want to run on the target. When you are ready to execute the command, add the bullseye emoji to the end: 🎯

![img.png](assets/img_5.png)

When your agent checks in, the stdout of the command will be added to the notebook under the `To Do` block as code-syntax highlighted text.

![img.png](assets/img_6.png)

Hey, remember this is a normal Notion page! Add some documentation for your operation while you go:

![img.png](assets/img_7.png)

Why not stack up a bunch of commands to do initial check-in safety checks...

![img.png](assets/img_8.png)

... and then run them all when an agent checks in?

![img.png](assets/img_9.png)

## TODO
### Setup
- [ ] Python Setup Script for config options
- [ ] Dynamic Docker container spin up/tear down for agent generation
- [ ] Parse args for Docker build options