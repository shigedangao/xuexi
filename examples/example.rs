use xuexi::ordering::Ops;
use xuexi::word::DetectWord;

fn main() {
    chinese_example();
    lao_example();
}

fn chinese_example() {
    let chinese = xuexi::load_chinese_dictionary().unwrap();
    let sentence = "今天天氣很熱非常熱";

    // hashamp
    let list = chinese.get_list_detected_words(sentence).unwrap();
    
    // we can get the list ordered as a vector
    let vec: Vec<_> = list.get_ordered_characters();
    
    let (character, definition) = vec.get(0).unwrap();

    assert_eq!(character, "熱");
    assert_eq!(definition.count, 2);
    assert_eq!(definition.pronunciation, "re4");

    println!("{:?}", definition);
}

fn lao_example() {
    let lao = xuexi::load_laotian_dictionary().unwrap();
    let sentence = "ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ";

    let list = lao.get_list_detected_words(sentence).unwrap();
    let eat = list.get("ກິນ").unwrap();

    assert_eq!(eat.writing_method, "ກິນ");
    assert_eq!(eat.pronunciation, "kin");
    assert_eq!(eat.english, "eat");

    println!("{:?}", eat);
}