# A very basic image filtering bot

This bot attempts to delete all NSFW images posted in any channel that the bot is allowed in.
There is currently no configuration unless directly modifying the code

# Running the bot
You will first need to set up the bot in the discord developer portal and obtain the api token.
Next, you will need to get a access token from Hugging Face.

Clone this repository and open the folder in the terminal and cd into the bot folder.

Export the discord api token as DISCORD_TOKEN (export DISCORD_TOKEN=xxxx)
and the Hugging Face access token as HF_TOKEN (export HF_TOKEN=xxxx)

If you do not have rust installed, visit https://rustup.rs

Then run: `cargo run --release` to run the program.
