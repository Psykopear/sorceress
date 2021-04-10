use sorceress::{
    server::{Server, SynthDefRecv, SynthNew},
    synthdef::{encoder::encode_synth_defs, SynthDef},
    ugen::{Out, Pan2, SinOsc},
};
use std::{thread::sleep, time::Duration};

fn main() {
    // Assuming the SC server is already running,
    // try to connect to the default address
    let server = Server::connect("127.0.0.1:57110").expect("Failed to connect to SC server");

    // Now create a SynthDef called "sine_wave"
    let sine_wave = SynthDef::new("sine_wave", |_| {
        // Use the `Out` ugen
        Out::ar().channels(
            // Pan2 takes the signal and pans it across 2 outputs
            Pan2::ar().input(
                // Here we create a fixed frequency Sin oscillator, at 220Hz
                SinOsc::ar().freq(220),
            ),
        )
    });
    // Now encode the SynthDef so the SC server understands it
    let encoded_synthdef = encode_synth_defs(vec![sine_wave]);
    // And send the encoded SynthDef to the server
    server
        .send_sync(SynthDefRecv::new(&encoded_synthdef))
        .expect("Failed to send SynthDef to SC server");

    // Finally ask the server to play
    server
        .send(SynthNew::new("sine_wave", 1))
        .expect("Failed to play synth in SC server");
    sleep(Duration::from_secs(1));

    server.reset().expect("Failed to reset the SC server");
}
