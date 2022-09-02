# Dictionary-CLI

Find definition of words in terminal

![Main](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/dictionary-cli/dictionary-cli.png)

## Installation

```text
$ cargo install --git https://github.com/Nguyen-Hoang-Nam/dictionary-cli
```

If you want man page when run.

```text
bash man.sh
```

### Oxford dictionary

Oxford dictionary need 2 environment variables to work.

```bash
export OXFORD_APP_ID=id
export OXFORD_APP_KEY=key
```

## Usage

### Show all

```text
$ dictionary-cli something

something (/ˈsəmˌθɪŋ/):
(adverd)
	_ Used for emphasis with a following adjective functioning as an adverb.

	_ To some extent; somewhat.

(pronoun)
	_ A thing that is unspecified or unknown.

	_ Used in various expressions indicating that a description or amount being stated is not exact.
```

### Show only phonetics

```text
$ dictionary-cli something -p

something (/ˈsəmˌθɪŋ/):
```

## TODO

- [ ] Support download sound
- [ ] Support google translate api
- [ ] Update cache file
- [ ] Configure cache file's directory
- [ ] Support completion for zsh

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Credit

This project uses free dictionary API from [freeDictionaryAPI](https://github.com/meetDeveloper/freeDictionaryAPI). You can support them by donating.
