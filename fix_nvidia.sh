#!/usr/bin/env bash
set -euo pipefail

LOG_FILE="drive_update.log"
exec > >(tee -a "$LOG_FILE") 2>&1

printf '\n[%s] Starting NVIDIA driver repair\n' "$(date)"

if [[ $EUID -ne 0 ]]; then
    echo "This script must be run with sudo (root privileges)." >&2
    exit 1
fi

mapfile -t LEGACY_PKGS < <(dpkg-query -W -f='${Package} ${Version}\n' \
    | awk '$2 ~ /^575/ && ($1 ~ /^(nvidia|libnvidia|libcuda|libnv|firmware-nvidia)/) {print $1}' \
    | sort -u)

EXTRA_PURGE=(
    libegl-nvidia0
    libegl-nvidia0:i386
    libgles-nvidia1
    libgles-nvidia1:i386
    libgles-nvidia2
    libgles-nvidia2:i386
    libglx-nvidia0
    libglx-nvidia0:i386
    libcuinj64-12.0
    libnvidia-ml-dev
    nsight-systems-target
    nvidia-cuda-dev
    xserver-xorg-video-nvidia
    nsight-systems
    nvidia-cuda-toolkit
    nvidia-profiler
    nvidia-visual-profiler
    nvidia-vulkan-icd:i386
    libnvidia-decode-580:i386
    nvidia-compute-utils-580
    libnvidia-encode-580:i386
    nvidia-driver-580
    nvidia-utils-580
    xserver-xorg-video-nvidia-580
    libnvidia-gl-580
    libnvidia-compute-580
    libnvidia-extra-580
    libnvidia-decode-580
    libnvidia-encode-580
    libnvidia-cfg1-580
    nvidia-opencl-dev
    nvidia-cuda-toolkit-doc
    nvidia-opencl-dev
    nvidia-cuda-toolkit-doc
)

declare -A TO_PURGE=()
for pkg in "${LEGACY_PKGS[@]}"; do
    TO_PURGE["$pkg"]=1
done
for pkg in "${EXTRA_PURGE[@]}"; do
    if dpkg -s "$pkg" >/dev/null 2>&1; then
        TO_PURGE["$pkg"]=1
    fi
done
LEGACY_PKGS=("${!TO_PURGE[@]}")
IFS=$'\n' LEGACY_PKGS=($(sort <<<"${LEGACY_PKGS[*]}"))
unset IFS

if [[ ${#LEGACY_PKGS[@]} -eq 0 ]]; then
    echo "No legacy 575.x NVIDIA packages detected."
else
    printf '[%s] Purging legacy packages:\n' "$(date)"
    printf '  %s\n' "${LEGACY_PKGS[@]}"
    apt remove --yes --purge "${LEGACY_PKGS[@]}"
fi

printf '[%s] Running apt --fix-broken install\n' "$(date)"
apt --yes --fix-broken install

printf '[%s] Installing nvidia-driver-580 and nvidia-utils-580\n' "$(date)"
apt install --yes nvidia-driver-580 nvidia-utils-580

printf '[%s] Installation complete. Consider rebooting, then run nvidia-smi to verify.\n' "$(date)"
