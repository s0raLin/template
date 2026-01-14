use anyhow::Result;

pub fn execute(name: &str, times: u32) -> Result<()> {
    for i in 1..=times {
        println!("{}. 你好, {}!", i, name);
    }
    Ok(())
}
