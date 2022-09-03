# Dictionary-CLI

Find definition of words in terminal

![Main](https://raw.githubusercontent.com/Nguyen-Hoang-Nam/readme-image/main/dictionary-cli/dictionary-cli.png)

## Supported

### Free dictionary

You can donate for their work here [freeDictionaryAPI](https://github.com/meetDeveloper/freeDictionaryAPI).

### Oxford dictionary

You need to create an account in [here](https://developer.oxforddictionaries.com/)
then request a credential key before use this feature. After that, create 2
environments like below and fill with your credential key.

```bash
export OXFORD_APP_ID=id
export OXFORD_APP_KEY=key
```

## Installation

```text
$ cargo install --git https://github.com/Nguyen-Hoang-Nam/dictionary-cli
```

If you want man page when run.

```text
bash man.sh
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

### List supported API

```text
$ dictionary-cli --list-api
```

### Use other API

To change api use flag `--api`, the default value is `free`.

```text
$ dictionary-cli --api oxford something
```

## TODO

- [ ] Support download sound
- [ ] Support [Collins API](https://www.collinsdictionary.com/collins-api)
- [ ] Support [Merriam Webster API](https://dictionaryapi.com/products/api-learners-dictionary)
- [ ] Support [Urban API](https://api.urbandictionary.com/v0)
- [ ] Support [Yandex API](https://yandex.com/dev/dictionary/)
- [ ] Support [Cambridge](https://dictionary-api.cambridge.org/)
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
