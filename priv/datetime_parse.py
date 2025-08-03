# noqa: INP001

import json
import os
import sys
from typing import final

from dateparser.custom_language_detection.fasttext import detect_languages
from dateparser.search import search_dates


@final
class FilteredStderr:
    def __init__(self):
        self.original = sys.stderr

    def write(self, text):
        if text == (
            "Warning : `load_model` does not return WordVectorModel or "
            "SupervisedModel any more, but a `FastText` object which is very similar."
        ):
            return

        self.original.write(text)

    def __getattr__(self, name):
        return getattr(self.original, name)


def read(length):
    return os.read(3, length)


def write(data):
    data_length = len(data).to_bytes(4, "big")
    os.write(4, data_length + data)


def parse(content, tz):
    return search_dates(
        content,
        settings={"TIMEZONE": tz, "CACHE_SIZE_LIMIT": 1, "DEFAULT_LANGUAGES": ["en"]},
        detect_languages_function=detect_languages,
    )


def serialize(results):
    results = results or []

    return json.dumps(
        [
            {"substring": substring, "date": date.isoformat()}
            for substring, date in results
        ]
    )


if __name__ == "__main__":
    sys.stderr = FilteredStderr()

    while True:
        length = int.from_bytes(read(4), "big")
        data = read(length)

        message = json.loads(data)

        results = parse(message.get("content"), message.get("tz"))
        response = serialize(results).encode("utf-8")

        write(response)
