---
title: DICTIONARY-CLI
section: 1
header: User Manual
footer: dictionary-cli 1.1.0
date: June 26, 2021
---

#NAME
dictionary-cli - Find definition of words in terminal

# SYNOPSIS
**dictionary-cli** [*OPTION*] [*INPUT*]

# DESCRIPTION
**dictionary-cli** is a Rust application to find meaning, example, phonetic of words. Currently, this project uses free dictionary API from [freeDictionaryAPI](https://github.com/meetDeveloper/freeDictionaryAPI). You can support them by donating.

# OPTIONS
**-h**
: Display help message

**-v**
: Display version

**-d**
: Display definitios of the word

**-p**
: Display phonetics of the word

**-e**
: Display examples of the word

**-s**
: Display similar words

# INPUT
**INPUT**
: The word that you seek

# EXAMPLES
**dictionary-cli something -d**
: Display only definitions of the word

**dictionary-cli other -d -s**
: Display both definitions and similar words of the word

**dictionary-cli anything**
: Display definitions, phonetics, examples and similar words of ther word

# AUTHORS
Written by Nguyen Hoang Nam

# SEE ALSO
Full documentation and sources at: <https://github.com/Nguyen-Hoang-Nam/dictionary-cli>
