/// clean_sentence from a sentence to avoid being count
///
/// # Arguments
///
/// * `sentence` - A slice of string which represent a sentence
/// * `punctuations`
pub fn clean_sentence<S: AsRef<str>>(sentence: S, punctuations: &[String]) -> String {
    let mut filtered_sentence = sentence.as_ref().to_string();
    punctuations
        .iter()
        .for_each(|pat| filtered_sentence = filtered_sentence.replace(pat, ""));

    filtered_sentence
}
