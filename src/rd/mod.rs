use redis;
use redis::cmd;
use redis::RedisResult;
use redis::Commands;
use redis::PipelineCommands;
// use serialize::json;


pub fn create_connection(path: &str) -> redis::Client {
    redis::Client::open(path).expect("Failed to connect to redis!!")
}

pub fn execute(con: &redis::ConnectionLike, command: String, query: Vec<String>) -> Option<String> {
    let mut comm = cmd(&command);
    for arg in query.iter() {
        // println!("{}", arg);
        comm.arg(arg.trim());
    }
    if let Some(out) = try_get_string(comm.clone(), con) {
        return Some(out);
    } else {
        return try_get_array(comm, con);
    }
    // Some(String::from("coool"))
}


fn try_get_string(cmd: redis::Cmd, con: &redis::ConnectionLike) -> Option<String> {
    let result: Option<String>;
    result = cmd.query(con).unwrap_or(None);
    result
    // Some(String::from("rad"))
}

fn try_get_array(cmd: redis::Cmd, con: &redis::ConnectionLike) -> Option<String> {
    let result: Vec<String>;
    result = cmd.query(con).unwrap_or(vec![]);
    Some(result.iter().fold(
        String::new(),
        |acc, item| acc + "," + &item,
    ))
    // Some(String::from("rad"))
}