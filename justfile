# Copyright 2018-2022 Cargill Incorporated
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


crates := '\
    libcylinder \
    cli
    '

features := '\
    --features=experimental \
    --features=stable \
    --features=default \
    --no-default-features \
    '

build:
    #!/usr/bin/env sh
    set -e
    for feature in $(echo {{features}})
    do
        for crate in $(echo {{crates}})
        do
            cmd="cargo build --tests --manifest-path=$crate/Cargo.toml $feature"
            echo "\033[1m$cmd\033[0m"
            $cmd
        done
    done
    echo "\n\033[92mBuild Success\033[0m\n"

clean:
    #!/usr/bin/env sh
    set -e
    for crate in $(echo {{crates}})
    do
        cmd="cargo clean --manifest-path=$crate/Cargo.toml"
        echo "\033[1m$cmd\033[0m"
        $cmd
        cmd="rm -f $crate/Cargo.lock"
        echo "\033[1m$cmd\033[0m"
        $cmd
    done

lint: version-check
    #!/usr/bin/env sh
    set -e
    for feature in $(echo {{features}})
    do
        for crate in $(echo {{crates}})
        do
            echo "\033[1mcargo fmt -- --check\033[0m"
            cargo fmt --manifest-path=$crate/Cargo.toml -- --check
            cmd="cargo clippy --manifest-path=$crate/Cargo.toml $feature -- -D warnings"
            echo "\033[1m$cmd\033[0m"
            $cmd
        done
    done
    echo "\n\033[92mLint Success\033[0m\n"

test:
    #!/usr/bin/env sh
    set -e
    for feature in $(echo {{features}})
    do
        for crate in $(echo {{crates}})
        do
            cmd="cargo build --tests --manifest-path=$crate/Cargo.toml $feature"
            echo "\033[1m$cmd\033[0m"
            $cmd
            cmd="cargo test --manifest-path=$crate/Cargo.toml $feature"
            echo "\033[1m$cmd\033[0m"
            $cmd
        done
    done
    echo "\n\033[92mTest Success\033[0m\n"

version-check:
    #!/usr/bin/env sh

    set -e

    version=$(cat VERSION)

    cylinder_version=$(cargo metadata --format-version 1 --no-deps \
        | jq '.packages[] | select(.name == "cylinder") | .version' \
        | sed -e 's/"//g')
    cyl_version=$(cargo metadata --format-version 1 --no-deps \
        | jq '.packages[] | select(.name == "cyl") | .version' \
        | sed -e 's/"//g')

    cyl_dep_version=$(cat cli/Cargo.toml \
        | grep "# cylinder Version" | sed -e 's/^.*"=//' -e 's/".*//')

    if [ "$version" != "$cylinder_version" ]; then
        echo "expected $version but found $cylinder_version in libcylinder/Cargo.toml"
        exit 1
    fi

    if [ "$version" != "$cyl_version" ]; then
        echo "expected $version but found $cyl_version in cli/Cargo.toml"
        exit 1
    fi

    if [ "$version" != "$cyl_dep_version" ]; then
        echo "expected $version but found $cyl_dep_version in cli/Cargo.toml for the cylinder dependency"
        exit 1
    fi

    echo "Version OK: $version"
