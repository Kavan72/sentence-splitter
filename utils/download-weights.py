import os
import json
import nltk
import argparse

AVAILABLE_LANGUAGES = {
    "czech",
    "danish",
    "dutch",
    "english",
    "estonian",
    "finnish",
    "french",
    "german",
    "greek",
    "italian",
    "norwegian",
    "polish",
    "portuguese",
    "slovene",
    "spanish",
    "swedish",
    "turkish"
}


def save_json_file(json_data: dict, output: str):
    with open(output, 'w') as fp:
        fp.write(json.dumps(json_data, sort_keys=True, indent=4))

    print(f"Weight file is created at `{output}`")


def main(language: str, output: str):
    nltk.download('punkt')
    tokenizers = nltk.data.load(
        os.path.join("tokenizers", "punkt", f"{language}.pickle")
    )
    save_json_file(
        {
            "abbrev_types": list(tokenizers._params.abbrev_types),
            "collocations": list(tokenizers._params.collocations),
            "sent_starters": list(tokenizers._params.sent_starters),
            "ortho_context": dict(tokenizers._params.ortho_context)
        },
        os.path.join(output, f"{language}.json")
    )


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '-output',
        help='Absolute path to the save weights file.'
    )
    parser.add_argument(
        '-language',
        type=lambda s: s.lower(),
        choices=AVAILABLE_LANGUAGES,
        help='Which you need to download weight file for that language.'
    )
    main(**vars(parser.parse_args()))
