use sorceress::{
    server::{BufferAllocateRead, Server, SynthDefRecv, SynthNew},
    synthdef::{encoder::encode_synth_defs, SynthDef},
    ugen::{Out, Pan2, PlayBuf},
};
use std::fs;
use std::path::PathBuf;
use std::{thread::sleep, time::Duration};

fn main() {
    let server = Server::connect("127.0.0.1:57110").expect("Failed to connect to SC server");
    let sample_path = PathBuf::from("assets/sample.wav");
    let sample_path = fs::canonicalize(&sample_path).unwrap();
    let sample_path = sample_path.as_os_str().to_str().unwrap();
    let buffer = BufferAllocateRead::new(-1, sample_path);
    server
        .send_sync(buffer)
        .expect("Failed to send buffer to SC server");
    let player = SynthDef::new("player", |_| {
        Out::ar().channels(Pan2::ar().input(PlayBuf::ar(1)))
    });
    let encoded_synthdef = encode_synth_defs(vec![player]);
    server
        .send_sync(SynthDefRecv::new(&encoded_synthdef))
        .expect("Failed to send synthdef to SC server");

    server
        .send(SynthNew::new("player", 0))
        .expect("Failed to play synth in SC server");
    sleep(Duration::from_secs(5));

    server.reset().expect("Failed to reset server");
}
