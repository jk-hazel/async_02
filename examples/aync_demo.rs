use anyhow::Result;
use std::{
    sync::mpsc::{self, Sender},
    thread,
};

const AYNC_NUM: i8 = 3;

#[derive(Debug)]
#[allow(dead_code)]
struct Msg {
    id: i64,
    data: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    //create 4 producers
    for i in 0..AYNC_NUM {
        let tx = tx.clone();
        thread::spawn(move || {
            let _ = produce_msg(i, tx);
        });
    }
    //when all producers are done, drop the tx
    drop(tx);

    //create a consumer
    let rx = rx;
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("msg: {:?}", msg);
        }
    });


    //wait for all threads to finish
    consumer.join().map_err(|e| anyhow::anyhow!("thread wait error: {:?}", e))?;

    Ok(())
}

fn produce_msg(id: i8, tx: Sender<Msg>) -> Result<()> {
    loop {
        let data: usize = rand::random::<usize>();
        let sleep_time = rand::random::<u8>() as u64 * 10;
        tx.send(Msg::new(id as i64, data))?;
        thread::sleep(std::time::Duration::from_millis(sleep_time));
        //if id % 10 == 0 exit
        if data % 4 == 0 {
            println!("exit produce num: {}", id);
            return Ok(());
        }
    }
}

impl Msg {
    fn new(id: i64, data: usize) -> Self {
        Msg { id, data }
    }
}
