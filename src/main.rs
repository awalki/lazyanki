use clap::{Parser, Subcommand};
use lazyanki_anki::{
    AnkiClient,
    models::{AddNoteResponse, Fields, Note, Params},
};
use lazyanki_parser::{GermanStrategy, ParserStrategy, utils::get_client};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        #[arg(long, short = 'w')]
        word: String,
        #[arg(long, short = 'd')]
        deck: String,
    },
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let client = get_client().await?;
    let anki = AnkiClient {
        client: client,
        url: "http://localhost:8765".to_string(),
    };

    match args.command {
        Commands::New { word, deck } => {
            println!("🪄 Creating a new card: {}", word);
            let german = GermanStrategy {
                url: "https://verbformen.ru/".to_string(),
                word: word,
            };

            let word = german.parse().await?;

            let back = format!("{}\n\n{}", word.translation, word.table);

            let result = create_card(anki, word.word, back, deck).await?;

            if let Some(value) = result.result {
                println!("✅ Card created successfully: {value}");
            } else {
                println!("🟥 Failed to create card, maybe there is a duplicate");
            }
        }
        Commands::List => {
            let deck_names = anki.deck_names().await?;
            println!("🎟️ Your decks:\n\n{}", deck_names.result.join("\n- "))
        }
    }

    Ok(())
}

async fn create_card(
    anki: AnkiClient,
    word: String,
    back: String,
    deck_name: String,
) -> anyhow::Result<AddNoteResponse> {
    let params = Params {
        note: Note {
            deck_name: deck_name,
            fields: {
                Fields {
                    front: word,
                    back: back,
                }
            },
            model_name: "Basic".to_string(),

            audio: None,
            options: None,
            picture: None,
            tags: None,
            video: None,
        },
    };

    let resp = anki.add_note(params).await?;

    Ok(resp)
}
