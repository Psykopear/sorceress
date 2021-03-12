use sorceress::{
    server::{Server, SynthDefRecv, SynthNew},
    synthdef::{encoder::encode_synth_defs, SynthDef},
    ugen::{Out, Pan2, SinOsc},
};
use std::{thread::sleep, time::Duration};

fn main() {
    let server = Server::connect("127.0.0.1:57110").expect("Failed to connect to SC server");

    let sine_wave = SynthDef::new("sine_wave", |_| {
        Out::ar().channels(Pan2::ar().input(SinOsc::ar().freq(220)))
    });
    let encoded_synthdef = encode_synth_defs(vec![sine_wave]);
    server
        .send_sync(SynthDefRecv::new(&encoded_synthdef))
        .expect("Failed to send SynthDef to SC server");

    server
        .send(SynthNew::new("sine_wave", 1))
        .expect("Failed to play synth in SC server");
    sleep(Duration::from_secs(1));

    server.reset().expect("Failed to reset the SC server");
}
