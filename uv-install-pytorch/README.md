## NOT a bug, google home wifi? Huh? 
[uv repo issue here](https://github.com/astral-sh/uv/issues/11156)

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

The verbose run was:
```bash
UV_CONCURRENT_DOWNLOADS=1 uv add nvidia-cudnn-cu12==9.7.066 --verbose
```
```bash
DEBUG uv 0.5.26 (b0e9781da 2025-02-01)
DEBUG Found project root: `/home/oliverkillane/files/bug-reports/uv-install-pytorch`
DEBUG No workspace root found, using project root
DEBUG Reading Python requests from version file at `/home/oliverkillane/files/bug-reports/uv-install-pytorch/.python-version`
DEBUG Using Python request `3.12` from version file at `.python-version`
DEBUG Checking for Python environment at `.venv`
DEBUG The virtual environment's Python version satisfies `3.12`
DEBUG Using request timeout of 30s
DEBUG Found static `pyproject.toml` for: uv-install-pytorch @ file:///home/oliverkillane/files/bug-reports/uv-install-pytorch
DEBUG No workspace root found, using project root
DEBUG Ignoring existing lockfile due to mismatched requirements for: `uv-install-pytorch==0.1.0`
  Requested: {Requirement { name: PackageName("nvidia-cublas-cu12"), extras: [], groups: [], marker: true, source: Registry { specifier: VersionSpecifiers([VersionSpecifier { operator: Equal, version: "12.4.5.8" }]), index: None, conflict: None }, origin: None }, Requirement { name: PackageName("nvidia-cudnn-cu12"), extras: [], groups: [], marker: true, source: Registry { specifier: VersionSpecifiers([VersionSpecifier { operator: Equal, version: "9.7.0.66" }]), index: None, conflict: None }, origin: None }, Requirement { name: PackageName("nvidia-cusolver-cu12"), extras: [], groups: [], marker: true, source: Registry { specifier: VersionSpecifiers([VersionSpecifier { operator: Equal, version: "11.6.1.9" }]), index: None, conflict: None }, origin: None }}
  Existing: {Requirement { name: PackageName("nvidia-cublas-cu12"), extras: [], groups: [], marker: true, source: Registry { specifier: VersionSpecifiers([VersionSpecifier { operator: Equal, version: "12.4.5.8" }]), index: None, conflict: None }, origin: None }, Requirement { name: PackageName("nvidia-cusolver-cu12"), extras: [], groups: [], marker: true, source: Registry { specifier: VersionSpecifiers([VersionSpecifier { operator: Equal, version: "11.6.1.9" }]), index: None, conflict: None }, origin: None }}
DEBUG Solving with installed Python version: 3.12.8
DEBUG Solving with target Python version: >=3.12
DEBUG Adding direct dependency: uv-install-pytorch*
DEBUG Searching for a compatible version of uv-install-pytorch @ file:///home/oliverkillane/files/bug-reports/uv-install-pytorch (*)
DEBUG Adding direct dependency: nvidia-cublas-cu12>=12.4.5.8, <12.4.5.8+
DEBUG Adding direct dependency: nvidia-cudnn-cu12>=9.7.0.66, <9.7.0.66+
DEBUG Adding direct dependency: nvidia-cusolver-cu12>=11.6.1.9, <11.6.1.9+
DEBUG Found stale response for: https://pypi.org/simple/nvidia-cublas-cu12/
DEBUG Sending revalidation request for: https://pypi.org/simple/nvidia-cublas-cu12/
DEBUG Found not-modified response for: https://pypi.org/simple/nvidia-cublas-cu12/
DEBUG Searching for a compatible version of nvidia-cublas-cu12 (>=12.4.5.8, <12.4.5.8+)
DEBUG Selecting: nvidia-cublas-cu12==12.4.5.8 [preference] (nvidia_cublas_cu12-12.4.5.8-py3-none-manylinux2014_aarch64.whl)
DEBUG Found stale response for: https://pypi.org/simple/nvidia-cudnn-cu12/
DEBUG Sending revalidation request for: https://pypi.org/simple/nvidia-cudnn-cu12/
DEBUG Found not-modified response for: https://pypi.org/simple/nvidia-cudnn-cu12/
DEBUG Found stale response for: https://pypi.org/simple/nvidia-cusolver-cu12/
DEBUG Sending revalidation request for: https://pypi.org/simple/nvidia-cusolver-cu12/
DEBUG Found not-modified response for: https://pypi.org/simple/nvidia-cusolver-cu12/
DEBUG Found fresh response for: https://files.pythonhosted.org/packages/7f/7f/7fbae15a3982dc9595e49ce0f19332423b260045d0a6afe93cdbe2f1f624/nvidia_cublas_cu12-12.4.5.8-py3-none-manylinux2014_aarch64.whl.metadata
DEBUG Searching for a compatible version of nvidia-cudnn-cu12 (>=9.7.0.66, <9.7.0.66+)
DEBUG Selecting: nvidia-cudnn-cu12==9.7.0.66 [compatible] (nvidia_cudnn_cu12-9.7.0.66-py3-none-manylinux_2_27_aarch64.whl)
DEBUG Found fresh response for: https://files.pythonhosted.org/packages/fe/8a/0efe67ee6d4d6e05d93e6deb9ae9520884e413f83dd2e012d62ec7e9589d/nvidia_cudnn_cu12-9.7.0.66-py3-none-manylinux_2_27_aarch64.whl.metadata
DEBUG Adding transitive dependency for nvidia-cudnn-cu12==9.7.0.66: nvidia-cublas-cu12*
DEBUG Searching for a compatible version of nvidia-cusolver-cu12 (>=11.6.1.9, <11.6.1.9+)
DEBUG Selecting: nvidia-cusolver-cu12==11.6.1.9 [preference] (nvidia_cusolver_cu12-11.6.1.9-py3-none-manylinux2014_aarch64.whl)
DEBUG Found fresh response for: https://files.pythonhosted.org/packages/46/6b/a5c33cf16af09166845345275c34ad2190944bcc6026797a39f8e0a282e0/nvidia_cusolver_cu12-11.6.1.9-py3-none-manylinux2014_aarch64.whl.metadata
DEBUG Adding transitive dependency for nvidia-cusolver-cu12==11.6.1.9: nvidia-cublas-cu12*
DEBUG Adding transitive dependency for nvidia-cusolver-cu12==11.6.1.9: nvidia-cusparse-cu12*
DEBUG Adding transitive dependency for nvidia-cusolver-cu12==11.6.1.9: nvidia-nvjitlink-cu12*
DEBUG Found stale response for: https://pypi.org/simple/nvidia-cusparse-cu12/
DEBUG Sending revalidation request for: https://pypi.org/simple/nvidia-cusparse-cu12/
DEBUG Found not-modified response for: https://pypi.org/simple/nvidia-cusparse-cu12/
DEBUG Searching for a compatible version of nvidia-cusparse-cu12 (*)
DEBUG Selecting: nvidia-cusparse-cu12==12.5.7.53 [preference] (nvidia_cusparse_cu12-12.5.7.53-py3-none-manylinux2014_aarch64.manylinux_2_17_aarch64.whl)
DEBUG Found stale response for: https://pypi.org/simple/nvidia-nvjitlink-cu12/
DEBUG Sending revalidation request for: https://pypi.org/simple/nvidia-nvjitlink-cu12/
DEBUG Found not-modified response for: https://pypi.org/simple/nvidia-nvjitlink-cu12/
DEBUG Found fresh response for: https://files.pythonhosted.org/packages/2e/a2/313db0453087f5324a5900380ca2e57e050c8de76f407b5e11383dc762ae/nvidia_cusparse_cu12-12.5.7.53-py3-none-manylinux2014_aarch64.manylinux_2_17_aarch64.whl.metadata
DEBUG Adding transitive dependency for nvidia-cusparse-cu12==12.5.7.53: nvidia-nvjitlink-cu12*
DEBUG Searching for a compatible version of nvidia-nvjitlink-cu12 (*)
DEBUG Selecting: nvidia-nvjitlink-cu12==12.8.61 [preference] (nvidia_nvjitlink_cu12-12.8.61-py3-none-manylinux2010_x86_64.manylinux_2_12_x86_64.whl)
DEBUG Found fresh response for: https://files.pythonhosted.org/packages/03/f8/9d85593582bd99b8d7c65634d2304780aefade049b2b94d96e44084be90b/nvidia_nvjitlink_cu12-12.8.61-py3-none-manylinux2010_x86_64.manylinux_2_12_x86_64.whl.metadata
DEBUG Tried 6 versions: nvidia-cublas-cu12 1, nvidia-cudnn-cu12 1, nvidia-cusolver-cu12 1, nvidia-cusparse-cu12 1, nvidia-nvjitlink-cu12 1, uv-install-pytorch 1
DEBUG all marker environments resolution took 0.147s
Resolved 6 packages in 159ms
DEBUG Using request timeout of 30s
DEBUG Requirement already installed: nvidia-cublas-cu12==12.4.5.8
DEBUG Identified uncached distribution: nvidia-cudnn-cu12==9.7.0.66
DEBUG Requirement already installed: nvidia-cusolver-cu12==11.6.1.9
DEBUG Requirement already installed: nvidia-cusparse-cu12==12.5.7.53
DEBUG Requirement already installed: nvidia-nvjitlink-cu12==12.8.61
DEBUG No cache entry for: https://files.pythonhosted.org/packages/4b/6d/afece827c6cb52d8a193e558a69a47012d27c34548450476594a876f9ba7/nvidia_cudnn_cu12-9.7.0.66-py3-none-manylinux_2_27_x86_64.whl
WARN Streaming failed for nvidia-cudnn-cu12==9.7.0.66; downloading wheel to disk (error decoding response body)
DEBUG No cache entry for: https://files.pythonhosted.org/packages/4b/6d/afece827c6cb52d8a193e558a69a47012d27c34548450476594a876f9ba7/nvidia_cudnn_cu12-9.7.0.66-py3-none-manylinux_2_27_x86_64.whl
DEBUG Reverting changes to `pyproject.toml`
DEBUG Reverting changes to `uv.lock`
  × Failed to download `nvidia-cudnn-cu12==9.7.0.66`
  ├─▶ Failed to write to the distribution cache
  ╰─▶ Failed to download distribution due to network timeout. Try increasing UV_HTTP_TIMEOUT (current value: 30s).
  help: `nvidia-cudnn-cu12` (v9.7.0.66) was included because `uv-install-pytorch` (v0.1.0) depends on `nvidia-cudnn-cu12
```

