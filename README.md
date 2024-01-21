# steam-idler

Trying to be a dead-simple tool to idle Steam games, that doesn't require a GUI to use

Currently works fine on systems with Steam installed & running, working on adding Dockerisation to be able to use on systems without window managers or Steam

## Usage

```shell
steam-idler <APP ID> <DURATION>
```

Where app ID is the Steam app ID, e.g. 480 for Spacewar.

Duration is written as something like 5d4h3m2s. The following units are supported (and can be given in any order):
- **d**ays
- **h**ours
- **m**inutes
- **s**econds
