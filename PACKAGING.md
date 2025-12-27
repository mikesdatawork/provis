# Packaging Guide

## Debian/Ubuntu (.deb)

### Build Package
```bash
./build-deb.sh
```

This creates: `target/debian/provis_3.6.0_amd64.deb`

### Install Locally
```bash
sudo dpkg -i target/debian/provis_*.deb
```

### Create APT Repository
```bash
cd target/debian
dpkg-scanpackages . /dev/null | gzip -9c > Packages.gz
# Upload to your web server
```

### User Installation
```bash
# Add repository
echo "deb [trusted=yes] https://your-server.com/apt ./" | sudo tee /etc/apt/sources.list.d/provis.list
sudo apt update
sudo apt install provis
```

## Snap

### Build Package
```bash
./build-snap.sh
```

### Test Locally
```bash
sudo snap install --dangerous provis_*.snap
```

### Publish to Snap Store
```bash
snapcraft login
snapcraft upload provis_*.snap --release=stable
```

### User Installation
```bash
sudo snap install provis
```

## Package Metadata

Update before building:
- `Cargo.toml`: Update repository URLs and maintainer info
- `snapcraft.yaml`: Verify confinement and plugs
- `build-deb.sh`: Check dependencies

## Testing

After building, test:
```bash
# .deb
sudo dpkg -i target/debian/provis_*.deb
provis --version
provis --commands
sudo dpkg -r provis

# Snap
sudo snap install --dangerous provis_*.snap
provis --version
provis --commands
sudo snap remove provis
```
