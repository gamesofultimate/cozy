import requests
import json
import os

USERNAME = "🤖 Deployment Bot"
GAME_NAME = "Moonfallen"
STAGING_URL = "https://stg.ultimate.games/game/branches"
GAME_ID = "2b0d0f16-5de5-46c5-93e6-137c541c0272"

DISCORD_WEBHOOK_URL = os.environ.get("DISCORD_WEBHOOK_URL")

BUILDKITE_BRANCH = os.environ.get("BUILDKITE_BRANCH")
BUILDKITE_MESSAGE = os.environ.get("BUILDKITE_MESSAGE")
BUILDKITE_BUILD_AUTHOR = os.environ.get("BUILDKITE_BUILD_AUTHOR")

if not DISCORD_WEBHOOK_URL:
    raise ValueError("DISCORD_WEBHOOK_URL is not set")

MAIN_THEME = {  
    "color": 5688562, #56CCF2 - Cyan
    "prefix": "Main: "
}

HOTFIX_THEME = {  
    "color": 16711680, #FF0000 - Red
    "prefix": "Hotfix: "
}

DEVELOP_THEME = {  
    "color": 16776960, #FFFF00 - Yellow
    "prefix": "Develop: "
}

OTHER_THEME = {  
    "color": 	16777215, #FFFFFF - White
    "prefix": "Branch: "
}

current_theme = OTHER_THEME

if BUILDKITE_BRANCH == "master":
    current_theme = MAIN_THEME
if BUILDKITE_BRANCH == "main":
    current_theme = MAIN_THEME
if BUILDKITE_BRANCH.startswith("hotfix/"):
    current_theme = HOTFIX_THEME
if BUILDKITE_BRANCH == "develop":
    current_theme = DEVELOP_THEME

description = f"A new deployment is available. [Play now!]({STAGING_URL}/{GAME_ID})"
description += f"\n\nBranch: `{BUILDKITE_BRANCH}`"
description += f"\nCommit: `{BUILDKITE_MESSAGE}` by {BUILDKITE_BUILD_AUTHOR}"

embed = {
    "title": f"{current_theme['prefix']} {GAME_NAME}",
    "description": description,
    "color": current_theme["color"],
}

data = {
    "username": USERNAME,
    "embeds": [embed],
}

headers = {
    "Content-Type": "application/json"
}

response = requests.post(DISCORD_WEBHOOK_URL, headers=headers, data=json.dumps(data))

