rm -Rf dist
mkdir dist

./node_modules/.bin/esbuild --define:process.env.NODE_ENV='"production"' src/client.tsx --bundle --outfile=dist/client.js
        #./node_modules/.bin/esbuild --bundle --platform=node --target=node10.4 src/client.tsx --outfile=dist/client.js

./node_modules/.bin/esbuild src/server.tsx --bundle --platform=node --target=node10.4  --outfile=dist/server.js