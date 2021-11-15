# sentence-splitter

With `sentence-splitter` you can split your paraphrase into small sentences using NLP. This library is created based on NLTK's [Punkt Sentence Tokenizer](https://github.com/nltk/nltk/blob/develop/nltk/tokenize/punkt.py)

First, you need to download the model weight. you can download one using `utils/download-weights.py`. Run the script and pass the file on PunktSentenceTokenizer class

```python
python download-weights.py -output /home/ubuntu/Downloads -language English
```

Currently, this project support 17 languages.

```python
AVAILABLE_LANGUAGES = {
    "czech",
    "danish",
    "dutch",
    "english",
    "estonian",
    "finnish",
    "french",
    "german",
    "greek",
    "italian",
    "norwegian",
    "polish",
    "portuguese",
    "slovene",
    "spanish",
    "swedish",
    "turkish"
}
```

## Usage

```rust
let string = r#"
    Both versions convey a topic; it’s pretty easy to predict that the paragraph will be about epidemiological evidence, but only the second version establishes an argumentative point and puts it in context.
    The paragraph doesn’t just describe the epidemiological evidence; it shows how epidemiology is telling the same story as etiology.
    Similarly, while Version A doesn’t relate to anything in particular, Version B immediately suggests that the prior paragraph addresses the biological pathway of a disease and that the new paragraph will bolster the emerging hypothesis with a different kind of evidence.
    As a reader, it’s easy to keep track of how the paragraph about cells and chemicals and such relates to the paragraph about populations in different places.
    A last thing to note about key sentences is that academic readers expect them to be at the beginning of the paragraph.
    This placement helps readers comprehend your argument.
    To see how, try this: find an academic piece (such as a textbook or scholarly article) that strikes you as well written and go through part of it reading just the first sentence of each paragraph.
    You should be able to easily follow the sequence of logic.
    When you’re writing for professors, it is especially effective to put your key sentences first because they usually convey your own original thinking.
    It’s a very good sign when your paragraphs are typically composed of a telling key sentence followed by evidence and explanation.
"#;

let punkt_sentence_tokenizer = PunktSentenceTokenizer::new(
    "english.json"
);

let sentences = punkt_sentence_tokenizer.tokenize(string, true);

println!("{:#?}", sentences);
```

Output:
```
[
    "Both versions convey a topic; it’s pretty easy to predict that the paragraph will be about epidemiological evidence, but only the second version establishes an argumentative point and puts it in context.",
    "The paragraph doesn’t just describe the epidemiological evidence; it shows how epidemiology is telling the same story as etiology.",
    "Similarly, while Version A doesn’t relate to anything in particular, Version B immediately suggests that the prior paragraph addresses the biological pathway of a disease and that the new paragraph will bolster the emerging hypothesis with a different kind of evidence.",
    "As a reader, it’s easy to keep track of how the paragraph about cells and chemicals and such relates to the paragraph about populations in different places.",
    "A last thing to note about key sentences is that academic readers expect them to be at the beginning of the paragraph.",
    "This placement helps readers comprehend your argument.",
    "To see how, try this: find an academic piece (such as a textbook or scholarly article) that strikes you as well written and go through part of it reading just the first sentence of each paragraph.",
    "You should be able to easily follow the sequence of logic.",
    "When you’re writing for professors, it is especially effective to put your key sentences first because they usually convey your own original thinking.",
    "It’s a very good sign when your paragraphs are typically composed of a telling key sentence followed by evidence and explanation.",
]
```

## TODO

- [X] Add direct language support (no need to download weight separately just pass the language and code will download weight file.)
- [ ] Out of index error (mostly if `realign_boundaries` is true)

## License

MIT License.

## Credits

[NLTK](https://www.nltk.org/)
