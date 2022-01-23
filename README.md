# OffensiveNotion
Notion as a platform for offensive operations

## Wait, what?
Yes.

## But Why?
What started as a meme grew into a full project. Just roll with it.

### Setup
To use Notion as a platform for offensive operations, you need a few things:

- A Notion account, available [here](https://www.notion.so/signup)
- A Notion Developers API account, available [here](https://developers.notion.com/)

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

5) Build your agent. From the root directory, enter:
```
$ cargo build [--release]
```