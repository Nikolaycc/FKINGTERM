use std::process::{Command, Output};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
    Kill your self
";

const HELP_COMMAND: &str = "$ help";
const COMMAND: char = '$';

struct Handler;

async fn cmd(con: String) -> String {
    let mut conn = con.clone();
    let mut vconn: Vec<&str> = Vec::new();
    conn.remove(0);

    let spl = conn.split_whitespace();
    vconn = spl.collect();

    println!("{:?}", vconn);

    let mut output = Command::new(format!("{}", vconn[0]));

    if vconn.len() > 0 {
        for n in vconn.clone() {
            if vconn[0] == n {
                continue;
            } else {
                output.arg(n);
            }
        }
    }

    let out: Output = output.output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout);
        if s.len() <= 1 {
            return String::from("Done!");
        }
        return format!("```\n\t\tstdout\n{}\n```", s.to_string());
    } else {
        let s = String::from_utf8_lossy(&out.stderr);
        return format!("```\n\t\tstderr\n{}\n```", s.to_string());
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content.chars().nth(0).unwrap() == COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, cmd(msg.content).await).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
#[tokio::main]
async fn main() {
  let token = "TOKEN";

  let mut client = Client::new(&token)
  .event_handler(Handler)
  .await
  .expect("Err creating client");

  if let Err(why) = client.start().await {
      println!("Client error: {:?}", why);
  }
}