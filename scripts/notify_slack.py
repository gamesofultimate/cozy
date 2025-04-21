import requests
import json
import os

USERNAME = "🤖 Deployment Bot"
GAME_NAME = "Moonfallen"
STAGING_URL = "https://stg.ultimate.games/game/branches"
GAME_ID = "2b0d0f16-5de5-46c5-93e6-137c541c0272"

SLACK_WEBHOOK_URL = os.environ.get("SLACK_WEBHOOK_URL")
BUILDKITE_BRANCH = os.environ.get("BUILDKITE_BRANCH")
BUILDKITE_MESSAGE = os.environ.get("BUILDKITE_MESSAGE")
BUILDKITE_BUILD_AUTHOR = os.environ.get("BUILDKITE_BUILD_AUTHOR")

if not SLACK_WEBHOOK_URL:
    raise ValueError("SLACK_WEBHOOK_URL is not set")

MAIN_THEME = {  
    "color": 5688562, #56CCF2 - Cyan
    "color_hex": "#56CCF2",
    "prefix": "Main: "
}

HOTFIX_THEME = {  
    "color": 16711680, #FF0000 - Red
    "color_hex": "#FF0000",
    "prefix": "Hotfix: "
}

DEVELOP_THEME = {  
    "color": 16776960, #FFFF00 - Yellow
    "color_hex": "#FFFF00",
    "prefix": "Develop: "
}

OTHER_THEME = {  
    "color": 16777215, #FFFFFF - White
    "color_hex": "#FFFFFF",
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

headers = {
    "Content-Type": "application/json"
}

title = {
    "type": "header",
    "text": {
        "type": "plain_text",
        "text": f"{current_theme['prefix']} {GAME_NAME}",
    }
}

url = {
    "type": "section",
    "text": {
        "type": "mrkdwn",
        "text": f"A new deployment is available. <{STAGING_URL}/{GAME_ID}|Play now!>"
    }
}

description = f"Branch: `{BUILDKITE_BRANCH}`"
description += f"\nCommit: `{BUILDKITE_MESSAGE}` by {BUILDKITE_BUILD_AUTHOR}"

commit = {
    "type": "section",
    "text": {
        "type": "mrkdwn",
        "text": description
    }
}

data = {
    "username": USERNAME,
    "attachments": [{
        "color": current_theme["color_hex"],
        "blocks": [title, url, commit],
    }]
}

response = requests.post(SLACK_WEBHOOK_URL, headers=headers, data=json.dumps(data))
