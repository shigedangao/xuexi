use std::path::PathBuf;
use std::thread;
use xuexi::{
    dictionary::{Chinese, Dictionary, Lang, Laotian},
    word::WordParser,
    KeyVariant,
};

fn main() {
    let (cn, la) = load_dictionary();

    chinese_example(&cn);
    lao_example(&la);
}

// Load dictionary in thread to make the example quicker
fn load_dictionary() -> (Dictionary<Chinese>, Dictionary<Laotian>) {
    let cn_handle = thread::spawn(|| {
        xuexi::load_chinese_dictionary(
            Lang::Chinese(KeyVariant::Traditional),
            PathBuf::from("./cedict_ts.u8"),
        )
        .unwrap()
    });

    let la_handle = thread::spawn(|| {
        xuexi::load_laotian_dictionary(PathBuf::from("./lao-eng-dictionary.csv")).unwrap()
    });

    let (cn, la) = (cn_handle.join(), la_handle.join());

    (cn.unwrap(), la.unwrap())
}

fn chinese_example(chinese: &Dictionary<Chinese>) {
    let sentence = "今天天氣很熱非常熱";

    // hashamp
    let list = chinese.parse_sentence_into_words(sentence);
    let definition = list.get("熱").unwrap();

    dbg!(definition);

    assert_eq!(definition.written, vec!["熱", "热"]);
    assert_eq!(definition.count, 2);
    assert_eq!(definition.pronunciations.get(0).unwrap(), "re4");
}

fn lao_example(lao: &Dictionary<Laotian>) {
    let sentence = "ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ";

    let list = lao.parse_sentence_into_words(sentence);
    let eat = list.get("ກິນ").unwrap();

    dbg!(eat);

    assert_eq!(eat.written, vec!["ກິນ"]);
    assert_eq!(eat.pronunciations.get(0).unwrap(), "kin");
    assert_eq!(eat.translations.get(0).unwrap(), "eat");
}
