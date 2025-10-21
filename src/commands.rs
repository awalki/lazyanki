use lazyanki_anki::{
    AnkiClient,
    models::{AddNoteResponse, Fields, Note, Params},
};
use lazyanki_common::{create_config, get_anki_client};
use lazyanki_parser::get_strategy;

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

pub async fn init_command(native_language: String, target_language: String) -> anyhow::Result<()> {
    create_config(native_language, target_language)?;

    Ok(())
}

pub async fn new_command(word: String, deck: String) -> anyhow::Result<()> {
    let anki = get_anki_client().await?;
    println!("🪄 Creating a new card: {}", word);
    let strategy = get_strategy(word).await?;

    let word = strategy.parse().await?;

    let back = format!("{}\n\n{}", word.translation, word.table);

    let result = create_card(anki, word.word, back, deck).await?;

    if let Some(value) = result.result {
        println!("✅ Card created successfully: {value}");
    } else {
        println!("🟥 Failed to create card, maybe there is a duplicate");
    }

    Ok(())
}

pub async fn list_command() -> anyhow::Result<()> {
    let anki = get_anki_client().await?;
    println!(
        "🎟️ Your decks:\n\n{}",
        anki.deck_names().await?.result.join("\n- ")
    );
    Ok(())
}
