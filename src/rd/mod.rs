use redis;
use redis::cmd;
use redis::RedisResult;
use redis::Commands;
use redis::PipelineCommands;


pub fn create_connection(path: &str) -> redis::Client {
    redis::Client::open(path).expect("Failed to connect to redis!!")
}

pub fn execute(
    con: &redis::ConnectionLike,
    command: String,
    query: Vec<String>,
) -> RedisResult<String> {
    let mut comm = cmd(&command);
    for arg in query.iter() {
        comm.arg(arg);
    }
    comm.query(con)
}
