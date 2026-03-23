use iced::clipboard::read_primary;
use iced::time::{self, Duration};
use iced::widget::{
    button, column, container, row, scrollable,
    text::{Rich, Span},
    text_input, tooltip,
};
use iced::window;
use iced::{theme::Theme, Alignment, Element, Length, Settings, Subscription, Task};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
mod colors;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.default_text_size = 12.into();

    let mut window = window::Settings::default();
    window.size = iced::Size::new(300.0, 200.0);
    iced::application(Artha::new, Artha::update, Artha::view)
        .settings(settings)
        .window(window)
        .title(Artha::title)
        .theme(Artha::theme)
        .subscription(Artha::subscription)
        .run()
}

#[derive(Default, Deserialize, Clone)]
struct Definition {
    grammar: Option<String>,
    senses: Vec<String>,
}

#[derive(Default, Deserialize, Clone)]
struct Word {
    word: String,
    definitions: Vec<Definition>,
}

impl Word {
    fn view(&self, all_wrds: &HashMap<String, Word>) -> Rich<'_, String, Message> {
        let mut spans = vec![
            Span::new(" * ").size(14),
            Span::new(&self.word).color(colors::WORD).size(14),
            Span::new("\n"),
        ];
        self.definitions.iter().for_each(|m| {
            if let Some(g) = &m.grammar {
                spans.push(Span::new("  ").size(14));
                spans.push(Span::new(g).color(colors::READING).size(16));
                spans.push(Span::new("\n").size(14));
            }
            m.senses.iter().for_each(|s| {
                for wrd in s.split(' ') {
                    if all_wrds.contains_key(wrd) {
                        spans.push(Span::new(wrd).color(colors::MEANING).link(wrd.to_string()));
                    } else {
                        spans.push(Span::new(wrd).link(wrd.to_string()));
                    }
                    spans.push(Span::new(" "));
                }
                spans.push(Span::new("\n"));
            });
        });
        Rich::with_spans(spans).on_link_click(Message::WordClicked)
    }
}

#[derive(Default)]
struct Artha {
    watching: bool,
    search: String,
    current_word: Option<Word>,
    all_words: HashMap<String, Word>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SearchPressed,
    WatchMode,
    CheckClipboard,
    ClipChanged(String),
    WordClicked(String),
    // NextClicked,
    // PreviousClicked,
}

impl Artha {
    fn new() -> Self {
        let file = File::open("sabdakosh.json").unwrap();
        let reader = BufReader::new(file);
        let all_words: Vec<Word> = serde_json::from_reader(reader).unwrap();
        let all_words = all_words
            .into_iter()
            .map(|w| (w.word.to_string(), w))
            .collect();
        Self {
            watching: false,
            search: "".into(),
            current_word: Some(Word {
                word: "सब्द खोज्नुहोस।".into(),
                definitions: vec![Definition {
                    grammar: Some("ना.".into()),
                    senses: vec!["अर्थ १".into(), "अर्थ २".into()],
                }],
            }),
            all_words,
        }
    }

    fn search(&mut self) {
        self.current_word = self.all_words.get(self.search.trim()).cloned();
    }

    fn title(&self) -> String {
        String::from("शब्दकोष")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchPressed => {
                self.search();
            }
            Message::InputChanged(inp) => {
                self.search = inp;
            }
            Message::WordClicked(wrd) => {
                self.search = wrd;
                self.search();
            }
            Message::ClipChanged(inp) => {
                if inp != self.search {
                    self.search = inp;
                    return Task::done(Message::SearchPressed);
                }
            }
            Message::WatchMode => {
                self.watching = !self.watching;
            }
            Message::CheckClipboard if self.watching => {
                return read_primary().then(|r| match r {
                    Some(txt) => return Task::perform(async { txt }, Message::ClipChanged),
                    _ => Task::none(),
                })
            }
            _ => (),
        }

        Task::none()
    }

    fn view(&'_ self) -> Element<'_, Message> {
        column![
            if self.watching {
                row![
                    tooltip(
                        button("X").on_press(Message::WatchMode),
                        "End Sync with Selection",
                        tooltip::Position::Top
                    )
                    .style(container::rounded_box),
                    text_input("Word", &self.search).width(Length::Fill),
                ]
                .width(Length::Fill)
            } else {
                row![
                    tooltip(
                        button("A").on_press(Message::WatchMode),
                        "Sync from Selection",
                        tooltip::Position::Top
                    )
                    .style(container::rounded_box),
                    text_input("Word", &self.search)
                        .width(Length::Fill)
                        .on_input(Message::InputChanged)
                        .on_submit(Message::SearchPressed),
                    button("Search").on_press(Message::SearchPressed),
                ]
                .width(Length::Fill)
            },
            container(
                scrollable(if let Some(cw) = &self.current_word {
                    cw.view(&self.all_words).width(Length::Fill)
                } else {
                    Rich::with_spans(vec![Span::new("शब्द भेटीएन।").color(colors::READING)])
                })
                .width(Length::Fill)
            )
            .padding(10)
        ]
        .padding(10)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(600)).map(|_| Message::CheckClipboard)
    }
}
