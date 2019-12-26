# eSCL-scan

---

[![Copr build status](https://copr.fedorainfracloud.org/coprs/elxreno/escl-scan/package/escl-scan/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/elxreno/escl-scan)
[![Travis-CI build status](https://travis-ci.com/ElXreno/escl-scan.svg?branch=master)](https://travis-ci.com/ElXreno/escl-scan)
[![dependency status](https://deps.rs/repo/github/elxreno/escl-scan/status.svg)](https://deps.rs/repo/github/elxreno/escl-scan)

```
escl-scan 0.1.0
ElXreno <elxreno@gmail.com>
Utility for scanning with eSCL protocol writen in Rust

USAGE:
    escl-scan [FLAGS] [OPTIONS] <ip> <output file>

FLAGS:
    -f, --force      Force scan and override output file
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dpi <dpi>    Scan resolution [default: 75]

ARGS:
    <ip>             IP of scanner
    <output file>    Destination file
```

# Install
Fedora [Copr](https://copr.fedorainfracloud.org/coprs/elxreno/escl-scan): `sudo dnf copr enable elxreno/escl-scan -y && sudo dnf install escl-scan`