# Xuexi 學習

A library which parse a dictionary in order to return a list of most used word / character within a text or a sentence.

## Supported language

All of these dictionaries are available on the internet. Big thanks to them for putting up these dictionaries. So far only these 2 languages are supported for personnal usage in future project and language learning

- Chinese by using the [cedict dictionnary](https://www.mdbg.net/chinese/dictionary?page=cedict)
- Laotian dictionary provided by the [LaoNLP repository](https://github.com/wannaphong/LaoNLP/blob/af9bae55b7265c740855787960ba6c1a357063fd/laonlp/corpus/lao-eng-dictionary.csv). Note that it has been edited to remove the index
- Lao words provided by [lao-dictionary repository](https://github.com/rober42539/lao-dictionary) and parsing provided by the [chamkho repository](https://github.com/veer66/chamkho)

## Example

An example can be found in the `examples` folder. The example can be run with the command

```sh
cargo run --example example --features="chinese, laotian"
```

## Usage

The library is feature gates. Below are example for the supported languages

### Chinese

In the Cargo.toml file, add these line below

```toml
xuexi = { version = "0.1.0", features = ["chinese"] }
```

Then you can import the dictionary like so

```rs
let chinese = xuexi::load_chinese_dictionary(
    Lang::Chinese(KeyVariant::Traditional),
    PathBuf::from("./cedict_ts.u8"),
).unwrap();

let sentence = "今天天氣很熱非常熱";

// hashamp
let list = chinese.parse_sentence_into_words(sentence);
let definition = list.get("熱").unwrap();

println!("{}", definition.count) // this will print 2
```

### Laotian

```rs
let lao = xuexi::xuexi::load_laotian_dictionary(PathBuf::from("./lao-eng-dictionary.csv")).unwrap();
let sentence = "ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ";

let list = lao.parse_sentence_into_words(sentence);
let eat = list.get("ກິນ").unwrap();

println!("{}", eat.pronounciation) // this will print "kin"
```
