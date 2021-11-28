#!/bin/sh

NO_COLOR=$'\e[0m'
ERROR=$'\e[0;31m'"ERROR: "
INFO=$'\e[0;32m'"INFO: "
WARN=$'\e[0;33m'"WARN: "
LATEST="https://github.com/francis-du/iotdb-cli/releases/latest"
LATEST_RELEASE="https://api.github.com/repos/francis-du/iotdb-cli/releases/latest"
BIN_PATH="/usr/local/bin"
VERSION=$1

set_version() {
  if [ ! -n "${VERSION}" ]; then
    status=$(curl -Is ${LATEST_RELEASE} | grep 'HTTP' | awk '{print $2}')
    if [ $status != 200 ]; then
      echo "${ERROR}Install failed, reason: $(curl -s ${LATEST_RELEASE} | grep '"message":' | awk '{print $0}')"
      exit 1
    fi

    if command -v curl &>/dev/null; then
      VERSION=$(curl -s ${LATEST_RELEASE} | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
      if command -v wget &>/dev/null; then
        VERSION=$(wget ${LATEST_RELEASE} | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
      else
        echo "${ERROR}Please install wget or curl${NO_COLOR}"
        exit 1
      fi
    fi
  fi
}

download() {
  local url=$1
  local file_name=$2

  if command -v curl &>/dev/null; then
    if [ -n $file_name ]; then
      if [ ! -f $file_name ]; then
        curl -L -f -# -C0 -o $file_name "$url"
      else
        command $file_name -h
        echo "${WARN}'$file_name' exists"
        echo "${WARN}Please run command 'iotdb' or '${file_name}'"
        exit 1
      fi
    else
      curl -L -f -# -OC0 "$url" --progress-bar
    fi
  else
    if command -v wget &>/dev/null; then
      command wget --spider -q "$url"
      if [ $? == 0 ]; then
        wget --spider -q "$url"
        if [ -n $file_name ]; then
          if [ ! -f $file_name ]; then
            wget -q -c --show-progress "$url" -O $file_name
          else
            command $file_name -h
            echo "${WARN}'$file_name' exists"
            echo "${WARN}Please run command 'iotdb' or '${file_name}'"
            exit 1
          fi
        else
          wget -q -c --show-progress "$url"
        fi
      else
        echo "${ERROR}Download failed from $url"
        exit 1
      fi
    else
      echo "${ERROR}Please install 'wget' or 'curl'"
      exit 1
    fi
  fi
}

function install() {
  set_version
  echo "${INFO}Version: ${VERSION}"
  echo "${INFO}Latest release '${LATEST}'"
  asset_base_url="https://github.com/francis-du/iotdb-cli/releases/download/${VERSION}"
  case "$(uname -s)" in
  Linux*)
    bin_name="${BIN_PATH}/iotdb"
    asset_url="$asset_base_url/iotdb-linux"
    echo "${INFO}Download from '${asset_url}'"
    download $asset_url $bin_name

    if [ -f "${bin_name}" ]; then
      chmod +x $bin_name
      echo "${INFO}Please run command 'iotdb' or '${bin_name}'"
    else
      echo "${ERROR}Install failed"
    fi
    ;;
  Darwin*)
    bin_name="${BIN_PATH}/iotdb"
    asset_url="${asset_base_url}/iotdb-mac"
    echo "${INFO}Download from '${asset_url}'"
    download $asset_url $bin_name

    if [ -f "${bin_name}" ]; then
      chmod +x $bin_name
      echo "${INFO}Please run command 'iotdb' or '${bin_name}'"
    else
      echo "${ERROR}Install failed"
    fi
    ;;
  CYGWIN*)
    # TODO: Need to be test
    bin_name="${BIN_PATH}/iotdb.exe"
    asset_url="$asset_base_url/iotdb.exe"
    echo "${INFO}Download from '${asset_url}'"
    download $asset_url $bin_name

    if [ -f "${bin_name}" ]; then
      echo "${INFO}Please run command 'iotdb.exe' or '${bin_name}'"
    else
      echo "${ERROR}Install failed"
    fi
    ;;
  MINGW*)
    # TODO: Need to be test
    bin_name="${BIN_PATH}/iotdb.exe"
    asset_url=$asset_base_url"/iotdb.exe"
    echo "${INFO}Download from '${asset_url}'"
    download $asset_url $bin_name

    if [ -f "${bin_name}" ]; then
      echo "${INFO}Please run command 'iotdb.exe' or '${bin_name}'"
    else
      echo "${ERROR}Install failed"
    fi
    ;;
  *)
    echo "${ERROR}Unknown OS"
    ;;
  esac
}

install
