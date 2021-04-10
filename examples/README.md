# Examples

All the examples assume that SuperCollider server is running and reachable at `127.0.0.1:57110`

To execute the examples run:
```
cargo run --release --example <examplename>
```
from the root directory of the repository.

### List of examples
- [sinewave](./sinewave.rs) - Plays a sine wave for 1 second.
- [playfile](./playfile.rs) - Loads a `.wav` file into a SC Buffer and plays it.
