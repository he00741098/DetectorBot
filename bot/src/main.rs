use json::JsonValue;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler {
    hf_token: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{:?}", msg.attachments);
        let mut delete = false;
        for ele in msg.clone().attachments {
            let body = reqwest::get(ele.url).await;
            if let Ok(image) = body {
                let bytes = image.bytes().await;
                let client = reqwest::Client::new();
                if bytes.is_err() {
                    println!("Image is error");
                    return;
                }
                let res = client.post("https://api-inference.huggingface.co/models/Falconsai/nsfw_image_detection").body(bytes.unwrap()).header("Authorization", &self.hf_token).send().await;
                if res.is_err() {
                    println!("RES ERRORED: \n{:?}", res);
                } else {
                    let text = res.unwrap().text().await;
                    println!("RES OK: \n{:?}", text);
                    if let Ok(text) = text {
                        let response = json::parse(&text);
                        if let Ok(res) = response {
                            if let JsonValue::Array(array) = res {
                                if array.len() == 2 {
                                    let one = array[0].clone();
                                    let two = array[1].clone();
                                    if let JsonValue::Object(ob1) = one {
                                        if let JsonValue::Object(ob2) = two {
                                            println!("OB1 {:?}\nOB2 {:?}", ob1, ob2);
                                            let mut nsfw_score: f64 = 500.0;
                                            // let mut normal_score: f64 = 500.0;
                                            if let Some(str) = ob1.get("label") {
                                                let str = match str {
                                                    JsonValue::String(s) => s.to_string(),
                                                    JsonValue::Short(s) => s.to_string(),
                                                    _ => "null".to_string(),
                                                };
                                                if str == "normal" {
                                                    // let score = ob1.get("score");
                                                    // if let Some(JsonValue::Number(num)) = score {
                                                    //     normal_score = (*num).into();
                                                    // } else {
                                                    //     println!(
                                                    //         "Score is not number: {:?}",
                                                    //         score
                                                    //     );
                                                    // }
                                                } else if str == "nsfw" {
                                                    let score = ob1.get("score");
                                                    if let Some(JsonValue::Number(num)) = score {
                                                        nsfw_score = (*num).into();
                                                    } else {
                                                        println!(
                                                            "Score is not number: {:?}",
                                                            score
                                                        );
                                                    }
                                                } else {
                                                    println!("Label did not match: {}", str)
                                                }
                                            } else {
                                                println!("Could not get label");
                                            }

                                            if let Some(str) = ob2.get("label") {
                                                let str = match str {
                                                    JsonValue::String(s) => s.to_string(),
                                                    JsonValue::Short(s) => s.to_string(),
                                                    _ => "null".to_string(),
                                                };
                                                if str == "normal" {
                                                    // let score = ob2.get("score");
                                                    // if let Some(JsonValue::Number(num)) = score {
                                                    //     normal_score = (*num).into();
                                                    // } else {
                                                    //     println!(
                                                    //         "Score is not number: {:?}",
                                                    //         score
                                                    //     );
                                                    // }
                                                } else if str == "nsfw" {
                                                    let score = ob2.get("score");
                                                    if let Some(JsonValue::Number(num)) = score {
                                                        nsfw_score = (*num).into();
                                                    } else {
                                                        println!(
                                                            "Score is not number: {:?}",
                                                            score
                                                        );
                                                    }
                                                } else {
                                                    println!("Label did not match: {}", str)
                                                }
                                            } else {
                                                println!("Could not get label");
                                            }

                                            if nsfw_score <= 2.0 && nsfw_score > 0.8 {
                                                delete = true;
                                                break;
                                            }
                                        } else {
                                            println!("Expected Two to be an object");
                                        }
                                    } else {
                                        println!("Expected One to be an object");
                                    }
                                } else {
                                    println!("Unexpected array length");
                                }
                            }
                        } else {
                            println!("Response could not be parsed");
                        }
                    } else {
                        println!("Response is not text");
                    }
                }
            } else {
                println!("Body is not OK");
            }
        }
        if delete {
            let delete_result = msg.delete(&ctx.http).await;
            println!("delete: {:?}", delete_result);
        }

        if msg.content == "!test" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Active!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token =
        std::env::var("DISCORD_TOKEN").expect("Please Set the DISCORD_TOKEN environment variable");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            hf_token: format!(
                "Bearer {}",
                std::env::var("HF_TOKEN").expect("Please Set the HF_TOKEN environment variable")
            ),
        })
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
