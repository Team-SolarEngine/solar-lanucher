import json
import argparse
import os

parser = argparse.ArgumentParser()
parser.add_argument("key", help="The key for the translation")
parser.add_argument("value", help="The value for en.")
args = parser.parse_args()

localization_path = "static/localization/"
localization_files = os.listdir(localization_path)

for i in localization_files:
    with open(f"{localization_path}/{i}", "r") as f:
        json_file_content = json.load(f)

    if i == "en.json":
        json_file_content[args.key] = args.value
    else:
        json_file_content[args.key] = ""

    with open(f"{localization_path}/{i}", "w") as f:
        json.dump(json_file_content, f, indent=4, ensure_ascii=False)
