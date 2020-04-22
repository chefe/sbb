# sbb
A simple rust app to get timetable information for swiss public transport.

## Installation via Make
1. Verify `make` and `cargo` are installed
2. Run the following commands to install the app

```bash
git clone git@github.com:chefe/sbb.git
cd sbb
sudo make install
```

## Installation via Flatpak
1. Verify `make`, `flatpak-builder` and `flatpak` are installed
2. Run the following commands to build the flatpak and install it

```bash
git clone git@github.com:chefe/sbb.git
cd sbb
make install-flatpak
```

## Credits
* [Swiss public transport API](https://transport.opendata.ch)
* [gtk-rs](https://gtk-rs.org)
