_help:
    @just --help

bump version='':
    #!/usr/bin/env bash
    if [ -z {{ version }} ];then
        echo "Please input version. eg: 0.1.0"
        exit
    fi
    echo -n "Updating version..."
    sed -i 's/^version = ".*"/version = "{{ version }}"/' Cargo.toml
    cargo update --workspace > /dev/null 2>&1
    git add . > /dev/null
    git commit -m 'ci(release): Release v{{ version }}' > /dev/null
    git tag v{{ version }} > /dev/null
    echo -e "\r\e[32mUpdated version   \e[0m"
