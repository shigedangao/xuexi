# Xuexie 學習 (WIP)

A library which parse a dictionary in order to return a list of most used word / character within a text or a sentence.

## Supported language

All of these dictionaries are available on the internet. Big thanks to them for putting up these dictionaries. So far only these 2 languages are supported for personnal usage in future project and language learning

- Chinese by using the [cedict dictionnary](https://www.mdbg.net/chinese/dictionary?page=cedict)
- Laotian by using the [Japanese laotian dictionnary made by srachai](http://srachai.web.fc2.com)

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
let chinese = xuexi::load_chinese_dictionnary();
let sentence = "今天天氣很熱非常熱";

// hashamp
let list = chinese.get_list_detected_words(sentence).unwrap();
let definition = list.get("熱").unwrap();

println!("{}", definition.count) // this will print 2
```

### Laotian

```rs
let lao = xuexi::load_laotian_dictionnary().unwrap();
let sentence = "ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ";

let list = lao.get_list_detected_words(sentence).unwrap();
let eat = list.get("ກິນ").unwrap();

println!("{}", eat.pronounciation) // this will print "kin"
```