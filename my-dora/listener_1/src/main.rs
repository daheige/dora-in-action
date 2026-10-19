use dora_node_api::{DoraNode, Event};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (node, mut events) = DoraNode::init_from_env()?;

    println!("Listener node started, waiting for input...");
    println!("node id: {}", node.id());
    while let Some(event) = events.recv() {
        match event {
            // 匹配event类型为Event::Input，获取输入数据
            Event::Input { id, metadata, data } => match id.as_str() {
                "speech" => {
                    let message: &str = (&data).try_into()?;
                    println!("current talker id:{id} metadata:{metadata:?}");
                    println!("I heard: {message} from {id}");
                }
                other => println!("Received input `{other}`"),
            },
            Event::Stop(_) => {
                println!("Received stop!");
            }
            _ => {}
        }
    }

    Ok(())
}
