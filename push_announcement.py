import sys
import urllib.request
import argparse
import json

# Argument passer for the Discord webhook URL
#   we use a arg parser because putting a webhook out in the wild
#   is dangerous and can spread attacks... so this is our best
#   choice. we pass it through like this
# 
#     python push_announcement.py <webhook_url:str> <tag:str> <test:bool>
# 
#   and we use the github secret to put the webhook url in the environment
#   so no one other than the github members of the organization can use it
# 
#   few notes for testers;
#     false = actual announcement
#     true = test mode (no one gets pinged)
parser = argparse.ArgumentParser()
parser.add_argument("webhook", help="Discord Webhook URL")
parser.add_argument("tag", help="Github tag to announce")
parser.add_argument("test", help="Test mode", type=lambda v: v.lower() in ("true", "1", "yes", "absolutely", "plsyes"), default=True)
args = parser.parse_args() #                       ^---------------hacky shit to get it ACTUALLY work.---------------^

# check if webhook and tag are provided
if not args.webhook or not args.tag:
    print("Webhook and tag are required")
    sys.exit(1)

# create the data to send
data = json.dumps({
    "content": (
        "<@&1531292825071653035> <@&1354140261294280798>"
        if not args.test
        else "This is a test."
    ),
    "embeds": [
        {
            "description": (
                f"# Solar Launcher; {args.tag}\n"
                f"A new **Solar Launcher** version has been dropped!\n\n"
                f"Download the new version of the [launcher here](https://github.com/Team-SolarEngine/solar-lanucher/releases/tag/{args.tag}).\n"
                f"Check out [CHANGELOG.md](https://github.com/Team-SolarEngine/solar-lanucher/blob/main/CHANGELOG.md#{args.tag.replace('.', '')}) to check out what we changed!"
            ),
            "color": 14495476,
        }
    ],
}).encode("utf-8")

# create the request
request = urllib.request.Request(
    args.webhook,
    method="POST",
    headers={
        "Content-Type": "application/json",
        "User-Agent": f"SolarLauncher/{args.tag}"
    },
    data=data
)

# sends the webhook request
with urllib.request.urlopen(request) as response:
    result = response.read().decode()

print(result)