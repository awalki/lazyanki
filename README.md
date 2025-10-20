# LazyAnki

Don't waste your time on making flashcards, use LazyAnki instead!

## Features

- Auto-parsing cards from the dictionaries
- Card automaticly will be added to the selected deck

## Installation

```bash
cargo install lazyanki
```

### Also you have to install Anki and AnkiConnect to use LazyAnki, otherwise you will get an error
- [AnkiConnect](https://git.sr.ht/~foosoft/anki-connect)

> Your anki instance should be running while performing actions

## Usage

```
lazyanki list
"""
🎟️ Your decks:

Deutsch
- Deutsch::A1 (Grammatik)
- Deutsch::A2 (Grammatik)
- Deutsch::B1 (Grammatik)
- Deutsch::Vokabeln
- Deutsch::Vokabeln (Verbs)
- English by the Movies
- Test
"""

lazyanki new -w abholen -d "Deutsch::Vokabeln (Verbs)"
```

## Roadmap

- Support for more dictionaries
- Support for more languages (Currently German cards and Russian translation)
