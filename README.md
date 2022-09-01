# Dictionary-CLI

📖 Find definition of words in terminal

![Main](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/dictionary-cli/dictionary-cli.png)

## Installation

```bash
$ cargo install --git https://github.com/Nguyen-Hoang-Nam/dictionary-cli
```

If you want man page when run

```base
bash man.sh
```

## Usage

### Show all

```bash
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

```bash
$ dictionary-cli something -p

something (/ˈsəmˌθɪŋ/):
```

## TODO

- [ ] Support download sound
- [ ] Support other dictionary APIs
- [ ] Config cache file's directory
- [ ] Support completion for zsh

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Credit

This project uses free dictionary API from [freeDictionaryAPI](https://github.com/meetDeveloper/freeDictionaryAPI). You can support them by donating.
