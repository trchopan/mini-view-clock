SERVER=""
DIST_PATH=""
BUILD_PATH="build/mini-view-clock.tar.gz"

while getopts "s:d:" opt; do
  case $opt in
    s) SERVER="$OPTARG";;
    d) DIST_PATH="$OPTARG";;
    \?) echo "Usage: $0 -s <server> -d <dist_path>" >&2; exit 1;;
  esac
done

if [ -z "$SERVER" ] || [ -z "$DIST_PATH" ]; then
  echo "Error: Both -s (server) and -d (dist_path) are required." >&2
  echo "Usage: $0 -s <server> -d <dist_path>" >&2
  exit 1
fi

echo "Deploying to $SERVER:$DIST_PATH..."

mkdir -p build
git archive --format=tar HEAD | gzip > $BUILD_PATH
ssh $SERVER "rm -r $DIST_PATH/*; mkdir -p $DIST_PATH;"
scp $BUILD_PATH "$SERVER:$DIST_PATH"
ssh $SERVER "cd $DIST_PATH; tar xvf mini-view-clock.tar.gz; cd server; docker compose up -d;"
