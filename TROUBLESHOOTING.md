# Installation Troubleshooting

## Missing `pkg-config` and `openssl-devel` on Linux

When building the project on Linux, you might encounter errors related to missing `pkg-config` or OpenSSL development files.

To resolve this, install the required packages for your distribution:

### Fedora/RHEL-based systems

```bash
sudo dnf install -y pkgconf-pkg-config openssl-devel
```

### Debian/Ubuntu-based systems

```bash
sudo apt install pkg-config libssl-dev -y
```
