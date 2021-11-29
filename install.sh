#!/bin/bash
# shellcheck disable=SC3003

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

    if
      command -v curl >/dev/null
    then
      VERSION=$(curl -s ${LATEST_RELEASE} | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
      if
        command -v wget >/dev/null
      then
        VERSION=$(wget ${LATEST_RELEASE} | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
      else
        echo "${ERROR}Please install wget or curl${NO_COLOR}"
        exit 1
      fi
    fi
  fi
}

download() {
  url=$1
  file_name=$2

  if
    command -v curl >/dev/null
  then
    curl -L -f -# -C0 -o "$file_name" "$url"
  else
    if
      command -v wget >/dev/null
    then
      wget --spider -q "$url"
      if [ $? = 0 ]; then
        wget -q -c --show-progress "$url" -O "$file_name"
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

bin="iotdb"
win_bin="iotdb.exe"
install() {
  set_version
  echo "${INFO}Version: ${VERSION}"
  echo "${INFO}Latest release '${LATEST}'"
  asset_base_url="https://github.com/francis-du/iotdb-cli/releases/download/${VERSION}"

  case "$(uname -s)" in
  Linux*)
    asset_url="$asset_base_url/iotdb-linux"
    echo "${INFO}Download from '${asset_url}'"

    if [ -w ${BIN_PATH} ] && [ -x ${BIN_PATH} ]; then
      bin="${BIN_PATH}/$bin"
      if [ -f $bin ]; then
        command "$bin" -h
        echo "${WARN}'$bin' exists"
        echo "${WARN}Please run '$bin -h' to get help information"
      else
        download "$asset_url" "$bin"
        if [ -f "$bin" ]; then
          chmod +x $bin
          echo "${WARN}Please run '$bin -h' to get help information"
        else
          echo "${ERROR}Install failed"
        fi
      fi
    else
      if [ -f $bin ]; then
        command "./$bin" -h
        echo "${WARN}'$bin' exists"
        echo "${WARN}Please run './$bin -h' to get help information"
      else
        download "$asset_url" "$bin"
        if [ -f "$bin" ]; then
          chmod +x $bin
          echo "${WARN}Please run './$bin -h' to get help information"
        else
          echo "${ERROR}Install failed"
        fi
      fi
    fi
    ;;
  Darwin*)
    asset_url="${asset_base_url}/iotdb-mac"
    echo "${INFO}Download from '${asset_url}'"

    if [ -w ${BIN_PATH} ] && [ -x ${BIN_PATH} ]; then
      bin="${BIN_PATH}/$bin"
      if [ -f $bin ]; then
        command "$bin" -h
        echo "${WARN}'$bin' exists"
        echo "${WARN}Please run '$bin -h' to get help information"
      else
        download "$asset_url" "$bin"
        if [ -f "$bin" ]; then
          chmod +x $bin
          echo "${WARN}Please run '$bin -h' to get help information"
        else
          echo "${ERROR}Install failed"
        fi
      fi
    else
      if [ -f $bin ]; then
        command "./$bin" -h
        echo "${WARN}'$bin' exists"
        echo "${WARN}Please run './$bin -h' to get help information"
      else
        download "$asset_url" "$bin"
        if [ -f "$bin" ]; then
          chmod +x $bin
          echo "${WARN}Please run './$bin -h' to get help information"
        else
          echo "${ERROR}Install failed"
        fi
      fi
    fi
    ;;
  CYGWIN*)
    # TODO: Need to be test
    asset_url="$asset_base_url/iotdb.exe"
    echo "${INFO}Download from '${asset_url}'"

    if [ -w ${BIN_PATH} ] && [ -x ${BIN_PATH} ]; then
      win_bin="${BIN_PATH}/$win_bin"
      if [ -f $win_bin ]; then
        command "$win_bin" -h
        echo "${WARN}'$win_bin' exists"
        echo "${WARN}Please run '$win_bin -h' to get help information"
      else
        download "$asset_url" "$win_bin"
        if [ -f "$win_bin" ]; then
          chmod +x $win_bin
          echo "${WARN}Please run '$win_bin -h' to get help information"
        else
          echo "${ERROR}Install failed"
        fi
      fi
    else
      if [ -f $win_bin ]; then
        command "./$win_bin" -h
        echo "${WARN}'$win_bin' exists"
        echo "${WARN}Please run './$win_bin -h' to get help information"
      else
        download "$asset_url" "$win_bin"
        if [ -f "$win_bin" ]; then
          chmod +x $win_bin
          echo "${WARN}Please run './$win_bin -h' to get help information"
        else
          echo "${ERROR}Install failed"
        fi
      fi
    fi
    ;;
  MINGW*)
    # TODO: Need to be test
    asset_url="$asset_base_url/iotdb.exe"
    echo "${INFO}Download from '${asset_url}'"

    if [ -w ${BIN_PATH} ] && [ -x ${BIN_PATH} ]; then
      win_bin="${BIN_PATH}/$win_bin"
      if [ -f $win_bin ]; then
        command "$win_bin" -h
        echo "${WARN}'$win_bin' exists"
        echo "${WARN}Please run '$win_bin -h' to get help information"
      else
        download "$asset_url" "$win_bin"
        if [ -f "$win_bin" ]; then
          chmod +x $win_bin
          echo "${WARN}Please run '$win_bin -h' to get help information"
        else
          echo "${ERROR}Install failed"
        fi
      fi
    else
      if [ -f $win_bin ]; then
        command "./$win_bin" -h
        echo "${WARN}'$win_bin' exists"
        echo "${WARN}Please run './$win_bin -h' to get help information"
      else
        download "$asset_url" "$win_bin"
        if [ -f "$win_bin" ]; then
          chmod +x $win_bin
          echo "${WARN}Please run './$win_bin -h' to get help information"
        else
          echo "${ERROR}Install failed"
        fi
      fi
    fi
    ;;
  *)
    echo "${ERROR}Unknown OS"
    ;;
  esac
}

install
