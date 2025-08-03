# noqa: INP001

import json
import os

from dateparser.custom_language_detection.fasttext import detect_languages
from dateparser.search import search_dates


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
    while True:
        length = int.from_bytes(read(4), "big")
        data = read(length)

        message = json.loads(data)
        results = parse(message.get("content"), message.get("tz"))
        response = serialize(results).encode("utf-8")

        write(response)
