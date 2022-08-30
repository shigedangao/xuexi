use std::thread;
use xuexi::dictionary::{
    Dictionary,
    Chinese,
    Laotian
};
use xuexi::ordering::Ops;
use xuexi::word::DetectWord;

fn main() {
    let (cn, la) = load_dictionary();

    chinese_example(&cn);
    lao_example(&la);
}

// Load dictionary in thread to make the example quicker
fn load_dictionary() -> (Dictionary<Chinese>, Dictionary<Laotian>) {
    let cn_handle = thread::spawn(|| xuexi::load_chinese_dictionary(xuexi::chinese::Version::Traditional).unwrap());
    let la_handle = thread::spawn(|| xuexi::load_laotian_dictionary().unwrap());

    let (cn, la) = (cn_handle.join(), la_handle.join());

    (cn.unwrap(), la.unwrap())
}

fn chinese_example(chinese: &Dictionary<Chinese>) {
    let sentence = "今天天氣很熱非常熱";

    // hashamp
    let list = chinese.get_list_detected_words(sentence).unwrap();
    
    // we can get the list ordered as a vector
    let vec: Vec<_> = list.get_ordered_characters();
    
    let (character, definition) = vec.get(0).unwrap();

    assert_eq!(character, "熱");
    assert_eq!(definition.count, 2);
    assert_eq!(definition.pronunciations.get(0).unwrap(), "rè");

    println!("{:?}", definition);
}

fn lao_example(lao: &Dictionary<Laotian>) {
    let sentence = "ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ";

    let list = lao.get_list_detected_words(sentence).unwrap();
    let eat = list.get("ກິນ").unwrap();

    assert_eq!(eat.writing_method, "ກິນ");
    assert_eq!(eat.pronunciations.get(0).unwrap(), "kin");
    assert_eq!(eat.translations.get(0).unwrap(), "eat");

    println!("{:?}", eat);
}
