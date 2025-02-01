## Failing to install pytorch
Either hands, and fails on timeout (all progress bars freeze), or with connection reset error.

From a fresh install of uv:
```bash
cargo install --git https://github.com/astral-sh/uv uv
```
```bash
uv --version
uv 0.5.26 (b0e9781da 2025-02-01)
```

### Timeout Failure
```bash
uv init
uv add torch
```
Freezes as:
```bash
Using CPython 3.12.8
Creating virtual environment at: .venv
Resolved 25 packages in 47ms
⠙ Preparing packages... (0/9)
nvidia-cusolver-cu12 ------------------------------ 193.91 KiB/122.01 MiB
nvidia-cusolver-cu12 ------------------------------ 19.62 MiB/122.01 MiB
nvidia-cusparselt-cu12 ------------------------------ 206.83 KiB/143.11 MiB
nvidia-cusparselt-cu12 ------------------------------ 19.56 MiB/143.11 MiB
nvidia-nccl-cu12 ------------------------------ 174.83 KiB/179.91 MiB
nvidia-nccl-cu12 ------------------------------ 19.37 MiB/179.91 MiB
nvidia-cusparse-cu12 ------------------------------ 222.71 KiB/197.84 MiB
nvidia-cusparse-cu12 ------------------------------ 19.03 MiB/197.84 MiB
nvidia-cufft-cu12 ------------------------------ 287.33 KiB/201.66 MiB
nvidia-cufft-cu12 ------------------------------ 19.12 MiB/201.66 MiB
triton     ------------------------------ 192.00 KiB/241.43 MiB
triton     ------------------------------ 19.71 MiB/241.43 MiB
nvidia-cublas-cu12 ------------------------------ 223.52 KiB/346.60 MiB
nvidia-cublas-cu12 ------------------------------ 19.49 MiB/346.60 MiB
nvidia-cudnn-cu12 ------------------------------ 222.13 KiB/633.96 MiB
nvidia-cudnn-cu12 ------------------------------ 19.17 MiB/633.96 MiB
torch      ------------------------------ 191.78 KiB/731.11 MiB
torch      ------------------------------ 20.08 MiB/731.11 MiB
```
Then after timeout fails with:
```bash
Using CPython 3.12.8
Creating virtual environment at: .venv
Resolved 25 packages in 47ms
  × Failed to download `nvidia-cusparselt-cu12==0.6.2`
  ├─▶ Failed to write to the distribution cache
  ╰─▶ Failed to download distribution due to network timeout. Try increasing UV_HTTP_TIMEOUT (current value: 30s).
  help: `nvidia-cusparselt-cu12` (v0.6.2) was included because `uv-install-pytorch` (v0.1.0) depends on `torch` (v2.6.0) which depends on `nvidia-cusparselt-cu12`
```
verbose logging is in [verbose.log](./verbose.log).

### Downloading packages separately
Seems to work for some of the packages that failed
```bash
uv add nvidia-cusolver-cu12==11.6.1.9
```
```bash
Resolved 5 packages in 11ms
Prepared 1 package in 4.62s
Installed 3 packages in 25ms
 + nvidia-cusolver-cu12==11.6.1.9
 + nvidia-cusparse-cu12==12.5.7.53
 + nvidia-nvjitlink-cu12==12.8.61
```
### Failure for nvidia-cudnn-cu12=9.7.0.66
```bash
UV_CONCURRENT_DOWNLOADS=1 uv add nvidia-cudnn-cu12==9.7.0.66
```
```bash
Resolved 6 packages in 7ms
⠦ Preparing packages... (0/1)
nvidia-cudnn-cu12 ------------------------------ 114.61 MiB/693.05 MiB
nvidia-cudnn-cu12 ------------------------------ 190.28 MiB/693.05 MiB  
```
Looks like it starts downloading itself again, then freezes with the above?!

