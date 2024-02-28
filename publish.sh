echo Stashing all uncommited data
git add --all
git stash

export VERSION=`cat ./Cargo.toml | grep '^version' | head -n1 | cut -d'"' -f2 | tr -d '\n'`
echo Publish catgirl-enging v$VERSION

echo Replacing version number in catgirl-engine-client with catgirl-engine version
sed -i 's/^version = "[0-9.]*"/version = "`$VERSION`"/' ./client/Cargo.toml

echo Replacing version number in catgirl-engine-server with catgirl-engine version
sed -i 's/^version = "[0-9.]*"/version = "`$VERSION`"/' ./server/Cargo.toml

# echo Publishing catgirl-engine-client
# cargo publish -p catgirl-engine-client --allow-dirty

# echo Publishing catgirl-engine-server
# cargo publish -p catgirl-engine-server --allow-dirty

# echo Publishing catgirl-engine
# cargo publish -p catgirl-engine --allow-dirty

echo Resetting client version
git checkout HEAD -- ./client/Cargo.toml

echo Resetting server version
git checkout HEAD -- ./server/Cargo.toml

git stash pop
