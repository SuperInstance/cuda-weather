# cuda-weather

Environmental sensing — temperature, humidity, light, noise, terrain, weather inference (Rust)

Part of the Cocapn spatial layer — how agents perceive and navigate physical space.

## What It Does

### Key Types

- `EnvReading` — core data structure
- `EnvAssessment` — core data structure
- `EnvHistory` — core data structure
- `EnvironmentMonitor` — core data structure
- `AlertThresholds` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-weather.git
cd cuda-weather

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_weather::*;

// See src/lib.rs for full API
// 11 unit tests included
```

### Available Implementations

- `ThermalZone` — see source for methods
- `LightCondition` — see source for methods
- `NoiseLevel` — see source for methods
- `EnvHistory` — see source for methods
- `Default for AlertThresholds` — see source for methods
- `EnvironmentMonitor` — see source for methods

## Testing

```bash
cargo test
```

11 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: spatial
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-sensor-agent](https://github.com/Lucineer/cuda-sensor-agent)
- [cuda-resolve-agent](https://github.com/Lucineer/cuda-resolve-agent)
- [cuda-voxel-logic](https://github.com/Lucineer/cuda-voxel-logic)
- [cuda-world-model](https://github.com/Lucineer/cuda-world-model)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
