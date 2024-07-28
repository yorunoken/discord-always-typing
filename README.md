# DISCORD-ALWAYS-TYPING
Always Typing on Discord Prank

## ⚠️ Warning ⚠️
I'd like to make it clear that this **does break discord's terms of service**.
So please **do know that you have a risk of terminating your Discord account**.

**Please use at your own risk.**

## How this program works
This program use a classic set interval with a fetch request inside.
To make it works it needs some data: your account token, and interval second, and the channel id where you want to be appearing as being typing a message.

## How and where to retrieve required data ?
For your **account token**, you'll need to go into the developer console inside discord (press **F12**, or **CTRL + MAJ + I**, or **CTRL + SHIFT I) then go into **Network** tab.
Type a message in Discord, after a second or two, you'll see a request called **typing**. Choose it, go into the **Headers** tab (it should already be there), go under **Request Headers** and find the **Authorization** Header. The value next to it is your account token.

**Disclaimer: Your token is like your password and should NEVER be shared with anyone. I'm not using it for any malicious purposes, and you can always build from source if you don't trust me.**

For the **channel ID**, you'll need to enable the developer mode into discord. You can enable it into the **Settings > Advanced** tab. Once it's done, just right click into the desired channel and click
**Copy Channel ID**. (It also works for DMs).

## Start
Install from [releases](https://github.com/yorunoken/discord-always-typing/releases), pick your binary, and run the binary.
