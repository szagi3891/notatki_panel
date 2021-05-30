rm -Rf ./build
wasm-pack build panel/client --no-typescript --target web --out-dir ../../build --out-name app --dev

rm ./build/.gitignore
rm ./build/package.json
