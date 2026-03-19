release:
	#!/bin/sh
	RUSTFLAGS="-D warnings" cargo build --release
	VERSION=$(git cliff --bumped-version | cut -d'v' -f2)
	cargo release -x $VERSION
	git cliff -o CHANGELOG.md --tag $VERSION
	git add CHANGELOG.md
	git commit --amend --no-edit
	git tag v$VERSION -f

push:
	git push
	git push --tags
